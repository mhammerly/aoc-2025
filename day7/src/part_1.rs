use std::fs::File;
use std::io::{BufRead, BufReader};

use day7::*;

fn solve(reader: BufReader<File>) -> anyhow::Result<String> {
    let manifold = TachyonManifold::run(reader.lines())?;
    tracing::info!("Tachyon was split {} times.", manifold.splits);
    Ok(manifold.splits.to_string())
}

util::main!();
