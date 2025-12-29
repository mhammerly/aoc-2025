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
    let accessible_rolls = paper_storage.remove_reachable_rolls();

    tracing::info!("There are {accessible_rolls} accessible paper rolls.");

    Ok(())
}
