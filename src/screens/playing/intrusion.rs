use bevy::prelude::*;

use super::spawn::apply_outcome;
use crate::domain::{Classification, Verb, resolve};
use crate::game_data::GameData;

/// How long 呼ばれる stays on screen before it resolves on its own -- long
/// enough to read, short enough that waiting it out isn't a real choice
/// (there is nothing to decide; see `resolve_intrusion`).
const INTRUSION_LIFETIME_SECONDS: f32 = 6.0;

/// The single always-present slot for 呼ばれる (CLAUDE.md §7), spawned once
/// in `super::setup::setup`, outside all three panes' `Node` trees -- not a
/// fourth pane, not anyone's `Pending`. That placement is what makes the
/// line structurally unreachable by any cursor, rather than merely
/// discouraged: there is no `Pending` for it to ever be marked in.
#[derive(Component)]
pub(super) struct IntrusionSlot;

/// `Some` for the `INTRUSION_LIFETIME_SECONDS` the line is showing; `None`
/// otherwise. Absence doubles as "nothing to resolve" for both
/// `spawn_intrusion` (don't re-trigger) and `resolve_intrusion` (nothing to
/// tick).
#[derive(Resource, Default)]
pub(super) struct ActiveIntrusion(Option<Timer>);

/// Picks up `GameData::pending_intrusion` the moment `GameData` sets it
/// (see `GameData::maybe_queue_name_call`) and starts its lifetime clock.
pub(super) fn spawn_intrusion(
    mut game_data: ResMut<GameData>,
    mut active: ResMut<ActiveIntrusion>,
    mut slot: Query<&mut Text, With<IntrusionSlot>>,
) {
    let Some(line) = game_data.pending_intrusion.take() else {
        return;
    };
    let Ok(mut text) = slot.single_mut() else {
        return;
    };
    text.0 = line.text;
    active.0 = Some(Timer::from_seconds(INTRUSION_LIFETIME_SECONDS, TimerMode::Once));
}

/// While an intrusion is showing, any 削除/検印 press anywhere -- regardless
/// of which pane is active, since this line isn't in any pane -- counts as
/// answering it. Reuses `domain::resolve`'s `ShouldNotReact` table exactly
/// like a real line would: silence for the full lifetime is free, touching
/// it is not (CLAUDE.md §7: "削除も検印も「返事」である").
pub(super) fn resolve_intrusion(
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut game_data: ResMut<GameData>,
    mut active: ResMut<ActiveIntrusion>,
    mut slot: Query<&mut Text, With<IntrusionSlot>>,
) {
    let Some(timer) = active.0.as_mut() else {
        return;
    };
    timer.tick(time.delta());

    let action = if keyboard.just_pressed(KeyCode::KeyZ) {
        Some(Verb::Delete)
    } else if keyboard.just_pressed(KeyCode::KeyX) {
        Some(Verb::Stamp)
    } else {
        None
    };

    if action.is_none() && !timer.is_finished() {
        return;
    }

    apply_outcome(&mut game_data, resolve(Classification::ShouldNotReact, action, 0.0));
    active.0 = None;
    if let Ok(mut text) = slot.single_mut() {
        text.0 = String::new();
    }
}
