use rand::RngExt;
use rand::rngs::ThreadRng;

use super::clock::DayClock;
use super::log_line::{Classification, LineFont, LogLine};
use super::pane::Pane;
use super::phase::Phase;
use super::rules::{Context, RuleLedger, Verdict};
use super::rumors::{ItemKind, ThreatKind, rumor_line};
use super::threats::{deviation_threat_line, night_delivery_threat_line};
use super::timestamp::{even_minute_of, fill_n, timestamp};
use super::verb::Verb;
use super::zone::Zone;

/// Every item `item_normal_line`/`item_miscount_line` can pick from, besides
/// the baguette (which keeps its own dedicated `baking_normal_line` and
/// always-`ShouldReact` deviations, per `deviation_threat_line` -- neither
/// touched by rumors, unlike this pool's items).
const ITEM_POOL: &[ItemKind] =
    &[ItemKind::Croissant, ItemKind::ShioPan, ItemKind::HotDog, ItemKind::MilkLoaf];

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

/// 外画面の通常行、侵食度の段階ごとの語彙プール(第3.5節: 「異常なし」を
/// 基本単位に、定型報告の文法を保ったまま語彙だけが徐々に不穏な方向へ動く)。
/// `corruption`(0-100, プレイヤーには数値として一切見せない、第8節)を
/// 唯一の入力として `outside_body_pool` が段階を選ぶ -- 外はゲージの
/// 代わりに読む「体温計」(第3.1節)なので、この語彙の変化だけが侵食の
/// 進み具合を伝える手段になる。各段階とも「異常なし」を完全には手放さない
/// (出現数を減らすだけ) -- 禁忌#9「外の異常なしを二回続けて信じてはいけ
/// ない」が全段階で意味を持ち続けるために、この一文自体はどの段階でも
/// 出続ける必要がある。
const OUTSIDE_CALM: &[&str] = &[
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
const OUTSIDE_UNEASY: &[&str] = &[
    "異常なし",
    "異常なし",
    "異常なし",
    "ゴミが{n}個 転がってきた",
    "野良猫が、少し長くこちらを見ていた",
    "自転車が倒れていた",
    "看板が揺れている",
    "配達車が、いつもより遅かった",
    "特に変わりなし",
];
const OUTSIDE_ALARMED: &[&str] = &[
    "異常なし",
    "異常なし",
    "同じ車が、二度通り過ぎた",
    "足音が、しばらく続いた",
    "看板の文字が、読み取れなかった",
    "野良猫が、動かなかった",
    "特に……変わりなし",
];
const OUTSIDE_DREAD: &[&str] = &[
    "異常なし",
    "足音が、止まらない",
    "窓の外に、誰か立っていた",
    "灯りが、いつもより暗い",
    "呼ぶ声が、した気がする",
    "……なし",
];

/// `corruption`の値だけを見て語彙プールの段階を選ぶ純粋な関数。閾値
/// (25/50/75)は数値調整の対象(CLAUDE.md第9節)であって構造ではない。
fn outside_body_pool(corruption: f32) -> &'static [&'static str] {
    if corruption < 25.0 {
        OUTSIDE_CALM
    } else if corruption < 50.0 {
        OUTSIDE_UNEASY
    } else if corruption < 75.0 {
        OUTSIDE_ALARMED
    } else {
        OUTSIDE_DREAD
    }
}

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
    LogLine::new(text, Classification::Normal)
}

fn baking_normal_line(clock: DayClock, rng: &mut ThreadRng) -> LogLine {
    let h = clock.hour();
    let m = even_minute_of(clock);
    let n = rng.random_range(1..20);
    let text = format!("{} バゲットが{}本 焼き上がり", timestamp(h, m), n);
    LogLine::new(text, Classification::Normal)
}

/// The correctly-counted baseline for `ITEM_POOL`'s items, mirroring
/// `baking_normal_line`'s role for the baguette -- without this, the only
/// place these items would ever appear is `item_miscount_line`, always
/// miscounted, which would make the "wrong" counter the only counter a
/// player ever sees (nothing to read it *against*). This is what actually
/// teaches the correct counter well enough for `item_miscount_line`'s wrong
/// one to land as wrong (第4節: 異常は筆癖でバレる、筆癖を知らなければ
/// 何も見えない)。
fn item_normal_line(clock: DayClock, rng: &mut ThreadRng) -> LogLine {
    let item = ITEM_POOL[rng.random_range(0..ITEM_POOL.len())];
    let h = clock.hour();
    let m = even_minute_of(clock);
    let n = rng.random_range(1..20);
    let text =
        format!("{} {}が{}{} 焼き上がり", timestamp(h, m), item.name(), n, item.correct_counter());
    LogLine::new(text, Classification::Normal)
}

fn outside_normal_line(clock: DayClock, corruption: f32, rng: &mut ThreadRng) -> LogLine {
    let h = clock.hour();
    let m = even_minute_of(clock);
    let n = rng.random_range(1..10);
    let pool = outside_body_pool(corruption);
    let body = pool[rng.random_range(0..pool.len())];
    let text = format!("{} {}", timestamp(h, m), fill_n(body, n));
    LogLine::new(text, Classification::Normal)
}

fn floor_normal_line(clock: DayClock, rng: &mut ThreadRng) -> LogLine {
    let h = clock.hour();
    let m = even_minute_of(clock);
    let n = rng.random_range(100..999);
    let body = FLOOR_BODIES[rng.random_range(0..FLOOR_BODIES.len())];
    let text = format!("{} {}", timestamp(h, m), fill_n(body, n));
    LogLine::new(text, Classification::Normal)
}

/// Each pane's ordinary business, in its own register (第4節). `Kiln`
/// occasionally swaps in the baking-specific line, matching its historical
/// 35% ratio, and separately has a smaller chance of a correctly-counted
/// `ITEM_POOL` line (`item_normal_line`); the other two panes just draw from
/// their own body pool. `corruption` only matters to `Outside`
/// (`outside_body_pool`) -- threaded through here anyway so every call site
/// (including `repeated_line`'s fallbacks) can stay pane-agnostic.
fn normal_line_for(pane: Pane, clock: DayClock, corruption: f32, rng: &mut ThreadRng) -> LogLine {
    match pane {
        Pane::Kiln => {
            if rng.random_bool(0.35) {
                baking_normal_line(clock, rng)
            } else if rng.random_bool(0.15) {
                item_normal_line(clock, rng)
            } else {
                normal_line(clock, rng)
            }
        }
        Pane::Outside => outside_normal_line(clock, corruption, rng),
        Pane::Floor => floor_normal_line(clock, rng),
    }
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
    LogLine::new(text, Classification::ShouldNotReact)
}

/// The scripted 二人称 beat that fires the first time the player mishandles a
/// line: the log itself briefly describes the player's own action back to them.
pub fn mistake_beat(clock: DayClock, verb: Verb) -> LogLine {
    let h = clock.hour();
    let m = even_minute_of(clock);
    let text = format!("{} あなたは 行を {}した", timestamp(h, m), verb.label());
    LogLine::new(text, Classification::Normal).scripted().with_font(LineFont::Mistake)
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
    LogLine::new(text, Classification::Normal).scripted().with_font(LineFont::Call)
}

/// 呼ばれる: the player's own name, once, at night. Never queued into any
/// pane -- see `GameData::pending_intrusion` and `screens::playing::intrusion`.
pub fn name_call(hour: u32, minute: u32, player_name: &str, loc: &str) -> LogLine {
    let text = format!("{} {}さん、{}", timestamp(hour, minute), player_name, loc);
    LogLine::new(text, Classification::ShouldNotReact).scripted().with_font(LineFont::Call)
}

pub fn day_marker(day: u32) -> LogLine {
    let text = format!("開店から{}日目 仕込みを始める", day);
    LogLine::new(text, Classification::Normal).scripted()
}

/// 売り場側の日替わり合図。ルールの効果は日ごとにリセットされる
/// (`CLAUDE.md` §3.4, `rules::RuleLedger::reset_day`)が、それを
/// 「リセットされました」のようなメタな文面では告げない -- 客の世間話
/// らしい、ただ「昨日の話はもう終わった」ことをほのめかすだけの一言に留め、
/// `day_marker`(焼成室側)と対になる形で日をまたぐたびに売り場に1件流す。
const RESET_NOTICE_BODIES: &[&str] = &[
    "今日はいつも通りで大丈夫みたいですよ",
    "もう気にしなくていいと聞きました",
    "昨日の話は、もう終わったことだそうです",
];

pub fn rule_reset_notice(clock: DayClock, rng: &mut ThreadRng) -> LogLine {
    let h = clock.hour();
    let m = even_minute_of(clock);
    let body = RESET_NOTICE_BODIES[rng.random_range(0..RESET_NOTICE_BODIES.len())];
    let text = format!("{} {}", timestamp(h, m), body);
    LogLine::new(text, Classification::Normal).scripted()
}

/// How strongly each of a pane's content buckets should be weighted this
/// tick. `Kiln` is the only pane that ever rolls `deviation`/`repeat`/
/// `taboo_event` -- 焼成室以外に操作対象は存在しない(第3.2節)ので、
/// `Outside`/`Floor` have no "correct action" for a threat bucket to mean
/// anything (`screens::playing::spawn::line_spawn` never calls `resolve()`
/// for their evicted lines at all). Those two panes' buckets simply stay at
/// weight 0 rather than needing a separate code path.
struct Weights {
    normal: u32,
    rumor: u32,
    deviation: u32,
    repeat: u32,
    /// 夜の納品(禁忌#2): only exists once its rumor has actually been
    /// heard this run -- 0 until then, so the event simply never happens
    /// rather than happening but reading as `Normal`.
    taboo_event: u32,
    /// バゲット以外の品目の数え違い(`item_miscount_line`)。`deviation`とは
    /// 独立したバケット -- こちらは常に生成され、`Verdict`で分類だけが
    /// 変わる(`taboo_event`のように重み0で起こらなくなることはない)。
    item_miscount: u32,
    call: u32,
}

/// Whether `threat` is currently reactable on `pane` at all -- collapses a
/// `Verdict` down to the yes/no `weights_for` needs to decide if its bucket
/// should be rollable this tick.
fn is_active(ledger: &RuleLedger, pane: Pane, threat: ThreatKind, day: u32) -> bool {
    matches!(ledger.verdict(pane, threat, Context { day }), Verdict::Active)
}

fn weights_for(
    pane: Pane,
    phase: Phase,
    zone: Zone,
    day: u32,
    minute_is_odd: bool,
    ledger: &RuleLedger,
) -> Weights {
    let react_base = 12 + (day.min(10) * 2) + if phase == Phase::Peak { 10 } else { 0 };
    // 呼びかけは必ず奇数分(第4節) -- on an even minute this bucket is zeroed
    // out entirely rather than rolled and then reassigned an odd minute, so
    // the tell stays a true read of the shared clock, never a fabricated one.
    let not_react_base = if minute_is_odd {
        4 + if phase == Phase::Night { 10 } else { 2 } + if zone == Zone::Counter { 5 } else { 0 }
    } else {
        0
    };
    // 禁忌集は売り場の噂話としてのみ流れる(第5節) -- 仕込みの静けさの中で
    // 読む時間、という第6節の位置づけはそのまま Floor 側に引き継ぐ。
    let rumor = if phase == Phase::Prep { 10 } else { 2 };
    let normal = 60;
    // 原則2(距離だけが縮む): the pane the current escalation stage is
    // "pointed at" gets a flat bonus on top of its own home-turf skew below.
    let zone_bonus = if pane.matches_zone(zone) { 2 } else { 1 };

    // 禁忌#2(夜の納品): 焼成室固有、`RuleLedger`が有効と判定して初めて
    // 起こり得る異常。時間帯まで揃わないと現れない -- 噂の内容そのものが
    // 夜限定の作法だから(この時間帯条件はルール台帳ではなくここに残す:
    // 台帳が答えるのは「有効かどうか」だけ)。焼成室以外は操作対象では
    // ないので(第3.2節)、そもそもこのバケットを持たない。
    let taboo_event = if pane == Pane::Kiln
        && is_active(ledger, pane, ThreatKind::NightDelivery, day)
        && phase == Phase::Night
    {
        14
    } else {
        0
    };

    match pane {
        Pane::Kiln => Weights {
            normal,
            rumor: 0,
            deviation: react_base * zone_bonus,
            repeat: react_base / 4,
            taboo_event,
            item_miscount: react_base / 3,
            call: not_react_base / 3,
        },
        Pane::Floor => Weights {
            normal,
            rumor,
            deviation: 0,
            repeat: 0,
            taboo_event: 0,
            item_miscount: 0,
            call: not_react_base / 2,
        },
        Pane::Outside => Weights {
            normal,
            rumor: 0,
            deviation: 0,
            repeat: 0,
            taboo_event: 0,
            item_miscount: 0,
            call: not_react_base * zone_bonus,
        },
    }
}

/// Builds the `Repeat` bucket's line once its `Verdict` is known -- only
/// ever reached for `Kiln` (`weights_for` keeps `repeat` at 0 everywhere
/// else). `relief` (`RuleLedger::relief_bonus`) is only ever nonzero when
/// `verdict` is `Active` -- see `Effect::Relieve`.
fn repeated_line(
    verdict: Verdict,
    relief: f32,
    last_normal: Option<&str>,
    pane: Pane,
    clock: DayClock,
    corruption: f32,
    rng: &mut ThreadRng,
) -> LogLine {
    match verdict {
        Verdict::Active => match last_normal {
            Some(text) => {
                let mut line = LogLine::new(text.to_string(), Classification::ShouldReact);
                if relief > 0.0 {
                    line = line.relieved(relief);
                }
                line
            }
            None => normal_line_for(pane, clock, corruption, rng),
        },
        Verdict::Suppressed => match last_normal {
            Some(text) => LogLine::new(text.to_string(), Classification::Normal),
            None => normal_line_for(pane, clock, corruption, rng),
        },
    }
}

/// 焼成室固有、バゲット以外の品目の数え違い(2026-07-21追加、CLAUDE.md §4)。
/// 生成される文面は正誤にかかわらず常に同じ形(`item.wrong_counter()` を
/// 使った、もっともらしいが間違った数え方)——`Verdict` が変えるのは分類
/// (検印が正解か削除が正解か)だけで、見た目には一切出ない。旗揚げゲームの
/// 「赤上げて、白上げて」そのままに、品目ごとに違う既定値と、噂で個別に
/// 上書きされた履歴を、プレイヤー自身が覚えておく必要がある。
fn item_miscount_line(
    clock: DayClock,
    ledger: &RuleLedger,
    day: u32,
    rng: &mut ThreadRng,
) -> LogLine {
    let item = ITEM_POOL[rng.random_range(0..ITEM_POOL.len())];
    let h = clock.hour();
    let m = even_minute_of(clock);
    let n = rng.random_range(1..20);
    let text =
        format!("{} {}が{}{} 焼き上がり", timestamp(h, m), item.name(), n, item.wrong_counter());

    let verdict = ledger.verdict(Pane::Kiln, ThreatKind::ItemMiscount(item), Context { day });
    match verdict {
        Verdict::Active => LogLine::new(text, Classification::ShouldReact),
        Verdict::Suppressed => LogLine::new(text, Classification::Normal),
    }
}

#[allow(clippy::too_many_arguments)]
pub fn generate(
    pane: Pane,
    clock: DayClock,
    zone: Zone,
    day: u32,
    last_normal: Option<&str>,
    ledger: &mut RuleLedger,
    corruption: f32,
    rng: &mut ThreadRng,
) -> LogLine {
    let phase = Phase::for_hour(clock.hour());
    let w = weights_for(pane, phase, zone, day, clock.minute_is_odd(), ledger);
    let total =
        w.normal + w.rumor + w.deviation + w.repeat + w.taboo_event + w.item_miscount + w.call;

    let roll = rng.random_range(0..total);
    if roll < w.normal {
        normal_line_for(pane, clock, corruption, rng)
    } else if roll < w.normal + w.rumor {
        rumor_line(clock, ledger, rng)
    } else if roll < w.normal + w.rumor + w.deviation {
        deviation_threat_line(clock, rng)
    } else if roll < w.normal + w.rumor + w.deviation + w.repeat {
        // 三のつく日の禁忌(#3)で無効化され得る、焼成室の通常の反復
        // (`weights_for` が `repeat` を焼成室以外では常に0にしているので、
        // ここに来るのは常に `Pane::Kiln`)。
        let verdict = ledger.verdict(pane, ThreatKind::Repeat, Context { day });
        let relief = ledger.relief_bonus(ThreatKind::Repeat);
        repeated_line(verdict, relief, last_normal, pane, clock, corruption, rng)
    } else if roll < w.normal + w.rumor + w.deviation + w.repeat + w.taboo_event {
        // `weights_for` が `taboo_event` を焼成室以外では常に0にしている
        // ので、ここに来るのも常に `Pane::Kiln`。
        night_delivery_threat_line(clock, rng)
    } else if roll < w.normal + w.rumor + w.deviation + w.repeat + w.taboo_event + w.item_miscount {
        // `weights_for` が `item_miscount` を焼成室以外では常に0にしている
        // ので、ここに来るのも常に `Pane::Kiln`。
        item_miscount_line(clock, ledger, day, rng)
    } else {
        call_line(clock, zone, rng)
    }
}
