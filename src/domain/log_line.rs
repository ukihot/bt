use super::verb::Verb;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Classification {
    Normal,
    ShouldReact,
    ShouldNotReact,
}

impl Classification {
    /// The one and only correct verb for this classification -- 検印
    /// (approve) for ordinary business, 削除 for a threat, and `None`
    /// (静観) for a threat that must not be touched. There is no longer a
    /// separate per-line `correct_verb`/`taboo` pair: which action is right
    /// is a pure function of the classification alone, and every other
    /// action -- including plain inaction on a `Normal`/`ShouldReact` line --
    /// is タタリ (`resolve`, below). Rumors that move a phenomenon between
    /// classifications (`rules::RuleLedger`) automatically change what
    /// counts as correct for it, without needing their own taboo bookkeeping.
    pub fn correct_action(self) -> Option<Verb> {
        match self {
            Classification::Normal => Some(Verb::Stamp),
            Classification::ShouldReact => Some(Verb::Delete),
            Classification::ShouldNotReact => None,
        }
    }
}

/// Which typeface a line renders in. Reserved for the scripted beats in
/// section 7 of CLAUDE.md -- every procedurally generated line (normal,
/// threat, or 呼びかけ) stays `Normal` so that anomalies are only ever
/// caught by their 筆癖, never by an out-of-band visual cue.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum LineFont {
    #[default]
    Normal,
    /// 二人称の障り (`あなたは 行を ◯◯した`)
    Mistake,
    /// 呼ばれる / Lost 画面の侵食ログ
    Call,
}

#[derive(Clone)]
pub struct LogLine {
    pub text: String,
    pub classification: Classification,
    pub scripted: bool,
    pub font: LineFont,
    /// Extra corruption a *correct* catch of this line refunds, on top of
    /// the flat -1.0 every correct `ShouldReact` catch already earns --
    /// set by a 援助系の禁忌 (`rumors::Effect::Relieve`) once its rumor has
    /// been heard. Deliberately only ever moves corruption, never `Zone`:
    /// 原則2(距離だけが縮む)is about the threat's physical proximity, which
    /// a folk charm doesn't touch.
    pub relief: f32,
}

impl LogLine {
    pub(super) fn new(text: String, classification: Classification) -> Self {
        Self { text, classification, scripted: false, font: LineFont::default(), relief: 0.0 }
    }

    pub(super) fn scripted(mut self) -> Self {
        self.scripted = true;
        self
    }

    pub(super) fn with_font(mut self, font: LineFont) -> Self {
        self.font = font;
        self
    }

    pub(super) fn relieved(mut self, bonus: f32) -> Self {
        self.relief = bonus;
        self
    }
}

pub struct Outcome {
    pub corruption: f32,
    pub income: i64,
    pub zone_bump: bool,
}

/// Resolve the consequence of applying `action` to a line of the given
/// classification, where `None` means the line was left untouched -- either
/// deliberately (静観), or because it aged out of the cursor's pending
/// window before the player got to it. Both cases resolve identically.
///
/// Every classification has exactly one correct action
/// (`Classification::correct_action`): 検印 for `Normal`, 削除 for
/// `ShouldReact`, 静観 for `ShouldNotReact`. Matching it is always a plain
/// reward; anything else -- wrong verb *or* inaction where an action was
/// required -- is uniformly タタリ: corruption plus `zone_bump`. There is no
/// longer a softer "just costs corruption" miss versus a harsher
/// taboo-flagged one; a rumor that moves a phenomenon between
/// classifications (`rules::RuleLedger`) changes what's correct for it, and
/// that's the only lever -- see CLAUDE.md §3.3/§5.
///
/// `relief` (`LogLine::relief`) is a 援助系の禁忌's bonus (`rumors::Effect::
/// Relieve`): additional corruption refunded on top of the flat -1.0 a
/// correct `ShouldReact` catch already earns. It only ever sweetens a
/// correct catch -- it does nothing on a miss, and never touches
/// `zone_bump`.
pub fn resolve(classification: Classification, action: Option<Verb>, relief: f32) -> Outcome {
    if action == classification.correct_action() {
        match classification {
            Classification::Normal => Outcome { corruption: 0.0, income: 1, zone_bump: false },
            Classification::ShouldReact => {
                Outcome { corruption: -1.0 - relief, income: 3, zone_bump: false }
            }
            Classification::ShouldNotReact => {
                Outcome { corruption: 0.0, income: 0, zone_bump: false }
            }
        }
    } else {
        // タタリ: whichever classification this line belongs to right now,
        // missing its one correct action always costs corruption and always
        // brings the threat a step closer (原則2) -- there is no exemption
        // for "just an ordinary line" or "just a baseline anomaly" anymore.
        match classification {
            Classification::Normal => Outcome { corruption: 2.0, income: -1, zone_bump: true },
            Classification::ShouldReact => Outcome { corruption: 4.0, income: -1, zone_bump: true },
            Classification::ShouldNotReact => {
                Outcome { corruption: 6.0, income: -2, zone_bump: true }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normal_correct_action_is_stamp_and_is_rewarded() {
        let o = resolve(Classification::Normal, Some(Verb::Stamp), 0.0);
        assert_eq!((o.corruption, o.zone_bump), (0.0, false));
        assert!(o.income > 0);
    }

    #[test]
    fn normal_left_untouched_is_tatari() {
        let o = resolve(Classification::Normal, None, 0.0);
        assert!(o.corruption > 0.0);
        assert!(o.zone_bump);
    }

    #[test]
    fn normal_deleted_instead_of_stamped_is_tatari() {
        let o = resolve(Classification::Normal, Some(Verb::Delete), 0.0);
        assert!(o.corruption > 0.0);
        assert!(o.zone_bump);
    }

    #[test]
    fn should_react_correct_verb_rewards() {
        let o = resolve(Classification::ShouldReact, Some(Verb::Delete), 0.0);
        assert!(o.corruption < 0.0);
        assert!(o.income > 0);
        assert!(!o.zone_bump);
    }

    #[test]
    fn should_react_untouched_is_tatari() {
        let untouched = resolve(Classification::ShouldReact, None, 0.0);
        let caught = resolve(Classification::ShouldReact, Some(Verb::Delete), 0.0);
        assert!(untouched.corruption > caught.corruption);
        assert!(untouched.income < caught.income);
        assert!(untouched.zone_bump);
    }

    #[test]
    fn should_react_stamped_instead_of_deleted_is_also_tatari() {
        // Every wrong action is タタリ uniformly now -- wrong verb is no
        // longer a milder outcome than plain inaction.
        let o = resolve(Classification::ShouldReact, Some(Verb::Stamp), 0.0);
        assert!(o.corruption > 0.0);
        assert!(o.zone_bump);
    }

    #[test]
    fn should_react_correct_catch_refunds_extra_corruption_when_relieved() {
        // 援助系の禁忌(Effect::Relieve): a correct catch heals more than the
        // flat baseline once its charm has been heard.
        let baseline = resolve(Classification::ShouldReact, Some(Verb::Delete), 0.0);
        let relieved = resolve(Classification::ShouldReact, Some(Verb::Delete), 3.0);
        assert!(relieved.corruption < baseline.corruption);
        assert!(!relieved.zone_bump);
    }

    #[test]
    fn relief_does_nothing_on_a_miss() {
        // The charm rewards catching it correctly -- it's not a flat discount
        // that also softens a miss.
        let missed = resolve(Classification::ShouldReact, None, 3.0);
        assert_eq!(missed.corruption, 4.0);
    }

    #[test]
    fn should_not_react_untouched_is_free() {
        let o = resolve(Classification::ShouldNotReact, None, 0.0);
        assert_eq!((o.corruption, o.income, o.zone_bump), (0.0, 0, false));
    }

    #[test]
    fn should_not_react_any_touch_brings_it_closer() {
        let stamped = resolve(Classification::ShouldNotReact, Some(Verb::Stamp), 0.0);
        assert!(stamped.corruption > 0.0);
        assert!(stamped.zone_bump);
        let deleted = resolve(Classification::ShouldNotReact, Some(Verb::Delete), 0.0);
        assert!(deleted.corruption > 0.0);
        assert!(deleted.zone_bump);
    }
}
