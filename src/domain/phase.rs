#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Phase {
    Prep,
    Morning,
    Peak,
    Evening,
    Night,
}

/// Order `for_hour` scans in. `hour_range` below must tile all 24 hours
/// across this order with no gaps or overlaps -- see
/// `hour_range_tiles_the_full_day_without_gaps_or_overlap`.
const ORDER: [Phase; 5] = [Phase::Prep, Phase::Morning, Phase::Peak, Phase::Evening, Phase::Night];

impl Phase {
    /// This phase's `[start, end)` span on the shared clock (`domain::clock`).
    /// Contiguous across all five phases -- every hour of the day belongs to
    /// exactly one, so the active phase is always a pure function of the
    /// clock's current hour (CLAUDE.md §3.7), never its own independent timer.
    pub fn hour_range(self) -> (u32, u32) {
        match self {
            Phase::Prep => (0, 7),
            Phase::Morning => (7, 11),
            Phase::Peak => (11, 14),
            Phase::Evening => (14, 19),
            Phase::Night => (19, 24),
        }
    }

    /// Which phase owns a given clock hour. The single source of truth for
    /// phase transitions -- callers re-derive this every tick from
    /// `DayClock::hour` rather than caching a phase that free-runs on its own
    /// timer and can drift out of sync with the displayed clock.
    pub fn for_hour(hour: u32) -> Self {
        ORDER
            .into_iter()
            .find(|p| {
                let (lo, hi) = p.hour_range();
                (lo..hi).contains(&hour)
            })
            .expect("hour_range partitions all 24 hours")
    }

    pub fn spawn_interval_secs(self) -> f32 {
        match self {
            Phase::Prep => 3.2,
            Phase::Morning => 2.2,
            Phase::Peak => 1.1,
            Phase::Evening => 1.8,
            Phase::Night => 2.6,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hour_range_tiles_the_full_day_without_gaps_or_overlap() {
        for hour in 0..24 {
            let phase = Phase::for_hour(hour);
            let (lo, hi) = phase.hour_range();
            assert!((lo..hi).contains(&hour), "hour {hour} not covered by its own phase's range");
        }
    }

    #[test]
    fn for_hour_matches_expected_boundaries() {
        assert_eq!(Phase::for_hour(0), Phase::Prep);
        assert_eq!(Phase::for_hour(6), Phase::Prep);
        assert_eq!(Phase::for_hour(7), Phase::Morning);
        assert_eq!(Phase::for_hour(10), Phase::Morning);
        assert_eq!(Phase::for_hour(11), Phase::Peak);
        assert_eq!(Phase::for_hour(13), Phase::Peak);
        assert_eq!(Phase::for_hour(14), Phase::Evening);
        assert_eq!(Phase::for_hour(18), Phase::Evening);
        assert_eq!(Phase::for_hour(19), Phase::Night);
        assert_eq!(Phase::for_hour(23), Phase::Night);
    }
}
