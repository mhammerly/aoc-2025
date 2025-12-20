use std::env;
use std::fs::File;
use std::io::{BufReader, prelude::*};

use day4::paper_storage::PaperStorage;

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let input_file = File::open(format!("{}/day4.input", env!("CARGO_MANIFEST_DIR")))?;
    let reader = BufReader::new(input_file);

    let paper_storage = PaperStorage::import(reader.lines())?;

    // A roll is accessible if it a) exists and b) has fewer than four adjacent rolls. Count all
    // such rolls.
    let accessible_rolls = paper_storage
        .iter()
        .filter_map(|cell: &Option<u8>| cell.filter(|count| *count < 4))
        .count();

    tracing::info!("There are {accessible_rolls} accessible paper rolls.");

    Ok(())
}
