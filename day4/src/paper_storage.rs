use std::io::{BufRead, Lines};

use crate::grid::{Adjacency, GridError, GrowableGrid};

pub struct PaperStorage {
    grid: GrowableGrid,
}

impl PaperStorage {
    /// Import a paper inventory from an iterable of strings.
    pub fn import<T: BufRead>(lines: Lines<T>) -> Result<PaperStorage, GridError> {
        let mut lines = lines.peekable();
        let cols = match lines.peek() {
            Some(Ok(line)) => Ok(line.len()),
            _ => Err(GridError {}),
        }?;
        let mut grid = GrowableGrid::new(cols);

        let mut current_row: usize = 0;
        for line in lines {
            let line = line.map_err(|_| GridError {})?;
            tracing::debug!("{:?}", line);
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

    pub fn iter(&self) -> std::slice::Iter<'_, Option<u8>> {
        self.grid.iter()
    }
}

impl std::fmt::Display for PaperStorage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "PaperStorage: {}", self.grid)
    }
}
