use std::io::{BufRead, Lines};

use crate::grid::{Adjacency, GrowableGrid};

/// A roll is unreachable when `UNREACHABLE_THRESHOLD` rolls are adjacent to it.
const UNREACHABLE_THRESHOLD: u8 = 4;

#[derive(thiserror::Error, Debug)]
pub enum PaperStorageError {
    #[error("no rows in paper inventory")]
    EmptyInventory,

    #[error(transparent)]
    IoError(#[from] std::io::Error),
}

pub struct PaperStorage {
    grid: GrowableGrid,
}

impl PaperStorage {
    /// Import a paper inventory from an iterable of strings.
    pub fn import<T: BufRead>(lines: Lines<T>) -> Result<PaperStorage, PaperStorageError> {
        let mut lines = lines.peekable();
        let cols = match lines.peek() {
            Some(Ok(line)) => Ok(line.len()),
            _ => Err(PaperStorageError::EmptyInventory),
        }?;
        let mut grid = GrowableGrid::new(cols);

        let mut current_row: usize = 0;
        for line in lines {
            let line = line?;
            tracing::trace!("{:?}", line);
            for (col, character) in line.chars().enumerate() {
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

        Ok(PaperStorage { grid })
    }

    /// Remove all reachable rolls of paper. Returns the number of rolls removed.
    pub fn remove_reachable_rolls(&mut self) -> usize {
        // Identify all of the reachable rolls
        let mut reachable = vec![];
        for (coords, val) in self.grid.iter() {
            match val {
                Some(adjacent_rolls) if *adjacent_rolls < UNREACHABLE_THRESHOLD => {
                    reachable.push(coords);
                }
                _ => {}
            }
        }

        // Remove each reachable roll and remove it from the adjacency count of each surrounding
        // roll
        for (x, y) in reachable.iter() {
            self.grid.put(*x, *y, None);
            for direction in [
                Adjacency::TopLeft,
                Adjacency::Top,
                Adjacency::TopRight,
                Adjacency::Right,
                Adjacency::BottomRight,
                Adjacency::Bottom,
                Adjacency::BottomLeft,
                Adjacency::Left,
            ] {
                if let Some(adjacent_rolls) = self.grid.get_adjacent(*x, *y, direction) {
                    *adjacent_rolls -= 1;
                    tracing::trace!(
                        "Decremented ({x}, {y}) {direction:?}, new value {adjacent_rolls}"
                    );
                }
            }
        }

        reachable.len()
    }
}

impl std::fmt::Display for PaperStorage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "PaperStorage: {}", self.grid)
    }
}
