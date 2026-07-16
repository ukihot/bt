use std::collections::VecDeque;

use bevy::prelude::*;

use crate::domain::{Classification, Verb};

pub(super) struct PendingLine {
    pub(super) entity: Entity,
    pub(super) text: String,
    pub(super) classification: Classification,
    pub(super) correct_verb: Option<Verb>,
    /// What the player has marked this line as, if anything. Purely a mark
    /// while the line is still reachable -- it has no effect on its own.
    pub(super) mark: Option<Verb>,
    /// 0.0..=1.0 sweep progress for a 削除 mark's left-to-right wipe.
    /// Irrelevant (and reset to 0) for any other mark.
    pub(super) delete_wipe: f32,
}

/// The window of recent, still-reachable lines for one pane, with a cursor
/// selecting which one J/K/削除/検印 apply to. New lines join at the back;
/// by default the cursor follows the newest one (tailing), until J/K move it
/// deliberately. A line that ages off the front is resolved right then,
/// using whatever mark it currently carries (or none) -- never at the
/// moment the player pressed a key. The gap between marking a line and it
/// actually mattering is the point.
///
/// One `Pending` component lives on each pane's entity -- `capacity` is
/// fixed per pane (`domain::Pane::capacity`), not a single shared constant,
/// since the three panes don't share a screen size (第3.1節).
#[derive(Component)]
pub(super) struct Pending {
    pub(super) lines: VecDeque<PendingLine>,
    pub(super) cursor: usize,
    following: bool,
    capacity: usize,
}

impl Pending {
    pub(super) fn new(capacity: usize) -> Self {
        Self { lines: VecDeque::new(), cursor: 0, following: true, capacity }
    }

    /// Adds a newly spawned line, then evicts from the front until the
    /// window is back within `capacity` -- normally at most one eviction,
    /// but a shrunk `capacity` can age out several at once.
    pub(super) fn push(&mut self, line: PendingLine) -> Vec<PendingLine> {
        self.lines.push_back(line);
        let mut evicted = Vec::new();
        while self.lines.len() > self.capacity.max(1) {
            if let Some(front) = self.lines.pop_front() {
                evicted.push(front);
                self.cursor = self.cursor.saturating_sub(1);
            }
        }
        self.cursor = if self.following {
            self.lines.len().saturating_sub(1)
        } else {
            self.cursor.min(self.lines.len().saturating_sub(1))
        };
        evicted
    }

    pub(super) fn move_cursor(&mut self, delta: isize) {
        if self.lines.is_empty() {
            return;
        }
        let last = self.lines.len() - 1;
        let next = (self.cursor as isize + delta).clamp(0, last as isize) as usize;
        self.cursor = next;
        self.following = self.cursor == last;
    }

    pub(super) fn mark_current(&mut self, verb: Verb) {
        if let Some(line) = self.lines.get_mut(self.cursor) {
            line.mark = Some(verb);
            if verb != Verb::Delete {
                line.delete_wipe = 0.0;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn line(text: &str) -> PendingLine {
        PendingLine {
            entity: Entity::PLACEHOLDER,
            text: text.to_string(),
            classification: Classification::Normal,
            correct_verb: None,
            mark: None,
            delete_wipe: 0.0,
        }
    }

    #[test]
    fn push_follows_the_newest_line_until_capacity_is_exceeded() {
        let mut pending = Pending::new(2);
        assert!(pending.push(line("a")).is_empty());
        assert_eq!(pending.cursor, 0);
        assert!(pending.push(line("b")).is_empty());
        assert_eq!(pending.cursor, 1);
    }

    #[test]
    fn push_evicts_from_the_front_once_over_capacity() {
        let mut pending = Pending::new(2);
        pending.push(line("a"));
        pending.push(line("b"));
        let evicted = pending.push(line("c"));
        assert_eq!(evicted.len(), 1);
        assert_eq!(evicted[0].text, "a");
        assert_eq!(pending.lines.len(), 2);
        // Still following the newest line after the eviction.
        assert_eq!(pending.cursor, 1);
    }

    #[test]
    fn move_cursor_clamps_to_the_window_bounds() {
        let mut pending = Pending::new(8);
        pending.push(line("a"));
        pending.push(line("b"));
        pending.move_cursor(-5);
        assert_eq!(pending.cursor, 0);
        pending.move_cursor(5);
        assert_eq!(pending.cursor, 1);
    }

    #[test]
    fn manual_cursor_move_stops_following_the_newest_line() {
        let mut pending = Pending::new(8);
        pending.push(line("a"));
        pending.push(line("b"));
        pending.move_cursor(-1);
        assert_eq!(pending.cursor, 0);
        // A newly spawned line must not yank the cursor back to the tail
        // once the player has moved it deliberately.
        pending.push(line("c"));
        assert_eq!(pending.cursor, 0);
    }

    #[test]
    fn mark_current_resets_delete_wipe_when_switching_away_from_delete() {
        let mut pending = Pending::new(8);
        pending.push(line("a"));
        pending.mark_current(Verb::Delete);
        pending.lines[0].delete_wipe = 0.6;
        pending.mark_current(Verb::Stamp);
        assert_eq!(pending.lines[0].mark, Some(Verb::Stamp));
        assert_eq!(pending.lines[0].delete_wipe, 0.0);
    }
}
