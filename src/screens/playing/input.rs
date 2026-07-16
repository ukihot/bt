use bevy::prelude::*;

use super::pane::{ActivePane, PaneRuntime};
use super::pending::Pending;
use crate::domain::Verb;

/// How long a 削除 mark takes to sweep across the line, left to right.
const DELETE_WIPE_SECONDS: f32 = 0.4;

fn verb_for_keycode(key: KeyCode) -> Option<Verb> {
    match key {
        KeyCode::KeyZ => Some(Verb::Delete),
        KeyCode::KeyX => Some(Verb::Stamp),
        _ => None,
    }
}

/// `H`/`L` switch which pane the rest of this module's input applies to,
/// cycling through `domain::pane::ORDER` (外→売り場→焼成室, per the
/// distance principle in CLAUDE.md §2). The two panes not selected keep
/// scrolling and resolving in the background -- see `super::spawn::line_spawn`.
pub(super) fn handle_pane_switch(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut active: ResMut<ActivePane>,
) {
    if keyboard.just_pressed(KeyCode::KeyH) {
        active.0 = active.0.prev_in_order();
    }
    if keyboard.just_pressed(KeyCode::KeyL) {
        active.0 = active.0.next_in_order();
    }
}

/// `J`/`K`/削除/検印 all target whichever single pane is currently active --
/// never the other two. A linear scan over three entities to find it is
/// simpler than threading a direct lookup through, and just as cheap.
pub(super) fn handle_line_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    active: Res<ActivePane>,
    mut panes: Query<(&PaneRuntime, &mut Pending)>,
) {
    let Some((_, mut pending)) = panes.iter_mut().find(|(runtime, _)| runtime.pane == active.0)
    else {
        return;
    };
    if keyboard.just_pressed(KeyCode::KeyJ) {
        pending.move_cursor(1);
    }
    if keyboard.just_pressed(KeyCode::KeyK) {
        pending.move_cursor(-1);
    }
    if let Some(verb) = keyboard.get_just_pressed().find_map(|key| verb_for_keycode(*key)) {
        pending.mark_current(verb);
    }
}

/// Unlike `handle_line_input`, this runs on all three panes regardless of
/// which is active -- a 削除 mark set before switching away must keep
/// sweeping in the background, not freeze until the player looks back.
pub(super) fn animate_delete_wipe(time: Res<Time>, mut panes: Query<&mut Pending>) {
    let step = time.delta_secs() / DELETE_WIPE_SECONDS;
    for mut pending in &mut panes {
        for line in pending.lines.iter_mut() {
            if line.mark == Some(Verb::Delete) && line.delete_wipe < 1.0 {
                line.delete_wipe = (line.delete_wipe + step).min(1.0);
            }
        }
    }
}
