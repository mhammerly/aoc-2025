use std::fs::File;
use std::io::{BufRead, BufReader};

use day6::*;

fn solve(reader: BufReader<File>) -> anyhow::Result<String> {
    Ok(Worksheet::new(reader.lines())?
        .solve(NumberFormat::TopBottomRightLeft)?
        .to_string())
}

util::main!();
