use std::fs::File;
use std::io::{BufReader, prelude::*};

use day5::*;
use util::cli::clap::Parser;
use util::input_filepath;

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let args = util::cli::Args::parse();

    let input_file = File::open(input_filepath!(args))?;
    let reader = BufReader::new(input_file);
    let mut lines = reader.lines();

    tracing::debug!("Building kitchen inventory");
    let kitchen = Kitchen::import_fresh_ranges(
        lines
            .by_ref()
            .map(|s| s.expect("failed to read file"))
            .take_while(|s| !s.is_empty()),
    )?;

    tracing::debug!("Counting fresh ingredients");
    let mut fresh_ingredients = 0;
    for line in lines {
        tracing::debug!("> Checking {line:?}");
        if kitchen.is_fresh(line?.parse::<u64>()?) {
            tracing::debug!("> > Fresh");
            fresh_ingredients += 1;
        }
    }

    tracing::info!("Available fresh ingredients: {fresh_ingredients}");

    Ok(())
}
