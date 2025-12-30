use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::RangeInclusive;

use day2::*;
use util::range::ParseRange;

fn solve(reader: BufReader<File>) -> anyhow::Result<String> {
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
    Ok(sum.to_string())
}

util::main!();
