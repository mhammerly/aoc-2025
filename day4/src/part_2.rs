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
    let mut _grid = GrowableGrid::new(cols);
    let _silence_unused = Adjacency::Left;

    panic!("Not implemented");
}
