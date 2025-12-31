use std::ops::RangeInclusive;
use std::str::FromStr;

#[derive(thiserror::Error, Debug)]
#[error("invalid range: {0}")]
pub struct RangeError(String);

/// Trait for parsing `0-100`-type ranges that often appear in AOC inputs.
pub trait ParseRange: std::marker::Sized {
    fn parse_range(s: &str) -> Result<Self, RangeError>;
}

impl<T: FromStr> ParseRange for RangeInclusive<T> {
    fn parse_range(s: &str) -> Result<Self, RangeError> {
        let (start, end) = s.split_once('-').ok_or(RangeError(s.into()))?;
        let (start, end) = (
            start.parse::<T>().map_err(|_| RangeError(s.into()))?,
            end.trim().parse::<T>().map_err(|_| RangeError(s.into()))?,
        );
        Ok(start..=end)
    }
}

impl<T: FromStr> ParseRange for (T, T) {
    fn parse_range(s: &str) -> Result<Self, RangeError> {
        let (start, end) = s.split_once('-').ok_or(RangeError(s.into()))?;
        let (start, end) = (
            start.parse::<T>().map_err(|_| RangeError(s.into()))?,
            end.trim().parse::<T>().map_err(|_| RangeError(s.into()))?,
        );
        Ok((start, end))
    }
}
