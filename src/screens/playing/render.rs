use bevy::prelude::*;

use super::pane::ActivePane;
use super::pending::{Pending, PendingLine};
use crate::domain::{Pane, Verb};

pub(super) const LOG_FONT_SIZE: f32 = 16.0;
/// Matches `bevy_text::LineHeight`'s own default (`RelativeToFont(1.2)`,
/// i.e. 1.2x font size) -- since the log lines never set `LineHeight`
/// themselves, this is the actual rendered line height, not an estimate.
/// Using anything else here desyncs `container_height_px` from what's
/// really on screen, silently clipping rows the cursor can still reach.
const LINE_HEIGHT_RATIO: f32 = 1.2;

pub(super) const CURSOR_MARK: &str = "＞";
pub(super) const CURSOR_BLANK: &str = "　";
const MARK_STAMP: &str = "✓";
pub(super) const MARK_BLANK: &str = "　";

/// 二重線 -- one glyph, two horizontal strokes. Progressively overwrites the
/// line's own characters as the wipe plays, rather than a separate
/// strikethrough decoration (which this font/shaper can't render -- see the
/// abandoned combining-character attempt).
const DELETE_WIPE_GLYPH: char = '═';

/// The pixel height of a log box showing exactly `capacity` lines at
/// `LOG_FONT_SIZE`. Used as the container's fixed `Node::height` -- never
/// `flex_grow` -- so the number of visible lines is dictated by `capacity`
/// alone, not by however much space the window happens to have.
pub(super) fn container_height_px(capacity: usize) -> f32 {
    capacity as f32 * LOG_FONT_SIZE * LINE_HEIGHT_RATIO
}

/// A flex item's default `min_width` (`Val::Auto`) falls back to its
/// content's own unbreakable width -- and every log line renders with
/// `TextLayout::no_wrap()` (第3.1節: 1行=1件を画面上の見た目とも合わせる
/// ため), so one long line (a rumor quoting a customer verbatim runs
/// noticeably longer than this pane's other lines, e.g.) is exactly the kind
/// of unbreakable content that can otherwise force its pane wider than its
/// `flex_grow` share, skewing the whole 2x2/8:5 grid (第3.1節). Every
/// width-bearing `Node` between the grid and an individual log line --
/// pane boxes, their log containers (`setup::spawn_pane`), and each spawned
/// line (`spawn::spawn_line_ui`) -- runs through this one function instead
/// of re-deriving `min_width: Val::Px(0.0)` at each call site, so the
/// invariant can't quietly go missing from just one of them.
pub(super) fn shrinkable(mut node: Node) -> Node {
    node.min_width = Val::Px(0.0);
    node
}

/// Overwrites the leading `progress` fraction of `text` (by character count)
/// with `DELETE_WIPE_GLYPH`, rounding up so any progress at all shows
/// something immediately.
fn wipe(text: &str, progress: f32) -> String {
    if progress <= 0.0 {
        return text.to_string();
    }
    let chars: Vec<char> = text.chars().collect();
    let covered = ((chars.len() as f32) * progress).ceil() as usize;
    chars
        .iter()
        .enumerate()
        .map(|(i, ch)| if i < covered { DELETE_WIPE_GLYPH } else { *ch })
        .collect()
}

/// The mark glyph and body text for a line, independent of cursor state.
fn render_line_body(line: &PendingLine) -> (&'static str, String) {
    match line.mark {
        Some(Verb::Delete) => (MARK_BLANK, wipe(&line.text, line.delete_wipe)),
        Some(Verb::Stamp) => (MARK_STAMP, line.text.clone()),
        None => (MARK_BLANK, line.text.clone()),
    }
}

fn render_pending_line(pending: &Pending, index: usize, line: &PendingLine) -> String {
    let cursor = if index == pending.cursor { CURSOR_MARK } else { CURSOR_BLANK };
    let (mark, body) = render_line_body(line);
    format!("{cursor}{mark}{body}")
}

/// Once a line leaves `Pending` -- aged out or otherwise resolved -- it
/// becomes read-only history: its final mark stays visible as a record of
/// what happened, but the cursor must never be left on it. Without this,
/// whichever row the cursor happened to be on at the moment it aged out
/// keeps showing `CURSOR_MARK` forever, since it's no longer in
/// `pending.lines` for `sync_log_display` to ever touch again.
pub(super) fn render_resolved_line(line: &PendingLine) -> String {
    let (mark, body) = render_line_body(line);
    format!("{CURSOR_BLANK}{mark}{body}")
}

/// Re-derives every pending row's displayed text from `pending` every
/// frame -- the source of truth -- rather than only on the frames something
/// changed. This is also what makes `super::glitch::glitch_flicker`'s
/// one-frame corruption self-healing: whatever it overwrites gets put back
/// correctly right here on the very next frame. Runs once for all three
/// panes rather than being copied per pane -- each `Pending` is just
/// another match for the query.
pub(super) fn sync_log_display(panes: Query<&Pending>, mut texts: Query<&mut Text>) {
    for pending in &panes {
        for (i, line) in pending.lines.iter().enumerate() {
            if let Ok(mut text) = texts.get_mut(line.entity) {
                text.0 = render_pending_line(pending, i, line);
            }
        }
    }
}

/// Marks a pane's title-row text (spawned in `super::setup`), so
/// `sync_pane_headers` knows which entity to update.
#[derive(Component)]
pub(super) struct PaneHeader(pub(super) Pane);

/// The selected pane is marked with the same `CURSOR_MARK` convention as a
/// selected line, not a color (第8節: 色による強調は禁止) -- this is the
/// only visible sign that `H`/`L` did anything.
pub(super) fn sync_pane_headers(
    active: Res<ActivePane>,
    mut headers: Query<(&PaneHeader, &mut Text)>,
) {
    for (header, mut text) in &mut headers {
        let cursor = if header.0 == active.0 { CURSOR_MARK } else { CURSOR_BLANK };
        text.0 = format!("{cursor}{}", header.0.label());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wipe_leaves_text_untouched_at_zero_progress() {
        assert_eq!(wipe("abc", 0.0), "abc");
    }

    #[test]
    fn wipe_covers_the_whole_line_at_full_progress() {
        assert_eq!(wipe("abc", 1.0), "═══");
    }

    #[test]
    fn wipe_rounds_up_so_partial_progress_is_visible_immediately() {
        // 1/3 of 3 chars rounds up to covering 1, not 0.
        assert_eq!(wipe("abc", 0.01), "═bc");
    }

    #[test]
    fn container_height_scales_linearly_with_capacity() {
        assert_eq!(container_height_px(8), container_height_px(4) * 2.0);
    }
}
