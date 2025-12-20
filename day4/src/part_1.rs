use std::env;
use std::fs::File;
use std::io::{BufReader, prelude::*};

use day4::{Adjacency, GridError, GrowableGrid};

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let input_file = File::open(format!("{}/day4.input", env!("CARGO_MANIFEST_DIR")))?;
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
