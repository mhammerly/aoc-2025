use std::ops::RangeInclusive;
use std::str::FromStr;

#[derive(thiserror::Error, Debug)]
#[error("invalid range")]
pub struct RangeError;

pub trait ParseRange: std::marker::Sized {
    fn parse_range(s: &str) -> Result<Self, RangeError>;
}

impl<T: FromStr> ParseRange for RangeInclusive<T> {
    fn parse_range(s: &str) -> Result<Self, RangeError> {
        let (start, end) = s.split_once('-').ok_or(RangeError {})?;
        let (start, end) = (
            start.parse::<T>().map_err(|_| RangeError {})?,
            end.trim().parse::<T>().map_err(|_| RangeError {})?,
        );
        Ok(start..=end)
    }
}
