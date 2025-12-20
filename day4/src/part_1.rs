use std::env;
use std::fs::File;
use std::io::{BufReader, prelude::*};

use day4::paper_storage::PaperStorage;

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let input_file = File::open(format!("{}/day4.input", env!("CARGO_MANIFEST_DIR")))?;
    let reader = BufReader::new(input_file);

    let mut paper_storage = PaperStorage::import(reader.lines())?;
    let accessible_rolls = paper_storage.remove_reachable_rolls();

    tracing::info!("There are {accessible_rolls} accessible paper rolls.");

    Ok(())
}
