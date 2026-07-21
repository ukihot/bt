use rand::RngExt;
use rand::rngs::ThreadRng;

use super::customer::{CUSTOMERS, CustomerId};
use super::pane::Pane;
use super::rumors::{CATALOG, Effect, ItemKind, RumorId, ThreatKind};

/// The facts a `Condition` (see `rumors::Condition`) needs to evaluate
/// itself. Grows as new conditions need new inputs -- kept separate from
/// `DayClock`/`Phase` so this module never has to depend on Bevy-adjacent
/// domain types it doesn't otherwise need.
#[derive(Clone, Copy, Debug)]
pub struct Context {
    pub day: u32,
}

/// The live answer to "is `threat` currently reactable on `pane`?", after
/// folding every heard rumor's effect in speaking order (第3.4節). No longer
/// carries a `taboo` payload -- `Active` always means "this is now
/// `ShouldReact`, and `log_line::resolve` treats missing it exactly like any
/// other misjudged classification" (タタリ, 原則2). There is nothing softer
/// to distinguish anymore.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Verdict {
    /// Reactable right now.
    Active,
    /// Not reactable right now -- resolves as ordinary business instead.
    Suppressed,
}

/// Which customer voices each `CATALOG` entry *this run*. A rumor's content
/// and mechanical effect are fixed (第3.6節: 脅威それぞれに一貫した「性格」
/// を定義する)，but who delivers it is re-rolled every run -- otherwise
/// "誰が言ったか" would be exactly as memorizable as the fixed rule itself,
/// which is not knowledge a roguelike run is supposed to hand the player for
/// free. Built once per `RuleLedger` and never changed afterward.
#[derive(Clone)]
struct Cast {
    /// Index-aligned with `CATALOG`.
    assignment: Vec<CustomerId>,
}

impl Cast {
    fn roll(rng: &mut ThreadRng) -> Self {
        let assignment: Vec<CustomerId> =
            (0..CATALOG.len()).map(|_| random_customer(rng)).collect();
        Self { assignment }
    }

    fn speaker_of(&self, id: RumorId) -> CustomerId {
        self.assignment[id]
    }
}

fn random_customer(rng: &mut ThreadRng) -> CustomerId {
    CustomerId(rng.random_range(0..CUSTOMERS.len() as u8))
}

/// Today's body of 禁忌集 knowledge: every rumor spoken *today*, in the order
/// it was spoken, plus this run's `Cast`. Nothing is ever removed or
/// reordered within a day -- contradictions are resolved at query time
/// (`verdict`), not by mutating history. `heard` is wiped wholesale at the
/// next in-fiction midnight (`reset_day`) -- rules don't
/// carry over from one day to the next; only `Cast` (who *would* voice each
/// rumor if it comes up) survives the reset, since that's a run-scoped
/// assignment, not a fact about what's been said today.
#[derive(Clone)]
pub struct RuleLedger {
    heard: Vec<RumorId>,
    cast: Cast,
}

impl Default for RuleLedger {
    fn default() -> Self {
        Self::new()
    }
}

impl RuleLedger {
    /// Starts a fresh run's ledger: no rumor heard yet, and a freshly rolled
    /// `Cast` deciding who *would* voice each rumor if it comes up.
    pub fn new() -> Self {
        Self { heard: Vec::new(), cast: Cast::roll(&mut rand::rng()) }
    }

    pub fn hear(&mut self, id: RumorId) {
        self.heard.push(id);
    }

    /// Clears everything heard so far, leaving `Cast` untouched. Called once
    /// per in-fiction midnight (`GameData`'s day-wrap handling) -- a rumor's
    /// mechanical effect only lasts through the rest of the day it was
    /// spoken on; it must be heard again on a later day to matter again
    /// (CLAUDE.md §3.4).
    pub fn reset_day(&mut self) {
        self.heard.clear();
    }

    /// Which customer this run's `Cast` has voicing `id` -- renders a
    /// rumor's `{name}` placeholder (`rumors::rumor_line`).
    pub fn speaker_of(&self, id: RumorId) -> CustomerId {
        self.cast.speaker_of(id)
    }

    /// Scans heard rumors newest-first and returns the first `Enable`/`Void`
    /// that mentions `threat` -- i.e. the most recently spoken word on the
    /// subject wins, exactly matching CLAUDE.md §3.4's "矛盾するルールが
    /// 出た場合は新しいものが優先される". Falls back to each threat kind's
    /// own baseline when nothing heard says anything about it -- and that
    /// baseline is deliberately *not* uniform: `Repeat` and
    /// `ItemMiscount(HotDog)` are reactable from day one (旗揚げゲームの
    /// 「赤上げて」側 -- 何もしなければ既に上がっている), while every other
    /// `ItemMiscount`品目 stays impossible until some rumor `Enable`s it
    /// (何もしなければ上がっていない側)。見た目がほぼ同じ「数え方がおかしい」
    /// 行でも、品目によって既定の"どちら側"が違う、という混乱そのものが狙い
    /// (2026-07-21)。
    pub fn verdict(&self, pane: Pane, threat: ThreatKind, ctx: Context) -> Verdict {
        for &id in self.heard.iter().rev() {
            let def = &CATALOG[id];
            match def.effect {
                Effect::Enable { pane: p, threat: t } if p == pane && t == threat => {
                    return Verdict::Active;
                }
                Effect::Void { threat: t, condition } if t == threat && condition.holds(ctx) => {
                    return Verdict::Suppressed;
                }
                _ => {}
            }
        }
        match threat {
            ThreatKind::Repeat => Verdict::Active,
            ThreatKind::ItemMiscount(ItemKind::HotDog) => Verdict::Active,
            _ => Verdict::Suppressed,
        }
    }

    /// The extra corruption a correct catch of `threat` refunds right now,
    /// on top of the flat -1.0 every correct `ShouldReact` catch already
    /// earns (`log_line::resolve`) -- 0.0 if no live `Effect::Relieve`
    /// currently names `threat`. Same newest-wins scan as `verdict`, kept as
    /// its own query rather than folded into `Verdict` because a charm and a
    /// threat's react/suppress state are independent facts: a `Relieve`
    /// doesn't imply `Active`, and doesn't need to.
    pub fn relief_bonus(&self, threat: ThreatKind) -> f32 {
        for &id in self.heard.iter().rev() {
            let def = &CATALOG[id];
            if let Effect::Relieve { threat: t, bonus } = def.effect
                && t == threat
            {
                return bonus;
            }
        }
        0.0
    }
}

#[cfg(test)]
mod tests {
    use super::super::rumors::Condition;
    use super::*;

    const CTX_NO_THREE: Context = Context { day: 4 };
    const CTX_HAS_THREE: Context = Context { day: 13 };

    fn hear_all(mut ledger: RuleLedger, ids: &[RumorId]) -> RuleLedger {
        for &id in ids {
            ledger.hear(id);
        }
        ledger
    }

    fn find(effect: impl Fn(&Effect) -> bool) -> RumorId {
        CATALOG.iter().position(|def| effect(&def.effect)).expect("no catalog entry matches")
    }

    #[test]
    fn repeat_is_active_from_the_start_on_kiln_and_floor() {
        let ledger = RuleLedger::new();
        assert_eq!(ledger.verdict(Pane::Kiln, ThreatKind::Repeat, CTX_NO_THREE), Verdict::Active);
        assert_eq!(ledger.verdict(Pane::Floor, ThreatKind::Repeat, CTX_NO_THREE), Verdict::Active);
    }

    #[test]
    fn gated_threats_are_suppressed_until_enabled() {
        let ledger = RuleLedger::new();
        assert_eq!(
            ledger.verdict(Pane::Kiln, ThreatKind::NightDelivery, CTX_NO_THREE),
            Verdict::Suppressed
        );
        let enable_night_delivery =
            find(|e| matches!(e, Effect::Enable { threat: ThreatKind::NightDelivery, .. }));
        let ledger = hear_all(ledger, &[enable_night_delivery]);
        assert_eq!(
            ledger.verdict(Pane::Kiln, ThreatKind::NightDelivery, CTX_NO_THREE),
            Verdict::Active
        );
    }

    #[test]
    fn void_only_suppresses_repeat_when_its_condition_holds() {
        let void_repeat = find(|e| matches!(e, Effect::Void { threat: ThreatKind::Repeat, .. }));
        let ledger = hear_all(RuleLedger::new(), &[void_repeat]);
        assert_eq!(ledger.verdict(Pane::Floor, ThreatKind::Repeat, CTX_NO_THREE), Verdict::Active);
        assert_eq!(
            ledger.verdict(Pane::Floor, ThreatKind::Repeat, CTX_HAS_THREE),
            Verdict::Suppressed
        );
    }

    #[test]
    fn void_does_nothing_before_its_rumor_has_been_heard() {
        let ledger = RuleLedger::new();
        // Day-with-three alone isn't enough -- the rule doesn't exist until
        // spoken (第3.4節).
        assert_eq!(ledger.verdict(Pane::Kiln, ThreatKind::Repeat, CTX_HAS_THREE), Verdict::Active);
    }

    #[test]
    fn item_miscount_defaults_are_not_uniform_across_items() {
        // The whole point of splitting ItemMiscount by item (旗揚げゲームの
        // 混乱、CLAUDE.md §4): most items start impossible like NightDelivery,
        // but HotDog starts reactable from day one like Repeat.
        let ledger = RuleLedger::new();
        assert_eq!(
            ledger.verdict(Pane::Kiln, ThreatKind::ItemMiscount(ItemKind::Croissant), CTX_NO_THREE),
            Verdict::Suppressed
        );
        assert_eq!(
            ledger.verdict(Pane::Kiln, ThreatKind::ItemMiscount(ItemKind::ShioPan), CTX_NO_THREE),
            Verdict::Suppressed
        );
        assert_eq!(
            ledger.verdict(Pane::Kiln, ThreatKind::ItemMiscount(ItemKind::MilkLoaf), CTX_NO_THREE),
            Verdict::Suppressed
        );
        assert_eq!(
            ledger.verdict(Pane::Kiln, ThreatKind::ItemMiscount(ItemKind::HotDog), CTX_NO_THREE),
            Verdict::Active
        );
    }

    #[test]
    fn enabling_one_item_s_miscount_does_not_enable_another_s() {
        let enable_croissant = find(|e| {
            matches!(
                e,
                Effect::Enable { threat: ThreatKind::ItemMiscount(ItemKind::Croissant), .. }
            )
        });
        let ledger = hear_all(RuleLedger::new(), &[enable_croissant]);
        assert_eq!(
            ledger.verdict(Pane::Kiln, ThreatKind::ItemMiscount(ItemKind::Croissant), CTX_NO_THREE),
            Verdict::Active
        );
        // A rumor naming one item has nothing to say about a different one.
        assert_eq!(
            ledger.verdict(Pane::Kiln, ThreatKind::ItemMiscount(ItemKind::ShioPan), CTX_NO_THREE),
            Verdict::Suppressed
        );
    }

    #[test]
    fn hot_dog_item_miscount_is_voided_only_on_a_three_day() {
        let void_hot_dog = find(|e| {
            matches!(e, Effect::Void { threat: ThreatKind::ItemMiscount(ItemKind::HotDog), .. })
        });
        let ledger = hear_all(RuleLedger::new(), &[void_hot_dog]);
        assert_eq!(
            ledger.verdict(Pane::Kiln, ThreatKind::ItemMiscount(ItemKind::HotDog), CTX_NO_THREE),
            Verdict::Active
        );
        assert_eq!(
            ledger.verdict(Pane::Kiln, ThreatKind::ItemMiscount(ItemKind::HotDog), CTX_HAS_THREE),
            Verdict::Suppressed
        );
    }

    #[test]
    fn verdict_ignores_unrelated_rumors_interspersed_in_the_ledger() {
        // The scan skips right past rumors with no opinion on this
        // (pane, threat) pair, regardless of where they sit in the timeline.
        let enable_night_delivery =
            find(|e| matches!(e, Effect::Enable { threat: ThreatKind::NightDelivery, .. }));
        let unrelated = find(|e| matches!(e, Effect::None));
        let ledger = hear_all(RuleLedger::new(), &[unrelated, enable_night_delivery, unrelated]);
        assert_eq!(
            ledger.verdict(Pane::Kiln, ThreatKind::NightDelivery, CTX_NO_THREE),
            Verdict::Active
        );
    }

    #[test]
    fn relief_bonus_is_zero_until_its_rumor_is_heard() {
        let ledger = RuleLedger::new();
        assert_eq!(ledger.relief_bonus(ThreatKind::Repeat), 0.0);
    }

    #[test]
    fn relief_bonus_applies_once_heard() {
        let relieve_repeat =
            find(|e| matches!(e, Effect::Relieve { threat: ThreatKind::Repeat, .. }));
        let ledger = hear_all(RuleLedger::new(), &[relieve_repeat]);
        assert!(ledger.relief_bonus(ThreatKind::Repeat) > 0.0);
        // A charm about `Repeat` has nothing to say about an unrelated threat.
        assert_eq!(ledger.relief_bonus(ThreatKind::NightDelivery), 0.0);
    }

    #[test]
    fn cast_speaker_assignment_varies_across_rolls() {
        // Directly answers the roguelike complaint this module fixes: the
        // same rumor must not be voiced by the same customer every run.
        let speakers: Vec<CustomerId> = (0..40).map(|_| RuleLedger::new().speaker_of(0)).collect();
        assert!(
            speakers.iter().any(|&s| s != speakers[0]),
            "rolled the same speaker for rumor 0 in all 40 runs"
        );
    }

    #[test]
    fn condition_variants_are_exercised() {
        // Guards against `Condition` growing a variant that `holds` forgets
        // to handle in a non-exhaustive way.
        assert!(Condition::DayHasThree.holds(CTX_HAS_THREE));
        assert!(!Condition::DayHasThree.holds(CTX_NO_THREE));
    }

    #[test]
    fn reset_day_clears_heard_rumors_but_keeps_the_cast() {
        let enable_night_delivery =
            find(|e| matches!(e, Effect::Enable { threat: ThreatKind::NightDelivery, .. }));
        let mut ledger = hear_all(RuleLedger::new(), &[enable_night_delivery]);
        let speaker_before = ledger.speaker_of(enable_night_delivery);
        assert_eq!(
            ledger.verdict(Pane::Kiln, ThreatKind::NightDelivery, CTX_NO_THREE),
            Verdict::Active
        );

        ledger.reset_day();

        // The effect no longer applies -- it must be heard again today.
        assert_eq!(
            ledger.verdict(Pane::Kiln, ThreatKind::NightDelivery, CTX_NO_THREE),
            Verdict::Suppressed
        );
        // But who *would* voice it, if re-heard, hasn't been rerolled.
        assert_eq!(ledger.speaker_of(enable_night_delivery), speaker_before);
    }
}
