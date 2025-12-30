use std::fs::File;
use std::io::{BufReader, prelude::*};

use day4::paper_storage::PaperStorage;

fn solve(reader: BufReader<File>) -> anyhow::Result<String> {
    let mut paper_storage = PaperStorage::import(reader.lines())?;
    tracing::debug!("{}", paper_storage);
    let mut total_removed = 0;
    loop {
        let removed = paper_storage.remove_reachable_rolls();
        if removed == 0 {
            tracing::debug!("Removed 0, exiting");
            break;
        }
        tracing::debug!("Removed {removed}: {paper_storage}");
        total_removed += removed;
    }

    tracing::info!("Total number of rolls removed: {total_removed}");

    Ok(total_removed.to_string())
}

util::main!();
