use bevy::prelude::*;
use rand::RngExt;

use super::pending::Pending;
use super::render::LineCells;
use crate::theme::{BG, DIM, FG};

/// Ambient CRT texture: every several seconds, exactly one character
/// somewhere in the visible log gets a brief artifact -- a noise glyph, a
/// flash of reverse video, or a thin frame -- then heals. Always a single
/// random cell out of everything currently on screen, never tied to any
/// specific pane or line's meaning (第8節 例外1) -- purely a texture, and
/// rare/localized enough that it can never read as strobing.
///
/// (An earlier version of this also ran a continuous, always-on per-
/// character shimmer across every line. It looked identical on every line
/// at once and read as constant flicker rather than texture, so it's gone --
/// see CLAUDE.md §8 例外3's replacement, the static per-line tint baked in
/// once at spawn time in `super::spawn::spawn_line_ui`.)
#[derive(Resource)]
pub(super) struct GlitchTimer(Timer);

impl Default for GlitchTimer {
    fn default() -> Self {
        Self(random_glitch_timer())
    }
}

fn random_glitch_timer() -> Timer {
    let mut rng = rand::rng();
    Timer::from_seconds(rng.random_range(5.0..15.0), TimerMode::Once)
}

/// How long a `Invert`/`Frame` artifact stays visible before healing. The
/// `Substitute` glyph swap needs no timer of its own -- `render::
/// sync_log_display` overwrites `Text` from `Pending` truth every frame
/// regardless, so it self-heals for free one frame after it appears.
const EFFECT_DURATION_SECONDS: f32 = 0.3;

const GLITCH_GLYPHS: &[char] =
    &['ｦ', 'ｱ', 'ｳ', 'ｴ', 'ｵ', 'ﾃ', 'ﾅ', 'ﾆ', 'ﾎ', 'ﾜ', '#', '/', '\\', '|'];

#[derive(Clone, Copy)]
enum GlitchKind {
    /// One frame only -- see the `EFFECT_DURATION_SECONDS` doc above.
    Substitute(char),
    /// Reverse video: `TextColor`/`BackgroundColor` swapped for
    /// `EFFECT_DURATION_SECONDS`.
    Invert,
    /// A thin one-pixel frame around the cell for `EFFECT_DURATION_SECONDS`.
    Frame,
}

/// The one character currently mid-artifact, if any. `Invert`/`Frame` need
/// this tracked explicitly so their timer can revert them -- unlike `Text`,
/// nothing else ever rewrites a cell's `BackgroundColor`/border back to
/// normal on its own.
#[derive(Resource, Default)]
pub(super) struct ActiveGlitch(Option<(Entity, GlitchKind, Timer)>);

fn revert(
    entity: Entity,
    kind: GlitchKind,
    colors: &mut Query<&mut TextColor>,
    backgrounds: &mut Query<&mut BackgroundColor>,
    nodes: &mut Query<&mut Node>,
    borders: &mut Query<&mut BorderColor>,
) {
    match kind {
        GlitchKind::Substitute(_) => {}
        GlitchKind::Invert => {
            if let Ok(mut color) = colors.get_mut(entity) {
                color.0 = FG;
            }
            if let Ok(mut background) = backgrounds.get_mut(entity) {
                background.0 = Color::NONE;
            }
        }
        GlitchKind::Frame => {
            if let Ok(mut node) = nodes.get_mut(entity) {
                node.border = UiRect::ZERO;
            }
            if let Ok(mut border) = borders.get_mut(entity) {
                *border = BorderColor::DEFAULT;
            }
        }
    }
}

pub(super) fn glitch_flicker(
    time: Res<Time>,
    mut glitch_timer: ResMut<GlitchTimer>,
    mut active: ResMut<ActiveGlitch>,
    panes: Query<&Pending>,
    line_cells: Query<&LineCells>,
    mut texts: Query<&mut Text>,
    mut colors: Query<&mut TextColor>,
    mut backgrounds: Query<&mut BackgroundColor>,
    mut nodes: Query<&mut Node>,
    mut borders: Query<&mut BorderColor>,
) {
    // Heal whatever's currently active once its own timer runs out --
    // independent of `glitch_timer`, which only governs *starting* a new
    // artifact.
    if let Some((entity, kind, timer)) = &mut active.0 {
        timer.tick(time.delta());
        if timer.is_finished() {
            let (entity, kind) = (*entity, *kind);
            revert(entity, kind, &mut colors, &mut backgrounds, &mut nodes, &mut borders);
            active.0 = None;
        }
    }

    glitch_timer.0.tick(time.delta());
    if !glitch_timer.0.just_finished() {
        return;
    }
    glitch_timer.0 = random_glitch_timer();

    // Every body-text cell currently on screen, across all three panes --
    // cursor/mark glyphs are UI chrome, not log text, and never take part.
    let candidates: Vec<Entity> = panes
        .iter()
        .flat_map(|pending| pending.lines.iter())
        .filter_map(|line| line_cells.get(line.entity).ok())
        .flat_map(|cells| cells.chars.iter().copied())
        .collect();
    if candidates.is_empty() {
        return;
    }
    let mut rng = rand::rng();
    let entity = candidates[rng.random_range(0..candidates.len())];

    let kind = match rng.random_range(0..3) {
        0 => GlitchKind::Substitute(GLITCH_GLYPHS[rng.random_range(0..GLITCH_GLYPHS.len())]),
        1 => GlitchKind::Invert,
        _ => GlitchKind::Frame,
    };

    match kind {
        GlitchKind::Substitute(glyph) => {
            if let Ok(mut text) = texts.get_mut(entity) {
                text.0 = glyph.to_string();
            }
        }
        GlitchKind::Invert => {
            if let Ok(mut color) = colors.get_mut(entity) {
                color.0 = BG;
            }
            if let Ok(mut background) = backgrounds.get_mut(entity) {
                background.0 = FG;
            }
            active.0 =
                Some((entity, kind, Timer::from_seconds(EFFECT_DURATION_SECONDS, TimerMode::Once)));
        }
        GlitchKind::Frame => {
            if let Ok(mut node) = nodes.get_mut(entity) {
                node.border = UiRect::all(Val::Px(1.0));
            }
            if let Ok(mut border) = borders.get_mut(entity) {
                *border = BorderColor::all(DIM);
            }
            active.0 =
                Some((entity, kind, Timer::from_seconds(EFFECT_DURATION_SECONDS, TimerMode::Once)));
        }
    }
}
