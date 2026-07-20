//! The Playing screen: three simultaneous camera-feed panes (第3.1節) plus
//! the screen furniture around them. Split by concern rather than kept as
//! one file -- `pane` owns each feed's identity and per-pane runtime state,
//! `pending` owns the cursor/eviction window (the part most worth
//! unit-testing in isolation), `render` turns state into text, `spawn`
//! drives new lines in from `GameData` across all three panes at once,
//! `input`/`pause` handle keyboard input (including which pane is active),
//! `intrusion` is 呼ばれる's own unreachable slot, `glitch` is the purely
//! cosmetic CRT flicker, `day_indicator` is the 7-segment-style day counter
//! next to `Kiln`, and `corruption` is the loss condition. `setup` ties the
//! screen's entities and resources together for `OnEnter`/`OnExit`.

mod corruption;
mod day_indicator;
mod glitch;
mod input;
mod intrusion;
mod pane;
mod pause;
mod pending;
mod render;
mod setup;
mod spawn;

use bevy::prelude::*;

use crate::app_state::AppState;

pub struct PlayingPlugin;

impl Plugin for PlayingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Playing), setup::setup)
            .add_systems(Update, pause::handle_pause_input.run_if(in_state(AppState::Playing)))
            .add_systems(
                Update,
                (
                    spawn::phase_tick,
                    spawn::line_spawn,
                    intrusion::spawn_intrusion,
                    input::handle_pane_switch,
                    input::handle_line_input,
                    input::animate_delete_wipe,
                    intrusion::resolve_intrusion,
                    corruption::corruption_check,
                    render::sync_log_display,
                    render::sync_pane_headers,
                    day_indicator::sync_day_indicator,
                    glitch::glitch_flicker,
                )
                    .chain()
                    .run_if(in_state(AppState::Playing).and_then(pause::not_paused)),
            )
            .add_systems(OnExit(AppState::Playing), setup::teardown);
    }
}
