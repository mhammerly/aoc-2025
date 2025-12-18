use std::env;
use std::fs::File;
use std::io::{BufReader, prelude::*};

/// "Parse" an ASCII character by subtracting the ASCII value of '0'.
///
/// While we're at it, convert to u16 so our math operations don't overflow.
fn unchar(ascii_char: u8) -> u16 {
    (ascii_char - 48).into()
}

#[derive(thiserror::Error, Debug)]
#[error("something went wrong when processing a bank")]
struct BankError;

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let input_file = File::open(format!("{}/day3-1.input", env!("CARGO_MANIFEST_DIR")))?;
    let reader = BufReader::new(input_file);

    let mut total_joltage: u16 = 0;
    for bank in reader.lines() {
        let bank = bank?;
        tracing::debug!("Processing bank: {}", bank);

        // First, we have to find the first occurrence of the largest joltage value in the bank. We
        // also need the index of that occurrence for later.
        //
        // `max_by_key()` returns the _last_ occurrence of the largest value, so to get the first,
        // we want to reverse the bank.
        //
        // We also have to skip the last battery in the bank, because the second battery must be
        // after the first battery.
        let (idx, first_joltage) = bank
            .as_bytes()
            .iter()
            .enumerate()
            .rev()
            .skip(1)
            .max_by_key(|&(_idx, val)| val)
            .ok_or(BankError {})?;
        tracing::trace!(
            "> Found first battery: {} (idx {})",
            *first_joltage as char,
            idx
        );

        // Now that we have our first battery, we need to find the next-largest battery in our bank
        // that comes after it. That's our second battery.
        let second_joltage = bank
            .as_bytes()
            .iter()
            .skip(idx + 1)
            .max()
            .ok_or(BankError {})?;
        tracing::trace!("> Found second battery: {}", *first_joltage as char,);

        let bank_joltage = (10 * unchar(*first_joltage)) + unchar(*second_joltage);
        tracing::debug!("> Final joltage for bank: {}", bank_joltage);

        total_joltage += bank_joltage;
    }

    tracing::info!("Total joltage: {total_joltage}");

    Ok(())
}
