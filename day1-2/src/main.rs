use std::env;
use std::fs::File;
use std::io::{BufReader, prelude::*};

use day1_1_lib::safe::*;

const STARTS_AT: u64 = 50;

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let input_file = File::open(format!("{}/day1-2.input", env!("CARGO_MANIFEST_DIR")))?;
    let reader = BufReader::new(input_file);

    let mut dial = Dial::new(STARTS_AT);

    tracing::debug!("Starting at {STARTS_AT}");
    for line in reader.lines() {
        dial.turn(line?.parse()?);
    }

    tracing::info!("Touched 0 a total of {} times.", dial.stats.touched_min);

    Ok(())
}
