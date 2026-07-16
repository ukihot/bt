use super::verb::Verb;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Classification {
    Normal,
    ShouldReact,
    ShouldNotReact,
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
    pub correct_verb: Option<Verb>,
    pub scripted: bool,
    pub font: LineFont,
}

impl LogLine {
    pub(super) fn new(
        text: String,
        classification: Classification,
        correct_verb: Option<Verb>,
    ) -> Self {
        Self { text, classification, correct_verb, scripted: false, font: LineFont::default() }
    }

    pub(super) fn scripted(mut self) -> Self {
        self.scripted = true;
        self
    }

    pub(super) fn with_font(mut self, font: LineFont) -> Self {
        self.font = font;
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
pub fn resolve(
    classification: Classification,
    correct_verb: Option<Verb>,
    action: Option<Verb>,
) -> Outcome {
    match classification {
        Classification::Normal => match action {
            None => Outcome { corruption: 0.0, income: 0, zone_bump: false },
            Some(_) => Outcome { corruption: 0.0, income: -2, zone_bump: false },
        },
        Classification::ShouldReact => {
            if action == correct_verb {
                Outcome { corruption: -1.0, income: 3, zone_bump: false }
            } else if action.is_none() {
                Outcome { corruption: 4.0, income: -1, zone_bump: false }
            } else {
                Outcome { corruption: 2.0, income: 0, zone_bump: false }
            }
        }
        Classification::ShouldNotReact => match action {
            None => Outcome { corruption: 0.0, income: 0, zone_bump: false },
            Some(_) => Outcome { corruption: 6.0, income: -2, zone_bump: true },
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normal_untouched_is_free() {
        let o = resolve(Classification::Normal, None, None);
        assert_eq!((o.corruption, o.income, o.zone_bump), (0.0, 0, false));
    }

    #[test]
    fn normal_action_costs_income_only() {
        let o = resolve(Classification::Normal, None, Some(Verb::Delete));
        assert_eq!((o.corruption, o.zone_bump), (0.0, false));
        assert!(o.income < 0);
    }

    #[test]
    fn should_react_correct_verb_rewards() {
        let o = resolve(Classification::ShouldReact, Some(Verb::Delete), Some(Verb::Delete));
        assert!(o.corruption < 0.0);
        assert!(o.income > 0);
    }

    #[test]
    fn should_react_untouched_is_worse_than_caught() {
        let untouched = resolve(Classification::ShouldReact, Some(Verb::Delete), None);
        let caught = resolve(Classification::ShouldReact, Some(Verb::Delete), Some(Verb::Delete));
        assert!(untouched.corruption > caught.corruption);
        assert!(untouched.income < caught.income);
    }

    #[test]
    fn should_react_acting_on_the_wrong_target_still_costs_something() {
        // `resolve` doesn't know the catalog currently only has one verb --
        // exercise the "acted, but not on what was actually required" branch
        // directly so it stays covered even though no real ShouldReact line
        // can produce this combination today (it becomes reachable again the
        // moment a second `Verb` variant returns).
        let o = resolve(Classification::ShouldReact, None, Some(Verb::Delete));
        assert_eq!((o.corruption, o.income, o.zone_bump), (2.0, 0, false));
    }

    #[test]
    fn should_not_react_untouched_is_free() {
        let o = resolve(Classification::ShouldNotReact, None, None);
        assert_eq!((o.corruption, o.income, o.zone_bump), (0.0, 0, false));
    }

    #[test]
    fn should_not_react_any_touch_brings_it_closer() {
        let o = resolve(Classification::ShouldNotReact, None, Some(Verb::Delete));
        assert!(o.corruption > 0.0);
        assert!(o.zone_bump);
    }
}
