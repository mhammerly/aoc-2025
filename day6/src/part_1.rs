use std::fs::File;
use std::io::{BufRead, BufReader};

use day6::Worksheet;

fn solve(reader: BufReader<File>) -> anyhow::Result<String> {
    Ok(Worksheet::new().solve(reader.lines())?.to_string())
}

util::main!();
