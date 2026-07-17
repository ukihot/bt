use bevy::prelude::*;

use crate::domain::{self, DayClock, LogLine, Phase, Zone};

/// Truly global run state -- the shared day clock, the shared corruption
/// meter, and the one-off scripted beats. Everything that belongs to a
/// single pane instead (its own spawn timer, its own last normal line, its
/// own queue of scripted lines) lives as ECS components on that pane's
/// entity -- see `screens::playing::pane::PaneRuntime` -- rather than here.
#[derive(Resource)]
pub struct GameData {
    pub day: u32,
    /// Cached `Phase::for_hour(clock.hour())` -- re-derived every tick
    /// (`screens::playing::spawn::phase_tick`), never advanced on its own
    /// timer, so it can never drift out of sync with `clock` (CLAUDE.md §3.7).
    pub phase: Phase,
    pub clock: DayClock,
    pub zone: Zone,
    pub corruption: f32,
    pub income: i64,
    pub player_name: String,
    pub first_mistake_done: bool,
    pub name_call_done: bool,
    /// 呼ばれる: set once, late in a run. Deliberately not queued into any
    /// pane's own scripted list -- unlike every other line in the game, it
    /// must never be reachable by a cursor at all (see
    /// `screens::playing::intrusion`).
    pub pending_intrusion: Option<LogLine>,
}

impl Default for GameData {
    fn default() -> Self {
        Self::fresh()
    }
}

impl GameData {
    pub fn fresh() -> Self {
        Self {
            day: 1,
            phase: Phase::Prep,
            clock: DayClock::default(),
            zone: Zone::Perimeter,
            corruption: 0.0,
            income: 0,
            player_name: String::new(),
            first_mistake_done: false,
            name_call_done: false,
            pending_intrusion: None,
        }
    }

    /// 呼ばれる only ever fires once per run, at whichever moment Night and
    /// `Zone::Counter` first coincide -- so "now" (the shared clock) is
    /// already the right timestamp; it also carries 呼びかけ's odd-minute
    /// tell (第4節), so this waits for an odd minute the same way
    /// `generate::call_line` does, rather than fabricating one.
    pub fn maybe_queue_name_call(&mut self) {
        if self.name_call_done
            || self.zone != Zone::Counter
            || self.phase != Phase::Night
            || !self.clock.minute_is_odd()
        {
            return;
        }
        self.name_call_done = true;
        let loc = self.zone.location_pool()[0];
        self.pending_intrusion =
            Some(domain::name_call(self.clock.hour(), self.clock.minute(), &self.player_name, loc));
    }
}
