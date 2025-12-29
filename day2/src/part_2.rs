use std::fs::File;
use std::io::{BufReader, prelude::*};
use std::ops::RangeInclusive;

use util::range::ParseRange;

use day2::*;
use util::cli::clap::Parser;
use util::input_filepath;

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let args = util::cli::Args::parse();

    let input_file = File::open(input_filepath!(args))?;
    let reader = BufReader::new(input_file);

    let mut sum = 0;
    for item in reader.split(b',') {
        let item = item?;
        let range = str::from_utf8(&item)?;
        tracing::debug!(">>> Processing range {:?}", range);

        let id_range = RangeInclusive::<u64>::parse_range(range)?;
        for i in id_range {
            // Check whether this ID consists of a substring repeated any number of times
            if count_repetitions(i) > 0 {
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
