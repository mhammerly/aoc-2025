use std::fs::File;
use std::io::{BufReader, prelude::*};

use day4::paper_storage::PaperStorage;

fn solve(reader: BufReader<File>) -> anyhow::Result<String> {
    let mut paper_storage = PaperStorage::import(reader.lines())?;
    let accessible_rolls = paper_storage.remove_reachable_rolls();

    tracing::info!("There are {accessible_rolls} accessible paper rolls.");

    Ok(accessible_rolls.to_string())
}

util::main!();
