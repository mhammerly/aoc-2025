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

    let total_fresh_ingredients = kitchen.fresh_ingredients().count();
    tracing::info!("Total number of fresh ingredients: {total_fresh_ingredients}");

    Ok(total_fresh_ingredients.to_string())
}

util::main!();
