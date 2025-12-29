use std::fs::File;
use std::io::{BufReader, prelude::*};

use day4::paper_storage::PaperStorage;
use util::cli::clap::Parser;
use util::input_filepath;

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let args = util::cli::Args::parse();

    let input_file = File::open(input_filepath!(args))?;
    let reader = BufReader::new(input_file);

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

    Ok(())
}
