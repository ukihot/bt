use bevy::prelude::*;
use rand::RngExt;

use super::pending::Pending;

/// Ambient CRT-static flicker: about once every few seconds, one character
/// somewhere in the visible log flips to a noise glyph for exactly one
/// frame, then self-heals -- `super::render::sync_log_display` runs every
/// frame and always re-derives the true text, so nothing needs to
/// explicitly revert this. Purely a texture on the whole screen, never tied
/// to any specific line's meaning (or any one pane), so it can never
/// function as an out-of-band tell.
#[derive(Resource)]
pub(super) struct GlitchTimer(Timer);

impl Default for GlitchTimer {
    fn default() -> Self {
        Self(random_glitch_timer())
    }
}

fn random_glitch_timer() -> Timer {
    let mut rng = rand::rng();
    Timer::from_seconds(rng.random_range(3.0..10.0), TimerMode::Once)
}

const GLITCH_GLYPHS: &[char] =
    &['ｦ', 'ｱ', 'ｳ', 'ｴ', 'ｵ', 'ﾃ', 'ﾅ', 'ﾆ', 'ﾎ', 'ﾜ', '#', '/', '\\', '|'];

pub(super) fn glitch_flicker(
    time: Res<Time>,
    mut glitch_timer: ResMut<GlitchTimer>,
    panes: Query<&Pending>,
    mut texts: Query<&mut Text>,
) {
    glitch_timer.0.tick(time.delta());
    if !glitch_timer.0.just_finished() {
        return;
    }
    glitch_timer.0 = random_glitch_timer();

    let non_empty: Vec<&Pending> = panes.iter().filter(|p| !p.lines.is_empty()).collect();
    if non_empty.is_empty() {
        return;
    }
    let mut rng = rand::rng();
    let pending = non_empty[rng.random_range(0..non_empty.len())];
    let Some(line) = pending.lines.get(rng.random_range(0..pending.lines.len())) else {
        return;
    };
    let Ok(mut text) = texts.get_mut(line.entity) else {
        return;
    };
    let mut chars: Vec<char> = text.0.chars().collect();
    if chars.is_empty() {
        return;
    }
    let i = rng.random_range(0..chars.len());
    chars[i] = GLITCH_GLYPHS[rng.random_range(0..GLITCH_GLYPHS.len())];
    text.0 = chars.into_iter().collect();
}
