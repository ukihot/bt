use rand::RngExt;
use rand::rngs::ThreadRng;

use super::customer::{CUSTOMERS, CustomerId};
use super::pane::Pane;
use super::rumors::{CATALOG, Effect, RumorId, ThreatKind};

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
        let mut assignment: Vec<CustomerId> =
            (0..CATALOG.len()).map(|_| random_customer(rng)).collect();
        // A rumor that discredits another rumor's speaker can't credibly be
        // voiced by that same speaker -- reroll until they differ. The
        // catalog is a handful of entries, so plain rejection sampling is
        // fine; see `cast_never_assigns_a_discredit_speaker_equal_to_their_target`.
        for (id, def) in CATALOG.iter().enumerate() {
            if let Effect::Discredit { target } = def.effect {
                while assignment[id] == assignment[target] {
                    assignment[id] = random_customer(rng);
                }
            }
        }
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
/// (`verdict`), not by mutating history, so a rumor that's later discredited
/// didn't stop being heard, it just stops being trusted. `heard` is wiped
/// wholesale at the next in-fiction midnight (`reset_day`) -- rules don't
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
    /// (CLAUDE.md §3.4). This applies uniformly, including `Discredit`: a
    /// speaker's credibility doesn't survive the reset either.
    pub fn reset_day(&mut self) {
        self.heard.clear();
    }

    /// Which customer this run's `Cast` has voicing `id` -- used both to
    /// render a rumor's `{name}` placeholder (`rumors::rumor_line`) and,
    /// internally, to resolve `Effect::Discredit` targets.
    pub fn speaker_of(&self, id: RumorId) -> CustomerId {
        self.cast.speaker_of(id)
    }

    /// Whether *any* heard rumor has discredited `customer` -- discrediting
    /// isn't a point-in-time event, it's a standing fact about the rest of
    /// the run once spoken (CLAUDE.md §5: 終盤に示してよい事実、と同じ
    /// 「後から効いてくる」時間差の一種)。
    ///
    /// Deliberately does *not* chase chains: a `Discredit` sourced from a
    /// speaker who is themselves later discredited still stands. Reversing
    /// a reversal is a real design question (whose word counts once every
    /// witness is suspect?) that CLAUDE.md doesn't answer yet, so v1 doesn't
    /// invent an answer -- see
    /// `discredit_does_not_chain_through_its_own_speaker`.
    fn is_discredited(&self, customer: CustomerId) -> bool {
        self.heard.iter().any(|&id| match CATALOG[id].effect {
            Effect::Discredit { target } => self.cast.speaker_of(target) == customer,
            _ => false,
        })
    }

    /// Scans heard rumors newest-first, skipping any whose speaker is
    /// discredited, and returns the first `Enable`/`Void` that mentions
    /// `threat` -- i.e. the most recently spoken word on the subject wins,
    /// exactly matching CLAUDE.md §3.4's "矛盾するルールが出た場合は新しい
    /// ものが優先される". Falls back to each threat kind's baseline when
    /// nothing heard says anything about it: `Repeat` is reactable from day
    /// one (it's one of the game's two baseline anomaly families, per §4),
    /// everything else stays impossible until some rumor `Enable`s it.
    pub fn verdict(&self, pane: Pane, threat: ThreatKind, ctx: Context) -> Verdict {
        for &id in self.heard.iter().rev() {
            let def = &CATALOG[id];
            if self.is_discredited(self.cast.speaker_of(id)) {
                continue;
            }
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
            _ => Verdict::Suppressed,
        }
    }

    /// The extra corruption a correct catch of `threat` refunds right now,
    /// on top of the flat -1.0 every correct `ShouldReact` catch already
    /// earns (`log_line::resolve`) -- 0.0 if no live, non-discredited
    /// `Effect::Relieve` currently names `threat`. Same newest-wins scan as
    /// `verdict`, kept as its own query rather than folded into `Verdict`
    /// because a charm and a threat's react/suppress state are independent
    /// facts: a `Relieve` doesn't imply `Active`, and doesn't need to.
    pub fn relief_bonus(&self, threat: ThreatKind) -> f32 {
        for &id in self.heard.iter().rev() {
            let def = &CATALOG[id];
            if self.is_discredited(self.cast.speaker_of(id)) {
                continue;
            }
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
            ledger.verdict(Pane::Outside, ThreatKind::BackDoor, CTX_NO_THREE),
            Verdict::Suppressed
        );
        let enable_back_door =
            find(|e| matches!(e, Effect::Enable { threat: ThreatKind::BackDoor, .. }));
        let ledger = hear_all(ledger, &[enable_back_door]);
        assert_eq!(
            ledger.verdict(Pane::Outside, ThreatKind::BackDoor, CTX_NO_THREE),
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
    fn verdict_ignores_unrelated_rumors_interspersed_in_the_ledger() {
        // The scan skips right past rumors with no opinion on this
        // (pane, threat) pair, regardless of where they sit in the timeline.
        let enable_back_door =
            find(|e| matches!(e, Effect::Enable { threat: ThreatKind::BackDoor, .. }));
        let unrelated = find(|e| matches!(e, Effect::None));
        let ledger = hear_all(RuleLedger::new(), &[unrelated, enable_back_door, unrelated]);
        assert_eq!(
            ledger.verdict(Pane::Outside, ThreatKind::BackDoor, CTX_NO_THREE),
            Verdict::Active
        );
    }

    #[test]
    fn discrediting_a_speaker_reverts_their_rumor_to_never_having_been_heard() {
        let enable_back_door =
            find(|e| matches!(e, Effect::Enable { threat: ThreatKind::BackDoor, .. }));
        let discredit_back_door =
            find(|e| matches!(e, Effect::Discredit { target } if *target == enable_back_door));

        // Heard *before* the discredit: still reverts, because verdicts are
        // computed live against the whole ledger, not frozen at hear-time.
        let ledger = hear_all(RuleLedger::new(), &[enable_back_door, discredit_back_door]);
        assert_eq!(
            ledger.verdict(Pane::Outside, ThreatKind::BackDoor, CTX_NO_THREE),
            Verdict::Suppressed
        );
    }

    #[test]
    fn discredit_does_not_chain_through_its_own_speaker() {
        // If the discrediting rumor's own speaker later gets discredited by
        // someone else, the original discredit still stands (v1 doesn't
        // recursively unwind trust chains) -- pinned here so any future
        // change to that behavior is a deliberate decision, not a regression.
        let enable_back_door =
            find(|e| matches!(e, Effect::Enable { threat: ThreatKind::BackDoor, .. }));
        let discredit_back_door =
            find(|e| matches!(e, Effect::Discredit { target } if *target == enable_back_door));

        let ledger = RuleLedger::new();
        let back_door_speaker = ledger.speaker_of(enable_back_door);
        let discreditor = ledger.speaker_of(discredit_back_door);

        let ledger = hear_all(ledger, &[enable_back_door, discredit_back_door]);
        assert!(ledger.is_discredited(back_door_speaker));
        // No entry discredits `discreditor` in turn, so nothing chains.
        assert!(!ledger.is_discredited(discreditor));
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
        assert_eq!(ledger.relief_bonus(ThreatKind::BackDoor), 0.0);
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
    fn cast_never_assigns_a_discredit_speaker_equal_to_their_target() {
        for _ in 0..40 {
            let ledger = RuleLedger::new();
            for (id, def) in CATALOG.iter().enumerate() {
                if let Effect::Discredit { target } = def.effect {
                    assert_ne!(ledger.speaker_of(id), ledger.speaker_of(target));
                }
            }
        }
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
        let enable_back_door =
            find(|e| matches!(e, Effect::Enable { threat: ThreatKind::BackDoor, .. }));
        let mut ledger = hear_all(RuleLedger::new(), &[enable_back_door]);
        let speaker_before = ledger.speaker_of(enable_back_door);
        assert_eq!(
            ledger.verdict(Pane::Outside, ThreatKind::BackDoor, CTX_NO_THREE),
            Verdict::Active
        );

        ledger.reset_day();

        // The effect no longer applies -- it must be heard again today.
        assert_eq!(
            ledger.verdict(Pane::Outside, ThreatKind::BackDoor, CTX_NO_THREE),
            Verdict::Suppressed
        );
        // But who *would* voice it, if re-heard, hasn't been rerolled.
        assert_eq!(ledger.speaker_of(enable_back_door), speaker_before);
    }

    #[test]
    fn reset_day_also_lifts_a_discredit() {
        let enable_back_door =
            find(|e| matches!(e, Effect::Enable { threat: ThreatKind::BackDoor, .. }));
        let discredit_back_door =
            find(|e| matches!(e, Effect::Discredit { target } if *target == enable_back_door));
        let mut ledger = hear_all(RuleLedger::new(), &[enable_back_door, discredit_back_door]);
        let speaker = ledger.speaker_of(enable_back_door);
        assert!(ledger.is_discredited(speaker));

        ledger.reset_day();

        // A discredited reputation doesn't survive the reset either --
        // everything heard yesterday is equally gone today (CLAUDE.md §3.4).
        assert!(!ledger.is_discredited(speaker));
    }
}
