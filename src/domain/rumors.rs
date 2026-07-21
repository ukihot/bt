use rand::RngExt;
use rand::rngs::ThreadRng;

use super::clock::DayClock;
use super::log_line::{Classification, LogLine};
use super::pane::Pane;
use super::rules::{Context, RuleLedger};
use super::timestamp::{even_minute_of, fill_name, timestamp};

/// Which recurring anomaly pattern a rumor's effect (below) talks about.
/// Replaces what used to be a scattered set of independent bools and
/// per-pane match arms in `generate.rs`/`threats.rs` with one name every
/// rule and every threat-generator agree on. Every variant here is 焼成室
/// content only (第3.2節: 操作できるのは焼成室だけ) -- `Outside`/`Floor`
/// used to have their own reactable variants (`OutsideRepeat`/`ClosingTime`/
/// `BackDoor`), but those required a correct action on panes that were never
/// actually operable, so they're gone (2026-07-21) rather than kept as
/// mechanically-inert flavor -- see CLAUDE.md §9.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ThreatKind {
    /// 焼成室の既定の反復 -- reactable from day one, not gated by any
    /// rumor; only ever *voided*, never *enabled*.
    Repeat,
    /// 夜の納品(禁忌#2)
    NightDelivery,
    /// バゲット以外のパンの数え違い(2026-07-21追加)。`ItemKind` ごとに
    /// 独立した `ThreatKind` -- 「パンの数え間違い」という1つの脅威では
    /// なく、品目の数だけ別々に有効/無効を切り替えられる。旗揚げゲームの
    /// 「赤上げて、白上げて」の混乱を狙った仕掛けで、各品目の既定状態
    /// (`rules::RuleLedger::verdict` のフォールバック)も品目ごとにバラバラ
    /// にしてある -- 同じ「数え方がおかしい」という見た目の行が、聞いた噂
    /// 次第で品目ごとに正解の動詞が違う、という状態を作るのが狙い。
    ItemMiscount(ItemKind),
}

/// バゲット以外の主要パン(第4節: バゲットの「本」数え・偶数分丸めと並ぶ、
/// 品目ごとの数え方の筆癖を広げるための追加)。`correct_counter`/
/// `wrong_counter` はどちらも実在しそうな助数詞から選んでいる -- 明らかな
/// でたらめ(「@個」のような)にはしない。異常だと気づけるかどうかは、
/// この店の筆癖(第4節)をどれだけ読み込んでいるか次第、という原則1の
/// 範囲に留める。
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ItemKind {
    Croissant,
    ShioPan,
    HotDog,
    MilkLoaf,
}

impl ItemKind {
    pub fn name(self) -> &'static str {
        match self {
            ItemKind::Croissant => "クロワッサン",
            ItemKind::ShioPan => "塩パン",
            ItemKind::HotDog => "ホットドッグ",
            ItemKind::MilkLoaf => "食パン",
        }
    }

    /// The counter word this shop's own bookkeeping actually uses for this
    /// item -- never appears in `ItemMiscount` lines themselves (those
    /// always use `wrong_counter`); only relevant if a future normal-line
    /// generator wants to mention the item correctly.
    pub fn correct_counter(self) -> &'static str {
        match self {
            ItemKind::Croissant => "個",
            ItemKind::ShioPan => "個",
            ItemKind::HotDog => "本",
            ItemKind::MilkLoaf => "斤",
        }
    }

    /// The mismatched counter `generate::item_miscount_line` writes instead
    /// -- plausible on its own (it's a real counter word), just wrong for
    /// this particular item.
    pub fn wrong_counter(self) -> &'static str {
        match self {
            ItemKind::Croissant => "本",
            ItemKind::ShioPan => "枚",
            ItemKind::HotDog => "個",
            ItemKind::MilkLoaf => "本",
        }
    }
}

/// A condition a `Void` effect can be guarded on. Only one variant exists
/// today; new ones slot in here and into `holds` without touching
/// `RuleLedger::verdict` at all.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Condition {
    /// 三のつく日 -- the day number contains the digit 3.
    DayHasThree,
}

impl Condition {
    pub fn holds(self, ctx: Context) -> bool {
        match self {
            Condition::DayHasThree => ctx.day.to_string().contains('3'),
        }
    }
}

/// What a rumor does to the ruleset the moment it's spoken (第3.4節).
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Effect {
    /// Pure foreshadowing -- describes a rule that's already always active
    /// (奇数分・二度聞き・名指し・焼き上がりの字, 第4節). Hearing it changes
    /// nothing mechanically.
    None,
    /// From the moment this is heard, `threat` becomes reactable on `pane`.
    /// Missing or misjudging it is タタリ exactly like any other misjudged
    /// classification (`log_line::resolve`) -- there is no separate taboo
    /// flag anymore; moving a phenomenon onto `ShouldReact`/`ShouldNotReact`
    /// is itself what makes getting it wrong costly.
    Enable { pane: Pane, threat: ThreatKind },
    /// From the moment this is heard, `threat` stops being reactable
    /// whenever `condition` holds -- it resolves as ordinary business
    /// instead. Not pane-scoped: today's only case (三のつく日) applies
    /// wherever `Repeat` would otherwise fire.
    Void { threat: ThreatKind, condition: Condition },
    /// 援助系の禁忌(民俗的なお守り): from the moment this is heard,
    /// correctly catching `threat` refunds an *extra* `bonus` corruption on
    /// top of the flat -1.0 every correct catch already earns
    /// (`log_line::resolve`) -- a folk charm's counterpart to the taboos
    /// above, not their opposite mechanism: it still only ever moves
    /// corruption, never `Zone` (原則2: 距離だけが縮む is about the
    /// threat's physical proximity, which this doesn't touch -- see
    /// `LogLine::relieved`).
    Relieve { threat: ThreatKind, bonus: f32 },
}

pub type RumorId = usize;

pub struct RumorDef {
    /// May contain a `{name}` placeholder, filled in at render time with
    /// this run's `Cast`-assigned speaker (`fill_name`) -- see `rumor_line`.
    pub body: &'static str,
    pub effect: Effect,
}

/// 禁忌集(第5節): 売り場の客の噂話・世間話に紛れて流れる。店主のメモとい
/// う体裁はもう取らない -- 焼成室には一切書かれず、読むだけの画面(売り場)
/// だけがルールの出所になる(第3.4・4節)。この配列がその内容と効果、
/// 両方の唯一の情報源 -- 新しい禁忌を1つ足すのに必要なのはここへの1行と
/// (必要なら)`ThreatKind`への1バリアントだけで済む。誰が話すかはここには
/// 書かない(`rules::Cast`参照)。
pub const CATALOG: &[RumorDef] = &[
    RumorDef {
        body: "{name}さんが、奇数分に帳面をつけるのは良くないと、真顔でおっしゃっていました",
        effect: Effect::None,
    },
    RumorDef {
        body: "{name}さんが「夜の納品だけは、お断りしたほうがいいですよ」と念を押していらっしゃいました",
        effect: Effect::Enable { pane: Pane::Kiln, threat: ThreatKind::NightDelivery },
    },
    RumorDef {
        body: "三のつく日は窯の火を落とさないほうがいいのだと、{name}さんが声をひそめておっしゃっていました",
        effect: Effect::Void { threat: ThreatKind::Repeat, condition: Condition::DayHasThree },
    },
    RumorDef {
        body: "{name}さんが「同じ話を二度されても、続けて聞かないほうがいいですよ」と教えてくださいました",
        effect: Effect::None,
    },
    RumorDef {
        body: "名前を呼ばれても、すぐには信じないほうがいいのだと、{name}さんがぽつりとおっしゃっていました",
        effect: Effect::None,
    },
    RumorDef {
        body: "{name}さんに「焼き上がりの字だけは、崩さないでくださいね」と頼まれました",
        effect: Effect::None,
    },
    RumorDef {
        body: "{name}さんが「帳面の同じ行に気づいたら、すぐに消してしまうと楽になりますよ」と、にっこり教えてくれました",
        effect: Effect::Relieve { threat: ThreatKind::Repeat, bonus: 3.0 },
    },
    RumorDef {
        body: "{name}さんが「クロワッサンの数が違っていたら、ちゃんと消したほうがいいですよ」と、真面目な顔でおっしゃっていました",
        effect: Effect::Enable {
            pane: Pane::Kiln,
            threat: ThreatKind::ItemMiscount(ItemKind::Croissant),
        },
    },
    RumorDef {
        body: "{name}さんが「塩パンだけは、数え間違いを見逃さないほうがいいらしいです」と教えてくださいました",
        effect: Effect::Enable {
            pane: Pane::Kiln,
            threat: ThreatKind::ItemMiscount(ItemKind::ShioPan),
        },
    },
    RumorDef {
        body: "{name}さんが「食パンの数だけは、間違えたままにしないほうがいいそうです」とおっしゃっていました",
        effect: Effect::Enable {
            pane: Pane::Kiln,
            threat: ThreatKind::ItemMiscount(ItemKind::MilkLoaf),
        },
    },
    RumorDef {
        body: "{name}さんが「ホットドッグの数は、三のつく日だけ気にしなくていいらしいですよ」と、笑いながら教えてくれました",
        effect: Effect::Void {
            threat: ThreatKind::ItemMiscount(ItemKind::HotDog),
            condition: Condition::DayHasThree,
        },
    },
];

pub fn rumor_line(clock: DayClock, ledger: &mut RuleLedger, rng: &mut ThreadRng) -> LogLine {
    let id = rng.random_range(0..CATALOG.len());
    ledger.hear(id);
    let def = &CATALOG[id];
    let h = clock.hour();
    let m = even_minute_of(clock);
    let body = fill_name(def.body, ledger.speaker_of(id).name());
    let text = format!("{} {}", timestamp(h, m), body);
    LogLine::new(text, Classification::Normal)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn catalog_bodies_are_well_formed() {
        for def in CATALOG {
            assert!(!def.body.is_empty());
        }
    }

    #[test]
    fn rumor_line_records_every_pick_into_the_ledger_fills_the_speaker_name_and_stamps_the_time() {
        let mut ledger = RuleLedger::new();
        let mut rng = rand::rng();
        for _ in 0..200 {
            let line = rumor_line(DayClock::at(10, 30), &mut ledger, &mut rng);
            assert_eq!(line.classification, Classification::Normal);
            assert!(!line.text.contains("{name}"));
            assert!(line.text.starts_with("10:30 "));
        }
        // 200 draws from an 11-entry catalog should have hit the one gated
        // entry at least once.
        assert_eq!(
            ledger.verdict(Pane::Kiln, ThreatKind::NightDelivery, Context { day: 1 }),
            super::super::rules::Verdict::Active
        );
    }
}
