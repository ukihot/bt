use bevy::prelude::*;

use super::pending::{Pending, PendingLine};
use crate::domain::Verb;

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
/// content's own unbreakable width -- and every log line renders as a row
/// of single-character cells with no wrapping (第3.1節: 1行=1件を画面上の
/// 見た目とも合わせるため), so one long line (a rumor quoting a customer
/// verbatim runs noticeably longer than this pane's other lines, e.g.) is
/// exactly the kind of unbreakable content that can otherwise force its
/// pane wider than its `flex_grow` share, skewing the whole 2x2/8:5 grid
/// (第3.1節). Every width-bearing `Node` between the grid and an individual
/// log line's row -- pane boxes, their log containers (`setup::spawn_pane`),
/// and each spawned line's row (`spawn::spawn_line_ui`) -- runs through this
/// one function instead of re-deriving `min_width: Val::Px(0.0)` at each
/// call site, so the invariant can't quietly go missing from just one of
/// them.
pub(super) fn shrinkable(mut node: Node) -> Node {
    node.min_width = Val::Px(0.0);
    node
}

/// One character cell (or the cursor/mark control glyph) belonging to a
/// pending line's row of cells (`super::spawn::spawn_line_ui`). A line used
/// to be a single `Text` entity holding its whole string; splitting it into
/// one small UI node per glyph is what lets a single character carry its
/// own background fill or border independent of its neighbors --
/// `TextSpan` can vary a run's color or font within one `Text`, but it
/// isn't a UI node itself and can't paint a fill or a border. `super::glitch`
/// is the only thing that ever touches a cell's `BackgroundColor`/`Node
/// ::border`/`BorderColor`; this module only ever touches `Text` (content)
/// and, via `super::glitch::ambient_shimmer`, `TextColor`.
#[derive(Component)]
pub(super) struct LineCells {
    pub(super) cursor: Entity,
    pub(super) mark: Entity,
    pub(super) chars: Vec<Entity>,
}

/// Per-character glyphs for a line's body -- the leading `delete_wipe`
/// fraction (by character count) is replaced with `DELETE_WIPE_GLYPH` when
/// the line is marked for deletion, rounding up so any progress at all
/// shows something immediately. Otherwise just the line's own characters.
fn body_glyphs(line: &PendingLine) -> Vec<char> {
    let chars: Vec<char> = line.text.chars().collect();
    if line.mark != Some(Verb::Delete) || line.delete_wipe <= 0.0 {
        return chars;
    }
    let covered = ((chars.len() as f32) * line.delete_wipe).ceil() as usize;
    chars
        .into_iter()
        .enumerate()
        .map(|(i, ch)| if i < covered { DELETE_WIPE_GLYPH } else { ch })
        .collect()
}

fn mark_glyph(mark: Option<Verb>) -> &'static str {
    match mark {
        Some(Verb::Stamp) => MARK_STAMP,
        _ => MARK_BLANK,
    }
}

/// Writes `value` into a cell only if it actually differs from what's
/// already there. `Text`'s `DerefMut` marks its entity for reshaping
/// unconditionally on every write, and with one entity per *character* now
/// (`LineCells`) rather than per line, touching all of them every frame
/// regardless of whether anything changed would multiply the per-frame
/// reshape cost by a visible line's length for nothing -- most cells are
/// identical frame to frame; only the cursor row and whatever
/// wipe/glitch/shimmer is actively animating actually differ.
fn set_cell_text(texts: &mut Query<&mut Text>, entity: Entity, value: &str) {
    if let Ok(mut text) = texts.get_mut(entity)
        && text.0 != value
    {
        text.0 = value.to_string();
    }
}

/// Writes every cell of one line -- cursor glyph, mark glyph, and body --
/// from `line`'s current state. Shared by the per-frame sync below and by
/// `render_resolved_line`'s one-shot freeze.
fn render_cells(
    cells: &LineCells,
    cursor_glyph: &str,
    line: &PendingLine,
    texts: &mut Query<&mut Text>,
) {
    set_cell_text(texts, cells.cursor, cursor_glyph);
    set_cell_text(texts, cells.mark, mark_glyph(line.mark));
    let mut buf = [0u8; 4];
    for (cell, ch) in cells.chars.iter().zip(body_glyphs(line)) {
        set_cell_text(texts, *cell, ch.encode_utf8(&mut buf));
    }
}

/// Once a line leaves `Pending` -- aged out or otherwise resolved -- it
/// becomes read-only history: its final mark stays visible as a record of
/// what happened, but the cursor must never be left on it. Without this,
/// whichever row the cursor happened to be on at the moment it aged out
/// keeps showing `CURSOR_MARK` forever, since it's no longer in
/// `pending.lines` for `sync_log_display` to ever touch again. Called once,
/// right when a line ages out, and never again -- its cells (and whatever
/// `super::glitch`/`ambient_shimmer` last left on them) are simply never
/// visited by anything past this point.
pub(super) fn render_resolved_line(
    line: &PendingLine,
    cells: &LineCells,
    texts: &mut Query<&mut Text>,
) {
    render_cells(cells, CURSOR_BLANK, line, texts);
}

/// Re-derives every pending row's displayed text from `pending` every
/// frame -- the source of truth -- rather than only on the frames something
/// changed. Runs once for all three panes rather than being copied per
/// pane -- each `Pending` is just another match for the query.
pub(super) fn sync_log_display(
    panes: Query<&Pending>,
    line_cells: Query<&LineCells>,
    mut texts: Query<&mut Text>,
) {
    for pending in &panes {
        for (i, line) in pending.lines.iter().enumerate() {
            let Ok(cells) = line_cells.get(line.entity) else { continue };
            let cursor = if i == pending.cursor { CURSOR_MARK } else { CURSOR_BLANK };
            render_cells(cells, cursor, line, &mut texts);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn line_with(text: &str, mark: Option<Verb>, delete_wipe: f32) -> PendingLine {
        PendingLine {
            entity: Entity::PLACEHOLDER,
            text: text.to_string(),
            classification: crate::domain::Classification::Normal,
            relief: 0.0,
            mark,
            delete_wipe,
        }
    }

    #[test]
    fn body_glyphs_leaves_text_untouched_without_a_delete_mark() {
        let line = line_with("abc", None, 0.0);
        assert_eq!(body_glyphs(&line), vec!['a', 'b', 'c']);
    }

    #[test]
    fn body_glyphs_covers_the_whole_line_at_full_wipe_progress() {
        let line = line_with("abc", Some(Verb::Delete), 1.0);
        assert_eq!(body_glyphs(&line), vec!['═', '═', '═']);
    }

    #[test]
    fn body_glyphs_rounds_up_so_partial_progress_is_visible_immediately() {
        // 1/3 of 3 chars rounds up to covering 1, not 0.
        let line = line_with("abc", Some(Verb::Delete), 0.01);
        assert_eq!(body_glyphs(&line), vec!['═', 'b', 'c']);
    }

    #[test]
    fn mark_glyph_only_shows_the_stamp_mark() {
        assert_eq!(mark_glyph(Some(Verb::Stamp)), MARK_STAMP);
        assert_eq!(mark_glyph(Some(Verb::Delete)), MARK_BLANK);
        assert_eq!(mark_glyph(None), MARK_BLANK);
    }

    #[test]
    fn container_height_scales_linearly_with_capacity() {
        assert_eq!(container_height_px(8), container_height_px(4) * 2.0);
    }
}
