use std::fs::File;
use std::io::{BufReader, prelude::*};

const BATTERIES_PER_BANK: usize = 2;

fn solve(reader: BufReader<File>) -> anyhow::Result<String> {
    let mut total_joltage = 0;
    for bank in reader.lines() {
        let bank = bank?;
        tracing::debug!("Processing bank: {}", bank);

        let bank_joltage = day3::max_joltage(bank.as_bytes(), BATTERIES_PER_BANK)?;

        tracing::debug!("> Bank joltage: {bank_joltage}");
        total_joltage += bank_joltage;
    }

    tracing::info!("Total joltage: {total_joltage}");
    Ok(total_joltage.to_string())
}

util::main!();
