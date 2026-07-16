use bevy::prelude::*;
use rand::RngExt;

use crate::domain::{self, LogLine, Phase, Zone};

/// Truly global run state -- the day/phase clock, the shared corruption
/// meter, and the one-off scripted beats. Everything that belongs to a
/// single pane instead (its own spawn timer, its own last normal line, its
/// own queue of scripted lines) lives as ECS components on that pane's
/// entity -- see `screens::playing::pane::PaneRuntime` -- rather than here.
#[derive(Resource)]
pub struct GameData {
    pub day: u32,
    pub phase: Phase,
    pub phase_timer: Timer,
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
            phase_timer: Timer::from_seconds(Phase::Prep.duration_secs(), TimerMode::Once),
            zone: Zone::Perimeter,
            corruption: 0.0,
            income: 0,
            player_name: String::new(),
            first_mistake_done: false,
            name_call_done: false,
            pending_intrusion: None,
        }
    }

    pub fn maybe_queue_name_call(&mut self, rng: &mut impl rand::Rng) {
        if self.name_call_done || self.zone != Zone::Counter || self.phase != Phase::Night {
            return;
        }
        self.name_call_done = true;
        let (lo, hi) = self.phase.hour_range();
        let loc = self.zone.location_pool()[0];
        let h = rng.random_range(lo..=hi);
        let m = rng.random_range(0..30) * 2 + 1;
        self.pending_intrusion = Some(domain::name_call(h, m, &self.player_name, loc));
    }
}
