use rand::RngExt;
use rand::rngs::ThreadRng;

use super::clock::DayClock;
use super::log_line::{Classification, LineFont, LogLine};
use super::pane::Pane;
use super::phase::Phase;
use super::threats::{RuleFlags, deviation_threat_line, repeat_threat_line};
use super::timestamp::{even_minute_of, fill_n, timestamp};
use super::verb::Verb;
use super::zone::Zone;

const NORMAL_BODIES: &[&str] = &[
    "食パンの型を{n}枚 洗った",
    "小麦粉を{n}kg 仕込んだ",
    "配達伝票を{n}件 確認した",
    "レジを開けた",
    "常連客に会釈した",
    "オーブンを{n}度に設定した",
    "冷蔵庫の在庫を数えた",
    "クロワッサンの生地を折った",
    "閉店の掃除を始めた",
    "釣り銭を{n}枚 補充した",
];

const FORBIDDEN_MEMOS: &[&str] = &[
    "奇数分に記帳せぬこと",
    "夜の納品は受けぬこと",
    "三のつく日は窯の火を落とさぬこと",
    "同じ行が二度流れたら、続けて読まぬこと",
    "呼ばれても、名を疑うこと",
    "焼き上がりの字は崩さぬこと",
];

/// 外画面の通常行。「異常なし」を基本単位にするため、
/// プールの大半を占めさせている (別途の重み付けテーブルを持ち込まず、
/// 配列内の出現回数だけで済ませる)。
const OUTSIDE_BODIES: &[&str] = &[
    "異常なし",
    "異常なし",
    "異常なし",
    "異常なし",
    "ゴミが{n}個 転がってきた",
    "野良猫が横切った",
    "配達車が通り過ぎた",
    "落ち葉が吹き溜まっている",
    "特に変わりなし",
];

/// 売り場画面の通常行。地の文ではなく、客への応対そのものを敬体で記録する。
const FLOOR_BODIES: &[&str] = &[
    "いらっしゃいませ",
    "かしこまりました",
    "少々お待ちください",
    "ありがとうございました",
    "{n}円のお返しです",
    "本日は品数が少なめです",
    "袋にお入れします",
];

const CALL_BODIES: &[&str] = &[
    "{loc}で誰かが名前を呼んでいる 応えないこと",
    "誰かがこちらを見ている 気づかないふりをした方がいい",
    "{loc}に何かが立っている",
    "背後に気配がある 振り返らないこと",
    "{loc}から物音がした そちらを見ないで",
];

fn normal_line(clock: DayClock, rng: &mut ThreadRng) -> LogLine {
    let h = clock.hour();
    let m = even_minute_of(clock);
    let n = rng.random_range(1..30);
    let body = NORMAL_BODIES[rng.random_range(0..NORMAL_BODIES.len())];
    let text = format!("{} {}", timestamp(h, m), fill_n(body, n));
    LogLine::new(text, Classification::Normal, None)
}

fn baking_normal_line(clock: DayClock, rng: &mut ThreadRng) -> LogLine {
    let h = clock.hour();
    let m = even_minute_of(clock);
    let n = rng.random_range(1..20);
    let text = format!("{} バゲットが{}本 焼き上がり", timestamp(h, m), n);
    LogLine::new(text, Classification::Normal, None)
}

fn outside_normal_line(clock: DayClock, rng: &mut ThreadRng) -> LogLine {
    let h = clock.hour();
    let m = even_minute_of(clock);
    let n = rng.random_range(1..10);
    let body = OUTSIDE_BODIES[rng.random_range(0..OUTSIDE_BODIES.len())];
    let text = format!("{} {}", timestamp(h, m), fill_n(body, n));
    LogLine::new(text, Classification::Normal, None)
}

fn floor_normal_line(clock: DayClock, rng: &mut ThreadRng) -> LogLine {
    let h = clock.hour();
    let m = even_minute_of(clock);
    let n = rng.random_range(100..999);
    let body = FLOOR_BODIES[rng.random_range(0..FLOOR_BODIES.len())];
    let text = format!("{} {}", timestamp(h, m), fill_n(body, n));
    LogLine::new(text, Classification::Normal, None)
}

/// Each pane's ordinary business, in its own register (第4節). `Kiln`
/// occasionally swaps in the baking-specific line, matching its historical
/// 35% ratio; the other two panes just draw from their own body pool.
fn normal_line_for(pane: Pane, clock: DayClock, rng: &mut ThreadRng) -> LogLine {
    match pane {
        Pane::Kiln => {
            if rng.random_bool(0.35) {
                baking_normal_line(clock, rng)
            } else {
                normal_line(clock, rng)
            }
        }
        Pane::Outside => outside_normal_line(clock, rng),
        Pane::Floor => floor_normal_line(clock, rng),
    }
}

fn memo_line(rng: &mut ThreadRng) -> LogLine {
    let text = FORBIDDEN_MEMOS[rng.random_range(0..FORBIDDEN_MEMOS.len())].to_string();
    LogLine::new(text, Classification::Normal, None)
}

/// 呼びかけ. Its odd-minute tell (第4節) is guaranteed by `weights_for`
/// zeroing this bucket out whenever the shared clock's minute is even
/// (`DayClock::minute_is_odd`) -- this reads the clock's true minute as-is,
/// it never fabricates one.
fn call_line(clock: DayClock, zone: Zone, rng: &mut ThreadRng) -> LogLine {
    debug_assert!(clock.minute_is_odd(), "call_line must only be reached on an odd minute");
    let h = clock.hour();
    let m = clock.minute();
    let loc = zone.location_pool()[rng.random_range(0..zone.location_pool().len())];
    let body = CALL_BODIES[rng.random_range(0..CALL_BODIES.len())];
    let text = format!("{} {}", timestamp(h, m), body.replace("{loc}", loc));
    LogLine::new(text, Classification::ShouldNotReact, None)
}

/// The scripted 二人称 beat that fires the first time the player mishandles a
/// line: the log itself briefly describes the player's own action back to them.
pub fn mistake_beat(clock: DayClock, verb: Verb) -> LogLine {
    let h = clock.hour();
    let m = even_minute_of(clock);
    let text = format!("{} あなたは 行を {}した", timestamp(h, m), verb.label());
    LogLine::new(text, Classification::Normal, None).scripted().with_font(LineFont::Mistake)
}

const CORRUPTED_BODIES: &[&str] = &[
    "何かが{n}本 焼き上がり",
    "帳場に誰かが座っている",
    "あなたの名が{n}回 記帳された",
    "この店は、まだ開いている",
    "続けてください",
    "扉に鍵はかかっていない",
];

/// Unlike every other line, this one's timestamp is *not* a read of the
/// shared clock -- the whole point of "別の何かの記録" (第3.1節) is that time
/// has stopped meaning anything, so an incoherent, independently-rolled hour
/// and minute is the correct behavior here, not a bug to fix.
pub fn corrupted_line(rng: &mut ThreadRng) -> LogLine {
    let h = rng.random_range(0..24);
    let m = rng.random_range(0..60);
    let n = rng.random_range(1..30);
    let body = CORRUPTED_BODIES[rng.random_range(0..CORRUPTED_BODIES.len())];
    let text = format!("{} {}", timestamp(h, m), fill_n(body, n));
    LogLine::new(text, Classification::Normal, None).scripted().with_font(LineFont::Call)
}

/// 呼ばれる: the player's own name, once, at night. Never queued into any
/// pane -- see `GameData::pending_intrusion` and `screens::playing::intrusion`.
pub fn name_call(hour: u32, minute: u32, player_name: &str, loc: &str) -> LogLine {
    let text = format!("{} {}さん、{}", timestamp(hour, minute), player_name, loc);
    LogLine::new(text, Classification::ShouldNotReact, None).scripted().with_font(LineFont::Call)
}

pub fn day_marker(day: u32) -> LogLine {
    let text = format!("開店から{}日目 仕込みを始める", day);
    LogLine::new(text, Classification::Normal, None).scripted()
}

/// How strongly each of a pane's five possible content buckets should be
/// weighted this tick. Every pane can roll every bucket (第3.3節: no
/// classification is off-limits anywhere) -- a bucket at weight 0 just never
/// wins, which is how `Kiln`-only content (deviation, memo) stays exclusive
/// without a separate code path.
struct Weights {
    normal: u32,
    memo: u32,
    deviation: u32,
    repeat: u32,
    call: u32,
}

fn weights_for(pane: Pane, phase: Phase, zone: Zone, day: u32, minute_is_odd: bool) -> Weights {
    let react_base = 12 + (day.min(10) * 2) + if phase == Phase::Peak { 10 } else { 0 };
    // 呼びかけは必ず奇数分(第4節) -- on an even minute this bucket is zeroed
    // out entirely rather than rolled and then reassigned an odd minute, so
    // the tell stays a true read of the shared clock, never a fabricated one.
    let not_react_base = if minute_is_odd {
        4 + if phase == Phase::Night { 10 } else { 2 } + if zone == Zone::Counter { 5 } else { 0 }
    } else {
        0
    };
    let memo = if phase == Phase::Prep { 10 } else { 2 };
    let normal = 60;
    // 原則2(距離だけが縮む): the pane the current escalation stage is
    // "pointed at" gets a flat bonus on top of its own home-turf skew below.
    let zone_bonus = if pane.matches_zone(zone) { 2 } else { 1 };

    match pane {
        Pane::Kiln => Weights {
            normal,
            memo,
            deviation: react_base * zone_bonus,
            repeat: react_base / 4,
            call: not_react_base / 3,
        },
        Pane::Floor => Weights {
            normal,
            memo: 0,
            deviation: 0,
            repeat: react_base * zone_bonus,
            call: not_react_base / 2,
        },
        Pane::Outside => Weights {
            normal,
            memo: 0,
            deviation: 0,
            repeat: react_base / 4,
            call: not_react_base * zone_bonus,
        },
    }
}

pub fn generate(
    pane: Pane,
    clock: DayClock,
    zone: Zone,
    day: u32,
    last_normal: Option<&str>,
    rng: &mut ThreadRng,
) -> LogLine {
    let phase = Phase::for_hour(clock.hour());
    let w = weights_for(pane, phase, zone, day, clock.minute_is_odd());
    let total = w.normal + w.memo + w.deviation + w.repeat + w.call;

    let roll = rng.random_range(0..total);
    if roll < w.normal {
        normal_line_for(pane, clock, rng)
    } else if roll < w.normal + w.memo {
        memo_line(rng)
    } else if roll < w.normal + w.memo + w.deviation {
        deviation_threat_line(clock, rng)
    } else if roll < w.normal + w.memo + w.deviation + w.repeat {
        repeat_threat_line(RuleFlags::compute(day), last_normal, rng)
            .unwrap_or_else(|| normal_line_for(pane, clock, rng))
    } else {
        call_line(clock, zone, rng)
    }
}
