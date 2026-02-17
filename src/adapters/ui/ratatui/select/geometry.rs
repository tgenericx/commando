//! Grid navigation math

#[derive(Debug, Clone, Copy)]
pub struct GridPosition {
    row: usize,
    col: usize,
    total_options: usize,
    grid_cols: usize,
}

impl GridPosition {
    pub fn new(index: usize, total_options: usize, grid_cols: usize) -> Self {
        assert!(grid_cols > 0, "grid_cols must be positive");
        Self {
            row: index / grid_cols,
            col: index % grid_cols,
            total_options,
            grid_cols,
        }
    }

    pub fn move_up(&self) -> Option<usize> {
        if self.row > 0 {
            Some((self.row - 1) * self.grid_cols + self.col)
        } else {
            // Wrap to bottom
            let last_row = (self.total_options - 1) / self.grid_cols;
            let last_row_cols = self.total_options % self.grid_cols;
            if last_row_cols > 0 && self.col < last_row_cols {
                Some(last_row * self.grid_cols + self.col)
            } else if last_row_cols > 0 {
                Some(last_row * self.grid_cols + (last_row_cols - 1))
            } else {
                Some((last_row - 1) * self.grid_cols + self.col)
            }
        }
    }

    pub fn move_down(&self) -> Option<usize> {
        let next_row_start = (self.row + 1) * self.grid_cols + self.col;
        if next_row_start < self.total_options {
            Some(next_row_start)
        } else {
            // Wrap to top
            Some(self.col)
        }
    }

    pub fn move_left(&self) -> Option<usize> {
        if self.col > 0 {
            Some(self.row * self.grid_cols + (self.col - 1))
        } else if self.row > 0 {
            // Previous row, last column
            Some((self.row - 1) * self.grid_cols + (self.grid_cols - 1))
        } else {
            // Wrap to bottom row
            let last_row = (self.total_options - 1) / self.grid_cols;
            Some(last_row * self.grid_cols + (self.grid_cols - 1))
        }
    }

    pub fn move_right(&self) -> Option<usize> {
        let next_col = self.row * self.grid_cols + (self.col + 1);
        if next_col < self.total_options {
            Some(next_col)
        } else if self.row * self.grid_cols + self.col + self.grid_cols < self.total_options {
            // Next row, first column
            Some((self.row + 1) * self.grid_cols)
        } else {
            // Wrap to top
            Some(0)
        }
    }

    pub fn home(&self) -> usize {
        0
    }

    pub fn end(&self) -> usize {
        self.total_options - 1
    }

    pub fn jump_to_number(&self, digit: u32) -> Option<usize> {
        let idx = (digit as usize).saturating_sub(1);
        if idx < self.total_options {
            Some(idx)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid_navigation_2x2() {
        let pos = GridPosition::new(0, 4, 2);
        assert_eq!(pos.move_right(), Some(1));
        assert_eq!(pos.move_down(), Some(2));
        assert_eq!(pos.move_left(), Some(1)); // Wraps to bottom?
    }

    #[test]
    fn test_grid_navigation_uneven() {
        let pos = GridPosition::new(2, 3, 2);
        assert_eq!(pos.move_down(), Some(0)); // Wraps to top
        assert_eq!(pos.move_right(), None); // No right from last item
    }
}
