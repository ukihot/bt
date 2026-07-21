use super::phase::Phase;
use super::zone::Zone;

const GOLDEN_RATIO: f32 = 1.618_034;

/// One of the three simultaneous camera feeds the player watches from the
/// counter. Each has its own register (see `domain::generate`) and its own
/// pacing. Only `Kiln` is ever operable (CLAUDE.md §3.2: 操作できるのは
/// 焼成室だけで、画面を切り替えるという概念自体がない) -- `Outside`/`Floor`
/// only ever generate `Classification::Normal` flavor (plus, for `Floor`,
/// the rumors that rewrite `Kiln`'s rules), never reactable threat content,
/// since there's no cursor on either screen for a "correct action" to mean
/// anything (see `screens::playing::spawn::line_spawn`, which never calls
/// `resolve()` for their evicted lines at all).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Pane {
    /// 焼成室. The baker's own record -- technical, terse. Home of the
    /// 表記の乱れ・数え違い・記載漏れ deviations, the 反復, and the 禁忌集
    /// memos. The only pane the player ever acts on.
    Kiln,
    /// 外. A deadpan fixed-point watch, mostly "異常なし" -- a read-only
    /// status display (第3.5節: 脅威との距離の体温計), not a site of its own
    /// corruption risk.
    Outside,
    /// 売り場. The clerk's customer-facing log, in です/ます -- a read-only
    /// rule-change source (第3.4節), not a site of its own corruption risk.
    Floor,
}

impl Pane {
    pub fn label(self) -> &'static str {
        match self {
            Pane::Kiln => "焼成室",
            Pane::Outside => "外",
            Pane::Floor => "売り場",
        }
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
