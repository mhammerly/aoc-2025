use std::fs::File;
use std::io::{BufRead, BufReader};

use day1::*;

const STARTS_AT: u64 = 50;

fn solve(reader: BufReader<File>) -> anyhow::Result<String> {
    let mut dial = Dial::new(STARTS_AT);

    tracing::debug!("Starting at {STARTS_AT}");
    for line in reader.lines() {
        dial.turn(line?.parse()?);
    }

    tracing::info!("Landed on 0 a total of {} times.", dial.stats.landed_on_min);
    Ok(dial.stats.landed_on_min.to_string())
}

util::main!();
