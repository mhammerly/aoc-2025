use std::env;
use std::fs::File;
use std::io::{BufReader, prelude::*};
use std::iter;

/// The eight adjacencies that each cell may have.
#[allow(unused)]
enum Adjacency {
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
struct GrowableGrid {
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
struct GridError;

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let input_file = File::open(format!("{}/day4-1.input", env!("CARGO_MANIFEST_DIR")))?;
    let reader = BufReader::new(input_file);

    // Figure out how many columns are in our grid of paper rolls. Create a `GrowableGrid` with
    // that number of columns, and it'll add rows as needed.
    let mut lines = reader.lines().peekable();
    let cols = match lines.peek() {
        Some(Ok(line)) => Ok(line.len()),
        _ => Err(GridError {}),
    }?;
    let mut grid = GrowableGrid::new(cols);

    // Iterate over each cell in the paper warehouse left to right, top to bottom.
    let mut current_row: usize = 0;
    for line in lines {
        tracing::debug!("{:?}", line);
        for (col, character) in line?.chars().enumerate() {
            // If there is no paper roll here, do nothing.
            if character != '@' {
                continue;
            }

            // Otherwise, we want to look at all previously-populated cells,
            // - Count how many rolls we find, because that's our initial adjacency count
            // - For each of those rolls, increment their adjacency count to include us
            // Later cells will update our count if they are adjacent to us so we don't have to
            // look to our right or below us.
            let mut adjacent_rolls = 0;
            for adj in [
                Adjacency::Left,
                Adjacency::TopLeft,
                Adjacency::Top,
                Adjacency::TopRight,
            ] {
                if let Some(adjacent) = grid.get_adjacent(current_row, col, adj) {
                    adjacent_rolls += 1;
                    *adjacent += 1;
                }
            }

            grid.put(current_row, col, Some(adjacent_rolls));
        }
        current_row += 1;
    }

    tracing::debug!("{}", grid);

    // A roll is accessible if it a) exists and b) has fewer than four adjacent rolls. Count all
    // such rolls.
    let accessible_rolls = grid
        .iter()
        .filter_map(|cell| cell.filter(|count| *count < 4))
        .count();

    tracing::info!("There are {accessible_rolls} accessible paper rolls.");

    Ok(())
}
