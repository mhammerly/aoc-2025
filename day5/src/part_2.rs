use std::env;
use std::fs::File;
use std::io::{BufReader, prelude::*};

use day5::*;

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let input_file = File::open(format!("{}/day5.input", env!("CARGO_MANIFEST_DIR")))?;
    let _reader = BufReader::new(input_file);

    panic!("not implemented");
}
