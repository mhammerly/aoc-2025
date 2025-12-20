use std::env;
use std::fs::File;
use std::io::{BufReader, prelude::*};

use day2::*;

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let input_file = File::open(format!("{}/day2.input", env!("CARGO_MANIFEST_DIR")))?;
    let reader = BufReader::new(input_file);

    let mut sum = 0;
    for item in reader.split(b',') {
        let item = item?;
        let range = str::from_utf8(&item)?;
        tracing::debug!(">>> Processing range {:?}", range);

        let id_range: IdRange = range.parse()?;
        for i in id_range.iter() {
            // Check whether this ID consists of the same thing repeated twice
            if count_repetitions(i) == 2 {
                tracing::info!("Invalid: {i}");
                sum += i;
            } else {
                tracing::debug!("Valid:   {i}");
            }
        }
    }

    tracing::info!("Sum of invalid IDs: {}", sum);

    Ok(())
}
