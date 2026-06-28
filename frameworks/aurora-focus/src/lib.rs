//! Aurora UI focus engine.
//!
//! TVs navigate by remote/D-pad, so focus is the core interaction model
//! (`docs/01-Vision.md` § Focus Navigation). This crate is the headless,
//! fully-tested model: a vertical stack of horizontal rows (the canonical TV
//! launcher), with directional movement, column memory, and clamping.
//!
//! The renderer reads [`Focus`] each frame to apply scale/glow/elevation; this
//! crate has no rendering or platform dependencies, so it ports unchanged into
//! the compositor.

/// A D-pad / remote direction.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

/// The currently focused cell: row index and column within that row.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Focus {
    pub row: usize,
    pub col: usize,
}

/// One horizontal row of focusable items (e.g. "Continue Watching").
pub struct Row {
    pub title: String,
    pub len: usize,
}

impl Row {
    pub fn new(title: impl Into<String>, len: usize) -> Self {
        Row { title: title.into(), len }
    }
}

/// A vertical stack of rows with a single focus point.
///
/// Movement rules (standard TV UX):
/// - Left/Right move within the row and clamp at the ends (no wrap).
/// - Up/Down change row and **remember the intended column**, clamping to the
///   new row's length so a short row doesn't lose your place in a long one.
/// - Empty rows are skipped vertically.
pub struct FocusGrid {
    rows: Vec<Row>,
    focus: Focus,
    /// The column the user "wants" — preserved across vertical moves.
    desired_col: usize,
}

impl FocusGrid {
    /// Build a grid and place focus on the first non-empty row.
    pub fn new(rows: Vec<Row>) -> Self {
        let first = rows.iter().position(|r| r.len > 0).unwrap_or(0);
        FocusGrid { rows, focus: Focus { row: first, col: 0 }, desired_col: 0 }
    }

    pub fn focus(&self) -> Focus {
        self.focus
    }

    pub fn rows(&self) -> &[Row] {
        &self.rows
    }

    /// Title of the focused row, if any.
    pub fn focused_row_title(&self) -> Option<&str> {
        self.rows.get(self.focus.row).map(|r| r.title.as_str())
    }

    /// Place focus at an arbitrary cell, clamping row and column into range.
    /// Lets a stateless caller (e.g. the backend API) navigate from any point.
    pub fn set_focus(&mut self, row: usize, col: usize) {
        let row = row.min(self.rows.len().saturating_sub(1));
        let col = self.clamp_col(row, col);
        self.focus = Focus { row, col };
        self.desired_col = col;
    }

    fn clamp_col(&self, row: usize, want: usize) -> usize {
        match self.rows.get(row) {
            Some(r) if r.len > 0 => want.min(r.len - 1),
            _ => 0,
        }
    }

    /// Apply a direction. Returns `true` if focus actually moved.
    pub fn navigate(&mut self, dir: Direction) -> bool {
        let before = self.focus;
        match dir {
            Direction::Left => {
                if self.focus.col > 0 {
                    self.focus.col -= 1;
                    self.desired_col = self.focus.col;
                }
            }
            Direction::Right => {
                let max = self.rows[self.focus.row].len.saturating_sub(1);
                if self.focus.col < max {
                    self.focus.col += 1;
                    self.desired_col = self.focus.col;
                }
            }
            Direction::Up => {
                if let Some(r) = self.next_nonempty_row(self.focus.row, false) {
                    self.focus.row = r;
                    self.focus.col = self.clamp_col(r, self.desired_col);
                }
            }
            Direction::Down => {
                if let Some(r) = self.next_nonempty_row(self.focus.row, true) {
                    self.focus.row = r;
                    self.focus.col = self.clamp_col(r, self.desired_col);
                }
            }
        }
        self.focus != before
    }

    /// Find the nearest non-empty row above (`down=false`) or below (`down=true`).
    fn next_nonempty_row(&self, from: usize, down: bool) -> Option<usize> {
        if down {
            ((from + 1)..self.rows.len()).find(|&i| self.rows[i].len > 0)
        } else {
            (0..from).rev().find(|&i| self.rows[i].len > 0)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn grid() -> FocusGrid {
        FocusGrid::new(vec![
            Row::new("Continue Watching", 5),
            Row::new("Apps", 2),
            Row::new("Recommended", 8),
        ])
    }

    #[test]
    fn starts_on_first_nonempty() {
        let g = grid();
        assert_eq!(g.focus(), Focus { row: 0, col: 0 });
    }

    #[test]
    fn horizontal_clamps_at_ends() {
        let mut g = grid();
        assert!(!g.navigate(Direction::Left)); // already at left edge
        for _ in 0..4 {
            assert!(g.navigate(Direction::Right));
        }
        assert_eq!(g.focus().col, 4);
        assert!(!g.navigate(Direction::Right)); // clamp at end of 5-item row
    }

    #[test]
    fn column_memory_survives_short_row() {
        let mut g = grid();
        for _ in 0..4 {
            g.navigate(Direction::Right); // col 4 in row 0
        }
        g.navigate(Direction::Down); // row 1 has only 2 items -> clamp to col 1
        assert_eq!(g.focus(), Focus { row: 1, col: 1 });
        g.navigate(Direction::Down); // row 2 has 8 -> restore desired col 4
        assert_eq!(g.focus(), Focus { row: 2, col: 4 });
    }

    #[test]
    fn vertical_skips_empty_rows() {
        let mut g = FocusGrid::new(vec![
            Row::new("Top", 3),
            Row::new("Empty", 0),
            Row::new("Bottom", 3),
        ]);
        g.navigate(Direction::Down);
        assert_eq!(g.focus().row, 2); // skipped the empty middle row
    }

    #[test]
    fn cannot_move_above_top() {
        let mut g = grid();
        assert!(!g.navigate(Direction::Up));
    }

    #[test]
    fn set_focus_clamps_into_range() {
        let mut g = grid();
        g.set_focus(99, 99); // last row is "Recommended" (8 items)
        assert_eq!(g.focus(), Focus { row: 2, col: 7 });
        g.set_focus(1, 99); // "Apps" has 2 items
        assert_eq!(g.focus(), Focus { row: 1, col: 1 });
    }
}
