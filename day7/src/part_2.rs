use std::fs::File;
use std::io::{BufRead, BufReader};

use day7::*;

fn solve(reader: BufReader<File>) -> anyhow::Result<String> {
    for line in reader.lines() {
        tracing::trace!("{line:?}");
    }

    panic!("not implemented");
}

util::main!();
