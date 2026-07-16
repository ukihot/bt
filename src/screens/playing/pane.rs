use std::collections::VecDeque;

use bevy::prelude::*;

use crate::domain::{LogLine, Pane, Phase};

/// Marks a pane's root entity with which of the three simultaneous feeds it
/// is, and carries the state that's local to that one pane: its own spawn
/// clock, its own memory of its last normal line (needed for 反復), and its
/// own queue of scripted lines (day markers, mistake beats) that must
/// appear on this pane specifically rather than wherever happened to spawn
/// next. Before the screen split into three feeds (第3.1節) this all lived
/// as a handful of fields on the single global `GameData`.
#[derive(Component)]
pub(super) struct PaneRuntime {
    pub(super) pane: Pane,
    pub(super) spawn_timer: Timer,
    pub(super) last_normal_line: Option<String>,
    pub(super) pending_scripted: VecDeque<LogLine>,
}

impl PaneRuntime {
    pub(super) fn new(pane: Pane, phase: Phase) -> Self {
        Self {
            pane,
            spawn_timer: Timer::from_seconds(pane.spawn_interval_secs(phase), TimerMode::Repeating),
            last_normal_line: None,
            pending_scripted: VecDeque::new(),
        }
    }

    /// Re-derives this pane's spawn cadence for a new `Phase`. Each pane
    /// keeps its own multiplier over the shared phase tempo (see
    /// `Pane::spawn_interval_secs`), so this can't just reuse one timer
    /// reset shared across all three.
    pub(super) fn retime(&mut self, phase: Phase) {
        self.spawn_timer =
            Timer::from_seconds(self.pane.spawn_interval_secs(phase), TimerMode::Repeating);
    }
}

/// Which pane `J`/`K`/削除/検印 currently apply to. Switched with `H`
/// (toward 外) / `L` (toward 焼成室) -- see `super::input::handle_pane_switch`.
/// The other two panes keep scrolling and resolving while unselected;
/// nothing pauses just because it's off-focus.
#[derive(Resource)]
pub(super) struct ActivePane(pub(super) Pane);

impl Default for ActivePane {
    fn default() -> Self {
        Self(Pane::Kiln)
    }
}
