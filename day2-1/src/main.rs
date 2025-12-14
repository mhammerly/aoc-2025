use std::env;
use std::fs::File;
use std::io::{BufReader, prelude::*};

fn is_invalid(id: u64) -> bool {
    // floor(log10)
    let log = id.ilog10();

    // If floor(log10) is even, our number has an odd number of digits. Invalid IDs must
    // have an even number of digits.
    // Example: 10**2 == 100
    // Example: 10**4 = 10000
    if log % 2 == 0 {
        return false;
    }

    // The midpoint of the number's base 10 representation.
    // Example: 1111122222 -> 100000
    let base_10_midpoint = 10_u64.pow(log / 2 + 1);

    // Example: 1111122222 / 100000 == 11111
    let top_half = id / base_10_midpoint;

    // Example: 1111122222 % 100000 == 22222
    let bottom_half = id % base_10_midpoint;

    // Example: 1111122222 -> 11111 == 22222 -> false
    // Example: 1234512345 -> 12345 == 12345 -> true
    top_half == bottom_half
}

#[derive(thiserror::Error, Debug)]
#[error("invalid range")]
struct InvalidRange();

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let input_file = File::open(format!("{}/day2-1.input", env!("CARGO_MANIFEST_DIR")))?;
    let reader = BufReader::new(input_file);

    let mut sum = 0;
    for item in reader.split(b',') {
        let item = item?;
        let range = str::from_utf8(&item)?;
        tracing::debug!("Processing range {:?}", range);

        let (start, end) = range.split_once('-').ok_or(InvalidRange {})?;
        let (start, end) = (start.parse::<u64>()?, end.trim().parse::<u64>()?);

        for i in start..end {
            tracing::trace!("Testing {i}");
            if is_invalid(i) {
                tracing::debug!("Found invalid id {i}");
                sum += i;
            }
        }
    }

    tracing::info!("Sum of invalid IDs: {}", sum);

    Ok(())
}
