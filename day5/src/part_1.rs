use std::fs::File;
use std::io::{BufReader, prelude::*};

use day5::*;

fn solve(reader: BufReader<File>) -> anyhow::Result<String> {
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
    Ok(fresh_ingredients.to_string())
}

util::main!();
