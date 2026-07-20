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
/// rule and every threat-generator agree on.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ThreatKind {
    /// 焼成室・売り場の既定の反復 -- reactable from day one, not gated by
    /// any rumor; only ever *voided*, never *enabled*.
    Repeat,
    /// 外画面固有の反復(禁忌#9) -- the mirror image of `Repeat`: impossible
    /// until its rumor enables it.
    OutsideRepeat,
    /// 夜の納品(禁忌#2)
    NightDelivery,
    /// 閉店時間(禁忌#7)
    ClosingTime,
    /// 裏口(禁忌#8)
    BackDoor,
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

/// What a rumor does to the ruleset the moment it's spoken (第3.4節). Note
/// what's *not* here: no `CustomerId`. A rumor's mechanical personality is
/// fixed catalog data, but who actually voices it is rolled fresh every run
/// (`rules::RuleLedger`/`Cast`) -- so effects that need to name a speaker
/// (`Discredit`) point at another *catalog entry*, not a customer, and get
/// resolved through that run's `Cast` at query time.
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
    /// Declares whoever this run's `Cast` has voicing `target` unreliable --
    /// every `Enable`/`Void`/`Relieve` sourced from *their* rumors stops
    /// applying from this point on, as if never spoken. See
    /// `RuleLedger::is_discredited`.
    Discredit { target: RumorId },
}

pub type RumorId = usize;

pub struct RumorDef {
    /// May contain a `{name}` placeholder, filled in at render time with
    /// this run's `Cast`-assigned speaker (`fill_name`) -- see `rumor_line`.
    pub body: &'static str,
    pub effect: Effect,
}

/// Index into `CATALOG` of the 裏口 rumor (禁忌#8) that the `Discredit`
/// entry later casts doubt on. Named and checked by
/// `discredit_target_is_still_the_back_door_rumor` so that reordering
/// `CATALOG` fails a test instead of silently discrediting the wrong rumor.
const BACK_DOOR_RUMOR: RumorId = 7;

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
        body: "常連の{name}さんが「この店、閉店時間だけは聞かないほうがいいですよ」と、困ったように話していらっしゃいました",
        effect: Effect::Enable { pane: Pane::Floor, threat: ThreatKind::ClosingTime },
    },
    RumorDef {
        // BACK_DOOR_RUMOR -- keep index 7 in sync with the constant above.
        body: "裏口にだけは近づかないほうがいいのだと、{name}さんがめずらしく真剣な声でおっしゃっていました",
        effect: Effect::Enable { pane: Pane::Outside, threat: ThreatKind::BackDoor },
    },
    RumorDef {
        body: "{name}さんが「外の異常なしは、二回続けて信じてはいけませんよ」と、どこか怯えた様子でおっしゃっていました",
        effect: Effect::Enable { pane: Pane::Outside, threat: ThreatKind::OutsideRepeat },
    },
    RumorDef {
        body: "裏口の話をしていた人のことは、あまり当てにしないほうがいいと、{name}さんが耳打ちしてきました",
        effect: Effect::Discredit { target: BACK_DOOR_RUMOR },
    },
    RumorDef {
        body: "{name}さんが「帳面の同じ行に気づいたら、すぐに消してしまうと楽になりますよ」と、にっこり教えてくれました",
        effect: Effect::Relieve { threat: ThreatKind::Repeat, bonus: 3.0 },
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
    fn discredit_targets_point_at_a_real_other_catalog_entry() {
        for (id, def) in CATALOG.iter().enumerate() {
            if let Effect::Discredit { target } = def.effect {
                assert!(target < CATALOG.len());
                assert_ne!(target, id, "a rumor cannot discredit itself");
            }
        }
    }

    #[test]
    fn discredit_target_is_still_the_back_door_rumor() {
        assert!(matches!(
            CATALOG[BACK_DOOR_RUMOR].effect,
            Effect::Enable { threat: ThreatKind::BackDoor, .. }
        ));
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
        // 200 draws from an 11-entry catalog should have hit every one of the
        // gated entries at least once.
        assert_eq!(
            ledger.verdict(Pane::Kiln, ThreatKind::NightDelivery, Context { day: 1 }),
            super::super::rules::Verdict::Active
        );
    }
}
