use super::phase::Phase;
use super::zone::Zone;

const GOLDEN_RATIO: f32 = 1.618_034;

/// One of the three simultaneous camera feeds the player watches from the
/// counter. Each has its own register (see `domain::generate`), its own
/// pacing, and its own "primary" anomaly -- but per CLAUDE.md §3.3, no
/// classification is exclusive to any one pane; `weights` only skews the
/// odds.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Pane {
    /// 焼成室. The baker's own record -- technical, terse. Home of the
    /// 表記の乱れ・数え違い・記載漏れ deviations and the 禁忌集 memos.
    Kiln,
    /// 外. A deadpan fixed-point watch, mostly "異常なし". Home of 呼びかけ.
    Outside,
    /// 売り場. The clerk's customer-facing log, in です/ます. Home of 反復.
    Floor,
}

/// Cycle order for `H`/`L`, matching CLAUDE.md's distance principle
/// (外 → 売り場 → 焼成室): `L` always steps toward the pane currently
/// favored by `Zone`'s escalation, `H` steps back out toward the perimeter.
pub const ORDER: [Pane; 3] = [Pane::Outside, Pane::Floor, Pane::Kiln];

impl Pane {
    pub fn label(self) -> &'static str {
        match self {
            Pane::Kiln => "焼成室",
            Pane::Outside => "外",
            Pane::Floor => "売り場",
        }
    }

    pub fn next_in_order(self) -> Self {
        let i = ORDER.iter().position(|&p| p == self).unwrap();
        ORDER[(i + 1) % ORDER.len()]
    }

    pub fn prev_in_order(self) -> Self {
        let i = ORDER.iter().position(|&p| p == self).unwrap();
        ORDER[(i + ORDER.len() - 1) % ORDER.len()]
    }

    /// How many lines this pane's cursor window holds at once. `Kiln` is
    /// the full-width main monitor; `Outside`/`Floor` share the screen's
    /// remaining space, so they hold fewer -- in golden-ratio proportion to
    /// `Kiln` (8:5, the nearest whole-line approximation of φ), so the
    /// visual weight of "the pane you're mainly watching" versus "the two
    /// you're not" reads the same whether you count screen area or line count.
    pub fn capacity(self) -> usize {
        const KILN_CAPACITY: usize = 8;
        match self {
            Pane::Kiln => KILN_CAPACITY,
            Pane::Outside | Pane::Floor => (KILN_CAPACITY as f32 / GOLDEN_RATIO).round() as usize,
        }
    }

    /// This pane's own pacing, layered on top of the shared `Phase` tempo --
    /// `外` is sparse and quiet, `売り場` gets busiest at `Phase::Peak`.
    pub fn spawn_interval_secs(self, phase: Phase) -> f32 {
        let base = phase.spawn_interval_secs();
        let multiplier = match self {
            Pane::Kiln => 1.0,
            Pane::Outside => 2.4,
            Pane::Floor => 0.85,
        };
        base * multiplier
    }

    /// Whether `Zone`'s current escalation stage is "pointed at" this pane,
    /// per CLAUDE.md's distance principle (外→売り場→焼成室).
    pub fn matches_zone(self, zone: Zone) -> bool {
        matches!(
            (self, zone),
            (Pane::Outside, Zone::Perimeter)
                | (Pane::Floor, Zone::Inside)
                | (Pane::Kiln, Zone::Counter)
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn next_and_prev_in_order_cycle_through_all_three() {
        for &pane in &ORDER {
            assert_eq!(pane.next_in_order().prev_in_order(), pane);
            assert_eq!(pane.prev_in_order().next_in_order(), pane);
        }
        // L from the outermost pane steps inward, toward Kiln.
        assert_eq!(Pane::Outside.next_in_order(), Pane::Floor);
        assert_eq!(Pane::Floor.next_in_order(), Pane::Kiln);
        // ...and wraps back out past Kiln.
        assert_eq!(Pane::Kiln.next_in_order(), Pane::Outside);
    }

    #[test]
    fn matches_zone_follows_the_distance_principle() {
        assert!(Pane::Outside.matches_zone(Zone::Perimeter));
        assert!(Pane::Floor.matches_zone(Zone::Inside));
        assert!(Pane::Kiln.matches_zone(Zone::Counter));
        // No pane matches a zone that isn't its own.
        assert!(!Pane::Outside.matches_zone(Zone::Inside));
        assert!(!Pane::Kiln.matches_zone(Zone::Perimeter));
    }

    #[test]
    fn outside_is_the_quietest_pane_and_floor_the_busiest() {
        let phase = Phase::Peak;
        assert!(Pane::Outside.spawn_interval_secs(phase) > Pane::Kiln.spawn_interval_secs(phase));
        assert!(Pane::Floor.spawn_interval_secs(phase) < Pane::Kiln.spawn_interval_secs(phase));
    }
}
