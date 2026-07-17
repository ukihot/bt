use rand::RngExt;
use rand::rngs::ThreadRng;

use super::clock::DayClock;
use super::log_line::{Classification, LogLine};
use super::timestamp::{even_minute_of, timestamp};
use super::verb::Verb;

/// Binary conditions a threat's resolution can be conditioned on, computed
/// once per generated line from the current run state. New taboo
/// interactions (one memo silently voiding another) are wired up by adding a
/// field here and reading it from `resolve_repeat` -- never by branching
/// inline at the call site.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct RuleFlags {
    /// 三のつく日 -- the day number contains the digit 3.
    day_has_three: bool,
}

impl RuleFlags {
    pub fn compute(day: u32) -> Self {
        Self { day_has_three: day.to_string().contains('3') }
    }
}

/// What the 反復 threat resolves to once its flags are known.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Resolution {
    /// Still a threat: must be caught with `Verb::Delete`.
    React,
    /// Some other active taboo has voided this one -- the line still
    /// appears, but it now reads as ordinary business (静観 is correct).
    Void,
}

/// 三のつく日は、反復を咎めても仕方ない、というのがどうやら店の作法らしい.
fn resolve_repeat(flags: RuleFlags) -> Resolution {
    if flags.day_has_three { Resolution::Void } else { Resolution::React }
}

/// 表記の乱れ: 「焼き上がり」ではなく「焼きあがり」と書く
fn deviation_spelling(clock: DayClock, rng: &mut ThreadRng) -> String {
    let h = clock.hour();
    let m = even_minute_of(clock);
    let n = rng.random_range(1..20);
    format!("{} バゲットが{}本 焼きあがり", timestamp(h, m), n)
}

/// 数え違い: バゲットを「本」ではなく「個」で数える
fn deviation_counter(clock: DayClock, rng: &mut ThreadRng) -> String {
    let h = clock.hour();
    let m = even_minute_of(clock);
    let n = rng.random_range(1..20);
    format!("{} バゲットが{}個 焼き上がり", timestamp(h, m), n)
}

/// 記載漏れ: 動詞を欠いた、書きかけの記録
fn deviation_missing_verb(clock: DayClock, rng: &mut ThreadRng) -> String {
    let h = clock.hour();
    let m = even_minute_of(clock);
    let n = rng.random_range(1..30);
    format!("{} 配達伝票を{}件", timestamp(h, m), n)
}

const DEVIATION_GENERATORS: &[fn(DayClock, &mut ThreadRng) -> String] =
    &[deviation_spelling, deviation_counter, deviation_missing_verb];

/// 焼成室固有の脅威: 店の筆癖(第4節)からの技術的な逸脱。
/// 内容だけを見れば正常な業務記録と 区別がつかず、常に `削除` が正解 --
/// ログ単独で読み取れる異常なので、履歴を必要としない (`repeat_threat_line`
/// と違って `None` を返すことはない)。
pub fn deviation_threat_line(clock: DayClock, rng: &mut ThreadRng) -> LogLine {
    let generator = DEVIATION_GENERATORS[rng.random_range(0..DEVIATION_GENERATORS.len())];
    let text = generator(clock, rng);
    LogLine::new(text, Classification::ShouldReact, Some(Verb::Delete))
}

/// 売り場固有の脅威: 直前とまったく同じ行がもう一度流れる。`last_normal`
/// がまだない (画面がこの日まだ何も書いていない)場合は `None` --
/// 呼び出し側は通常行にフォールバックする。
pub fn repeat_threat_line(
    flags: RuleFlags,
    last_normal: Option<&str>,
    rng: &mut ThreadRng,
) -> Option<LogLine> {
    let _ = rng; // reserved: a future taboo interaction may need randomness here too
    let text = last_normal?.to_string();
    Some(match resolve_repeat(flags) {
        Resolution::React => LogLine::new(text, Classification::ShouldReact, Some(Verb::Delete)),
        Resolution::Void => LogLine::new(text, Classification::Normal, None),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_has_three_matches_the_digit_not_the_multiple() {
        assert!(!RuleFlags::compute(1).day_has_three);
        assert!(RuleFlags::compute(3).day_has_three);
        assert!(RuleFlags::compute(13).day_has_three);
        assert!(RuleFlags::compute(23).day_has_three);
        assert!(RuleFlags::compute(30).day_has_three);
        // 6 is a multiple of 3 but contains no '3' -- this must stay false,
        // or the flag is silently testing the wrong thing.
        assert!(!RuleFlags::compute(6).day_has_three);
    }

    #[test]
    fn repeat_threat_voids_only_on_a_day_with_three() {
        assert_eq!(resolve_repeat(RuleFlags { day_has_three: false }), Resolution::React);
        assert_eq!(resolve_repeat(RuleFlags { day_has_three: true }), Resolution::Void);
    }

    #[test]
    fn deviation_threat_line_is_always_legible_and_always_react() {
        let mut rng = rand::rng();
        for _ in 0..200 {
            let line = deviation_threat_line(DayClock::at(11, 30), &mut rng);
            assert_eq!(line.classification, Classification::ShouldReact);
            assert_eq!(line.correct_verb, Some(Verb::Delete));
            assert!(!line.text.is_empty());
        }
    }

    #[test]
    fn repeat_threat_line_needs_history() {
        let mut rng = rand::rng();
        assert!(repeat_threat_line(RuleFlags::default(), None, &mut rng).is_none());
        let line = repeat_threat_line(RuleFlags::default(), Some("前の行"), &mut rng).unwrap();
        assert_eq!(line.text, "前の行");
        assert_eq!(line.classification, Classification::ShouldReact);
    }

    #[test]
    fn repeat_threat_line_voids_to_normal_on_a_day_with_three() {
        let mut rng = rand::rng();
        let flags = RuleFlags { day_has_three: true };
        let line = repeat_threat_line(flags, Some("前の行"), &mut rng).unwrap();
        assert_eq!(line.classification, Classification::Normal);
        assert_eq!(line.correct_verb, None);
    }
}
