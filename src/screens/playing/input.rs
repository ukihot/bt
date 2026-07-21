use bevy::prelude::*;

use super::pane::PaneRuntime;
use super::pending::Pending;
use crate::domain::{Pane, Verb};

/// How long a 削除 mark takes to sweep across the line, left to right.
const DELETE_WIPE_SECONDS: f32 = 0.4;

fn verb_for_keycode(key: KeyCode) -> Option<Verb> {
    match key {
        KeyCode::KeyZ => Some(Verb::Delete),
        KeyCode::KeyX => Some(Verb::Stamp),
        _ => None,
    }
}

/// `J`/`K`/削除/検印 all target 焼成室, and only 焼成室 -- CLAUDE.md §3.2:
/// 操作できるのは焼成室だけで、画面を切り替えるという概念自体がない。 A
/// linear scan over three entities to find it is simpler than threading a
/// direct lookup through, and just as cheap.
pub(super) fn handle_line_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut panes: Query<(&PaneRuntime, &mut Pending)>,
) {
    let Some((_, mut pending)) = panes.iter_mut().find(|(runtime, _)| runtime.pane == Pane::Kiln)
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
