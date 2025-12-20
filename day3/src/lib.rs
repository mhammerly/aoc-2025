pub type Battery = u8;
pub type BatteryBank<'a> = &'a [Battery];

/// "Parse" an ASCII character by subtracting the ASCII value of '0'.
///
/// While we're at it, convert to u64 so our math operations don't overflow.
fn unchar(ascii_char: Battery) -> u64 {
    (ascii_char - 48).into()
}

#[derive(thiserror::Error, Debug)]
#[error("something went wrong when processing a bank")]
pub struct BankError;

/// Find the maximum voltage possible for `bank` by turning on `enable_limit` batteries.
pub fn max_joltage(bank: BatteryBank, enable_limit: usize) -> Result<u64, BankError> {
    let mut bank_joltage = 0;

    let mut first_available_battery = 0;
    for i in 1..=enable_limit {
        let (idx, juiciest_battery) = bank
            .iter()
            .enumerate()
            .skip(first_available_battery)
            .rev()
            .skip(enable_limit - i)
            .max_by_key(|&(_idx, val)| val)
            .ok_or(BankError {})?;

        let pow = u32::try_from(enable_limit - i).map_err(|_| BankError {})?;
        bank_joltage += 10_u64.pow(pow) * unchar(*juiciest_battery);

        tracing::trace!(
            "> {}th juiciest battery is {} at {}",
            i,
            *juiciest_battery as char,
            idx
        );

        first_available_battery = idx + 1;
    }

    Ok(bank_joltage)
}
