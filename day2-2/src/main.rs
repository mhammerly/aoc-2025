use std::env;
use std::fs::File;
use std::io::{BufReader, prelude::*};

fn is_invalid(id: u64) -> bool {
    // 10_u64.ilog10() = 1
    // 100_u64.ilog10() = 2
    // 1000_u64.ilog10() = 3
    // The number of digits in a number is `num.ilog10() + 1`
    let num_digits = id.ilog10() + 1;

    // We want to check for 1-digit repeated patterns, 2-digit repeated patterns,
    // 3-digit repeated patterns... up to patterns that are half the length of the
    // original number.
    'outer: for segment_length in 1..=num_digits / 2 {
        // If the original number's length is not a clean multiple of this pattern
        // length, then skip this pattern length.
        if num_digits % segment_length != 0 {
            tracing::trace!("{id}: Skipping segment length of {segment_length}");
            continue;
        }

        // Check whether each segment of this length is equal.
        let potential_repetitions = num_digits / segment_length;
        tracing::trace!("{id}: Trying {potential_repetitions} of length {segment_length}");
        for i in 0..potential_repetitions - 1 {
            // There are two segments we want to extract, so there are three "fenceposts"
            // that we need to identify. These fenceposts are the powers of 10
            // surrounding the segments we want.
            let low_cutoff = 10_u64.pow(segment_length * i);
            let midpoint = 10_u64.pow(segment_length * (i + 1));
            let high_cutoff = 10_u64.pow(segment_length * (i + 2));

            // Consider an example 456789, which we want to split into 3-digit segments.
            //
            // To get the larger segment, we want to strip everything in/above the
            // "millions" place and below the "thousands" place:
            //   456789 % 1000000 / 1000 == 456
            //
            // To get the smaller segment, we want to strip everything in/above the
            // "thousands" place and below the "ones" place:
            //   456789 % 1000 / 1 == 789
            //
            // In this example, there is nothing in the "millions" place and nothing below
            // the "ones" place, so those operations don't do anything. However, if we were to
            // run a larger number 123456789 through the same modulo/division operations,
            // the result is the same.
            let low_segment = id % midpoint / low_cutoff;
            let high_segment = id % high_cutoff / midpoint;

            tracing::trace!("Compare {} with {}", low_segment, high_segment);
            if low_segment != high_segment {
                continue 'outer;
            }
        }
        // If every segment of this length matches, the inner loop exits and we
        // know our ID is invalid.
        return true;
    }
    // If the outer loop exits, we know we failed to find a segment length for which
    // each segment is identical. This means the ID is valid.
    false
}

#[derive(thiserror::Error, Debug)]
#[error("invalid range")]
struct InvalidRange();

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let input_file = File::open(format!("{}/day2-2.input", env!("CARGO_MANIFEST_DIR")))?;
    let reader = BufReader::new(input_file);

    let mut sum = 0;
    for item in reader.split(b',') {
        let item = item?;
        let range = str::from_utf8(&item)?;
        tracing::debug!(">>> Processing range {:?}", range);

        let (start, end) = range.split_once('-').ok_or(InvalidRange {})?;
        let (start, end) = (start.parse::<u64>()?, end.trim().parse::<u64>()?);

        for i in start..=end {
            if is_invalid(i) {
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
