use std::fs::File;
use std::io::Write;

use reqwest::{Url, blocking::Client, cookie::Jar};

const AOC_BASE_URL: &str = "https://adventofcode.com";

#[derive(thiserror::Error, Debug)]
pub enum AocError {
    /// Error indicating a URL failed to parse. Unfortunately `reqwest` does not re-export the
    /// `url` crate's `ParseError`.
    #[error("failed to parse url")]
    UrlError,

    #[error("invalid argument")]
    ArgError,

    #[error(transparent)]
    IoError(#[from] std::io::Error),

    #[error(transparent)]
    ClientError(#[from] reqwest::Error),
}

pub struct Aoc {
    client: Client,
}

impl Aoc {
    pub fn new(session_cookie: &str) -> Result<Aoc, AocError> {
        let cookie_jar = Jar::default();
        cookie_jar.add_cookie_str(
            session_cookie,
            &AOC_BASE_URL
                .parse::<Url>()
                .map_err(|_| AocError::UrlError)?,
        );

        let client = Client::builder()
            .cookie_provider(cookie_jar.into())
            .build()?;
        Ok(Aoc { client })
    }

    pub fn view_input(&self, day: &str) -> Result<String, AocError> {
        let day_number = day.chars().last().ok_or(AocError::ArgError)?;
        let url = format!("{AOC_BASE_URL}/2025/day/{day_number}/input");
        tracing::info!("Fetching {day} input from {url}");
        Ok(self.client.get(url).send()?.text()?)
    }

    pub fn download_input(&self, day: &str, filepath: &str) -> Result<(), AocError> {
        let input = self.view_input(day)?;

        let mut file = File::create(filepath)?;
        tracing::info!("Saving {day} input to {filepath}");
        Ok(write!(file, "{}", input)?)
    }
}
