use bevy::prelude::*;

use super::render::shrinkable;
use crate::game_data::GameData;
use crate::theme::{DIM, FG};

/// How many decimal digits the counter shows. Four comfortably outlasts any
/// plausible run (`CLAUDE.md` never promises the game continues past day
/// 9999) while keeping the widget itself narrow.
const DIGIT_COUNT: usize = 4;

const DIGIT_WIDTH: f32 = 22.0;
const DIGIT_HEIGHT: f32 = 40.0;
const SEGMENT_THICKNESS: f32 = 5.0;
const DIGIT_GAP: f32 = 5.0;
/// Total width of the whole indicator, fixed rather than a flex share of its
/// row -- it's a small instrument cluster, not a panel competing for space
/// with `Kiln`'s log (which takes all the row's remaining width instead).
const INDICATOR_WIDTH: f32 =
    DIGIT_COUNT as f32 * DIGIT_WIDTH + (DIGIT_COUNT as f32 - 1.0) * DIGIT_GAP;

/// A genuine 7-segment display: each digit is 7 individually lit/unlit
/// segments (`Segment`), not a font glyph -- an overlaid "8" ghost behind a
/// real digit glyph looked like a smear rather than a display, since the two
/// `Text` nodes' glyph outlines never actually line up segment-for-segment.
/// Plain rectangles stand in for the real display's elongated hexagonal caps
/// (Bevy's UI `Node` has no polygon/clip-path support to cut angled ends) --
/// the closest a box-only layout system can get to the look, not a claim
/// that it's pixel-faithful to a physical LED. This is a deliberate, narrow
/// exception to 第8節's "文字のみ" rule, alongside the CRT glitch
/// (`glitch.rs`) -- both documented there.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Segment {
    Top,
    TopRight,
    BottomRight,
    Bottom,
    BottomLeft,
    TopLeft,
    Middle,
}

const ALL_SEGMENTS: [Segment; 7] = [
    Segment::Top,
    Segment::TopRight,
    Segment::BottomRight,
    Segment::Bottom,
    Segment::BottomLeft,
    Segment::TopLeft,
    Segment::Middle,
];

/// Which segments are lit for each digit 0-9, indexed the same as
/// `ALL_SEGMENTS` (top/top-right/bottom-right/bottom/bottom-left/top-left/
/// middle) -- the standard 7-segment encoding.
const DIGIT_SEGMENTS: [[bool; 7]; 10] = [
    [true, true, true, true, true, true, false],     // 0
    [false, true, true, false, false, false, false], // 1
    [true, true, false, true, true, false, true],    // 2
    [true, true, true, true, false, false, true],    // 3
    [false, true, true, false, false, true, true],   // 4
    [true, false, true, true, false, true, true],    // 5
    [true, false, true, true, true, true, true],     // 6
    [true, true, true, false, false, false, false],  // 7
    [true, true, true, true, true, true, true],      // 8
    [true, true, true, true, false, true, true],     // 9
];

/// This segment's box within one `DIGIT_WIDTH` x `DIGIT_HEIGHT` cell.
/// Horizontal segments (top/middle/bottom) run the cell's width minus a
/// half-thickness inset at each end, so they don't visually collide with
/// the vertical segments they meet at the corners.
fn segment_node(segment: Segment) -> Node {
    let half_h = (DIGIT_HEIGHT - SEGMENT_THICKNESS) / 2.0;
    let inset = SEGMENT_THICKNESS / 2.0;
    let (top, left, width, height) = match segment {
        Segment::Top => (0.0, inset, DIGIT_WIDTH - SEGMENT_THICKNESS, SEGMENT_THICKNESS),
        Segment::Middle => (half_h, inset, DIGIT_WIDTH - SEGMENT_THICKNESS, SEGMENT_THICKNESS),
        Segment::Bottom => (
            DIGIT_HEIGHT - SEGMENT_THICKNESS,
            inset,
            DIGIT_WIDTH - SEGMENT_THICKNESS,
            SEGMENT_THICKNESS,
        ),
        Segment::TopLeft => (inset, 0.0, SEGMENT_THICKNESS, half_h - inset),
        Segment::TopRight => {
            (inset, DIGIT_WIDTH - SEGMENT_THICKNESS, SEGMENT_THICKNESS, half_h - inset)
        }
        Segment::BottomLeft => (half_h + inset, 0.0, SEGMENT_THICKNESS, half_h - inset),
        Segment::BottomRight => {
            (half_h + inset, DIGIT_WIDTH - SEGMENT_THICKNESS, SEGMENT_THICKNESS, half_h - inset)
        }
    };
    Node {
        position_type: PositionType::Absolute,
        top: Val::Px(top),
        left: Val::Px(left),
        width: Val::Px(width),
        height: Val::Px(height),
        ..default()
    }
}

/// Marks one segment of one digit cell so `sync_day_indicator` can find it
/// again -- `position` is which of the `DIGIT_COUNT` cells (0 = leftmost),
/// `segment` is which of the 7 segments within it.
#[derive(Component)]
pub(super) struct DigitSegment {
    position: usize,
    segment: Segment,
}

fn day_digits(day: u32) -> [u32; DIGIT_COUNT] {
    let clamped = day.min(9999);
    let mut out = [0u32; DIGIT_COUNT];
    for (i, c) in format!("{clamped:0width$}", width = DIGIT_COUNT).chars().enumerate() {
        out[i] = c.to_digit(10).unwrap_or(0);
    }
    out
}

/// Builds the indicator's own container (one cell per digit) and returns its
/// entity so the caller can slot it into the layout next to `Kiln`
/// (`setup::setup`) -- it doesn't know or care where it ends up placed.
pub(super) fn spawn_day_indicator(commands: &mut Commands, day: u32) -> Entity {
    let container = commands
        .spawn(shrinkable(Node {
            width: Val::Px(INDICATOR_WIDTH),
            flex_grow: 0.0,
            flex_shrink: 0.0,
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            column_gap: px(DIGIT_GAP),
            ..default()
        }))
        .id();

    let digits = day_digits(day);
    for (position, &digit) in digits.iter().enumerate() {
        let cell = commands
            .spawn(Node {
                width: Val::Px(DIGIT_WIDTH),
                height: Val::Px(DIGIT_HEIGHT),
                position_type: PositionType::Relative,
                ..default()
            })
            .with_children(|parent| {
                for &segment in &ALL_SEGMENTS {
                    let lit = DIGIT_SEGMENTS[digit as usize][segment_index(segment)];
                    parent.spawn((
                        DigitSegment { position, segment },
                        segment_node(segment),
                        BackgroundColor(if lit { FG } else { DIM }),
                    ));
                }
            })
            .id();
        commands.entity(container).add_child(cell);
    }

    container
}

fn segment_index(segment: Segment) -> usize {
    ALL_SEGMENTS.iter().position(|&s| s == segment).expect("segment is one of ALL_SEGMENTS")
}

/// Re-derives every segment's lit/unlit color from `GameData::day` every
/// frame -- the source of truth -- rather than only on the tick the day
/// actually changes, matching `render::sync_log_display`'s pattern.
pub(super) fn sync_day_indicator(
    game_data: Res<GameData>,
    mut segments: Query<(&DigitSegment, &mut BackgroundColor)>,
) {
    let digits = day_digits(game_data.day);
    for (marker, mut color) in &mut segments {
        let lit = DIGIT_SEGMENTS[digits[marker.position] as usize][segment_index(marker.segment)];
        color.0 = if lit { FG } else { DIM };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_digits_pads_small_days_with_leading_zeros() {
        assert_eq!(day_digits(3), [0, 0, 0, 3]);
    }

    #[test]
    fn day_digits_shows_all_four_digits() {
        assert_eq!(day_digits(1234), [1, 2, 3, 4]);
    }

    #[test]
    fn day_digits_clamps_beyond_four_digits() {
        assert_eq!(day_digits(12345), [9, 9, 9, 9]);
    }

    #[test]
    fn every_digit_lights_a_distinct_segment_pattern() {
        // Sanity check the encoding table has no accidental duplicate rows,
        // which would make two different digits look identical.
        for a in 0..10 {
            for b in (a + 1)..10 {
                assert_ne!(
                    DIGIT_SEGMENTS[a], DIGIT_SEGMENTS[b],
                    "digits {a} and {b} look identical"
                );
            }
        }
    }
}
