use std::env;
use std::fs::File;
use std::io::{BufReader, prelude::*};

/// "Parse" an ASCII character by subtracting the ASCII value of '0'.
///
/// While we're at it, convert to u64 so our math operations don't overflow.
fn unchar(ascii_char: u8) -> u64 {
    (ascii_char - 48).into()
}

const BATTERIES_PER_BANK: usize = 12;

#[derive(thiserror::Error, Debug)]
#[error("something went wrong when processing a bank")]
struct BankError;

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let input_file = File::open(format!("{}/day3-2.input", env!("CARGO_MANIFEST_DIR")))?;
    let reader = BufReader::new(input_file);

    let mut total_joltage: u64 = 0;
    for bank in reader.lines() {
        let bank = bank?;
        tracing::debug!("Processing bank: {}", bank);

        let mut bank_joltage = 0;

        let mut first_available_battery = 0;
        for i in 1..=BATTERIES_PER_BANK {
            let (idx, juiciest_battery) = bank
                .as_bytes()
                .iter()
                .enumerate()
                .skip(first_available_battery)
                .rev()
                .skip(BATTERIES_PER_BANK - i)
                .max_by_key(|&(_idx, val)| val)
                .ok_or(BankError {})?;

            let pow = u32::try_from(BATTERIES_PER_BANK - i)?;
            bank_joltage += 10_u64.pow(pow) * unchar(*juiciest_battery);

            tracing::trace!(
                "> {}th juiciest battery is {} at {}",
                i,
                *juiciest_battery as char,
                idx
            );

            first_available_battery = idx + 1;
        }

        tracing::debug!("> Bank joltage: {bank_joltage}");
        total_joltage += bank_joltage;
    }

    tracing::info!("Total joltage: {total_joltage}");

    Ok(())
}
