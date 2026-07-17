/// 24 in-game hours flow in 4 real minutes (CLAUDE.md §3.7): a fixed rate,
/// not a per-line dice roll, so every line's timestamp is a read of one
/// monotonically advancing "now" shared by all three panes, instead of an
/// independent random draw that can make time appear to jump or rewind.
pub const GAME_MINUTES_PER_REAL_SECOND: f32 = 6.0;
pub const MINUTES_PER_DAY: u32 = 24 * 60;

/// Minutes elapsed since this in-game day's midnight. The only clock in the
/// game -- every pane's displayed hour/minute is a read of this value
/// (through its own writer's rounding, see `timestamp::even_minute_of`),
/// never an independent roll.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DayClock {
    minutes: f32,
}

impl DayClock {
    /// Test-only constructor for an exact hour/minute -- production code
    /// only ever advances a clock starting from `default()`.
    #[cfg(test)]
    pub(crate) fn at(hour: u32, minute: u32) -> Self {
        Self { minutes: (hour * 60 + minute) as f32 }
    }

    /// Advances by `real_seconds` and returns how many midnights it crossed
    /// (almost always 0, occasionally 1; more only if a frame stalls across
    /// several in-fiction days, which callers handle by looping that many
    /// times rather than assuming at most one).
    pub fn advance(&mut self, real_seconds: f32) -> u32 {
        let total = self.minutes + real_seconds * GAME_MINUTES_PER_REAL_SECOND;
        let wraps = (total / MINUTES_PER_DAY as f32).floor().max(0.0) as u32;
        self.minutes = total.rem_euclid(MINUTES_PER_DAY as f32);
        wraps
    }

    pub fn hour(self) -> u32 {
        (self.minutes as u32 / 60) % 24
    }

    pub fn minute(self) -> u32 {
        self.minutes as u32 % 60
    }

    pub fn minute_is_odd(self) -> bool {
        self.minute() % 2 == 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn never_rewinds_within_a_day() {
        let mut clock = DayClock::default();
        let mut last_total_minutes = 0u32;
        for _ in 0..2000 {
            let wraps = clock.advance(0.05);
            let now_total_minutes = clock.hour() * 60 + clock.minute();
            if wraps == 0 {
                assert!(
                    now_total_minutes >= last_total_minutes,
                    "time must never rewind within a day"
                );
            }
            last_total_minutes = now_total_minutes;
        }
    }

    #[test]
    fn twenty_four_hours_take_exactly_four_real_minutes() {
        let mut clock = DayClock::default();
        assert_eq!(clock.advance(239.0), 0);
        assert_eq!(clock.advance(1.0), 1);
        assert_eq!((clock.hour(), clock.minute()), (0, 0));
    }

    #[test]
    fn at_constructs_the_expected_hour_and_minute() {
        let clock = DayClock::at(13, 45);
        assert_eq!((clock.hour(), clock.minute()), (13, 45));
    }
}
