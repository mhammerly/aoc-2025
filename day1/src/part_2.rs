use std::fs::File;
use std::io::{BufReader, prelude::*};

use day1::*;
use util::cli::clap::Parser;
use util::input_filepath;

const STARTS_AT: u64 = 50;

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let args = util::cli::Args::parse();

    let input_file = File::open(input_filepath!(args))?;
    let reader = BufReader::new(input_file);

    let mut dial = Dial::new(STARTS_AT);

    tracing::debug!("Starting at {STARTS_AT}");
    for line in reader.lines() {
        dial.turn(line?.parse()?);
    }

    tracing::info!("Touched 0 a total of {} times.", dial.stats.touched_min);

    Ok(())
}
