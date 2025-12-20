use std::env;
use std::fs::File;
use std::io::{BufReader, prelude::*};

const BATTERIES_PER_BANK: usize = 12;

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let input_file = File::open(format!("{}/day3.input", env!("CARGO_MANIFEST_DIR")))?;
    let reader = BufReader::new(input_file);

    let mut total_joltage = 0;
    for bank in reader.lines() {
        let bank = bank?;
        tracing::debug!("Processing bank: {}", bank);

        let bank_joltage = day3::max_joltage(bank.as_bytes(), BATTERIES_PER_BANK)?;

        tracing::debug!("> Bank joltage: {bank_joltage}");
        total_joltage += bank_joltage;
    }

    tracing::info!("Total joltage: {total_joltage}");

    Ok(())
}
