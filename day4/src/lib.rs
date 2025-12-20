use std::iter;

/// The eight adjacencies that each cell may have.
#[allow(unused)]
pub enum Adjacency {
    TopLeft,
    Top,
    TopRight,
    Right,
    BottomRight,
    Bottom,
    BottomLeft,
    Left,
}

/// A 2D grid with a fixed number of columns that automatically appends rows.
pub struct GrowableGrid {
    /// Fixed number of columns.
    cols: usize,
    /// 1D vector in which our 2D grid actually lives.
    grid: Vec<Option<u8>>,
}

impl GrowableGrid {
    /// Create a [`GrowableGrid`] with a fixed number of columns.
    pub fn new(cols: usize) -> GrowableGrid {
        GrowableGrid {
            cols,
            grid: vec![None; cols],
        }
    }

    /// Figures out the 1D vector index for given 2D grid coordinates. If that index doesn't exist,
    /// add rows to the grid until it does.
    #[inline]
    fn idx(&mut self, x: usize, y: usize) -> usize {
        let idx = self.cols * x + y;
        while idx >= self.grid.len() {
            self.grid.extend(iter::repeat_n(None, self.cols));
        }
        idx
    }

    /// Get a mutable reference to a grid cell at coordinates `(x, y)`, or `None`.
    pub fn get(&mut self, x: usize, y: usize) -> Option<&mut u8> {
        let idx = self.idx(x, y);
        self.grid[idx].as_mut()
    }

    /// Get a mutable reference to a cell that is adjacent to `(x, y)` based on `adj`, or `None`.
    ///
    /// `None` is returned if:
    /// - the adjacent cell exists and is empty
    /// - the adjacent cell doesn't exist because its column is outside `0..self.cols`
    /// - the adjacent cell doesn't exist because its row is less than zero
    pub fn get_adjacent(&mut self, x: usize, y: usize, adj: Adjacency) -> Option<&mut u8> {
        let adjacent_coords = match adj {
            Adjacency::TopLeft => (x.checked_sub(1), y.checked_sub(1)),
            Adjacency::Top => (x.checked_sub(1), Some(y)),
            Adjacency::TopRight => (x.checked_sub(1), Some(y + 1).filter(|y| *y < self.cols)),
            Adjacency::Right => (Some(x), Some(y + 1).filter(|y| *y <= self.cols)),
            Adjacency::BottomRight => (x.checked_add(1), Some(y + 1).filter(|y| *y < self.cols)),
            Adjacency::Bottom => (x.checked_add(1), Some(y)),
            Adjacency::BottomLeft => (x.checked_add(1), y.checked_sub(1)),
            Adjacency::Left => (Some(x), y.checked_sub(1)),
        };
        if let (Some(adjacent_x), Some(adjacent_y)) = adjacent_coords {
            self.get(adjacent_x, adjacent_y)
        } else {
            None
        }
    }

    /// Insert `val` into `(x, y)`.
    pub fn put(&mut self, x: usize, y: usize, val: Option<u8>) {
        let idx = self.idx(x, y);
        self.grid[idx] = val;
    }

    /// Iterate over the grid left to right, top to bottom.
    pub fn iter(&self) -> std::slice::Iter<'_, Option<u8>> {
        self.grid.iter()
    }
}

impl std::fmt::Display for GrowableGrid {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for (i, cell) in self.grid.iter().enumerate() {
            if i != 0 && i % self.cols == 0 {
                writeln!(f)?;
            }
            match cell {
                Some(adjacency_count) => write!(f, "{}", adjacency_count),
                None => write!(f, "."),
            }?;
        }

        Ok(())
    }
}

#[derive(thiserror::Error, Clone, Debug)]
#[error("failed to parse grid")]
pub struct GridError;
