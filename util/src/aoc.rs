use std::collections::BTreeMap;
use std::fs::File;
use std::io::Write;

use reqwest::{Url, blocking::Client, cookie::Jar, header};

const AOC_BASE_URL: &str = "https://adventofcode.com";

const SESSION_COOKIE_VAR: &str = "AOC_SESSION_COOKIE";

const CORRECT_ANSWER: &str = "That's the right answer!";
const INCORRECT_ANSWER: &str = "That's not the right answer.";
const ALREADY_SOLVED: &str = "You don't seem to be solving the right level.";
const RATE_LIMIT: &str = " left to wait.";

const USER_AGENT: &str = "github.com/mhammerly/aoc-2025";

#[derive(PartialEq)]
pub enum AocResult {
    Correct,
    Incorrect,
}

fn parse_submission_response(text: &str) -> Result<AocResult, AocError> {
    if text.contains(CORRECT_ANSWER) {
        Ok(AocResult::Correct)
    } else if text.contains(INCORRECT_ANSWER) {
        Ok(AocResult::Incorrect)
    } else if text.contains(RATE_LIMIT) {
        Err(AocError::RateLimited)
    } else if text.contains(ALREADY_SOLVED) {
        Err(AocError::AlreadySolved)
    } else {
        tracing::warn!("Unrecognized AOC response: {text}");
        Err(AocError::UnrecognizedResponse)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum AocError {
    /// Error indicating a URL failed to parse. Unfortunately `reqwest` does not re-export the
    /// `url` crate's `ParseError`.
    #[error("failed to parse url")]
    UrlError,

    #[error("unrecognized day: {0}")]
    DayError(String),

    #[error("rate limit tripped")]
    RateLimited,

    #[error("already submitted correct solution for this problem")]
    AlreadySolved,

    #[error("cannot parse AoC response")]
    UnrecognizedResponse,

    #[error("unrecognized day: {0}")]
    UnrecognizedDay(String),

    #[error("unrecognized problem: {0}")]
    UnrecognizedProblem(String),

    #[error("must set `$AOC_SESSION_COOKIE` env var")]
    MissingSession(#[from] std::env::VarError),

    #[error(transparent)]
    IoError(#[from] std::io::Error),

    #[error(transparent)]
    ClientError(#[from] reqwest::Error),
}

fn day_number(day: &str) -> Result<char, AocError> {
    day.chars()
        .nth(3)
        .ok_or(AocError::UnrecognizedDay(day.into()))
}

fn part_number(problem: &str) -> Result<char, AocError> {
    problem
        .chars()
        .nth(5)
        .ok_or(AocError::UnrecognizedProblem(problem.into()))
}

/// Advent of Code client.
pub struct Aoc {
    client: Client,
}

impl Aoc {
    /// Create a new [`Aoc`] instance with an `https://adventofcode.com` session cookie read from
    /// [`SESSION_COOKIE_VAR`].
    ///
    /// The session cookie must begin with `session=`.
    pub fn new() -> Result<Aoc, AocError> {
        let session_cookie = std::env::var(SESSION_COOKIE_VAR)?;
        let cookie_jar = Jar::default();
        cookie_jar.add_cookie_str(
            &session_cookie,
            &AOC_BASE_URL
                .parse::<Url>()
                .map_err(|_| AocError::UrlError)?,
        );

        let client = Client::builder()
            .cookie_provider(cookie_jar.into())
            .build()?;
        Ok(Aoc { client })
    }

    /// View a day's input file.
    ///
    /// `day` is expected to be `day1`, `day2`, or similar, as it would be from `$CARGO_PKG_NAME`.
    pub fn view_input(&self, day: &str) -> Result<String, AocError> {
        let day_number = day_number(day)?;
        let url = format!("{AOC_BASE_URL}/2025/day/{day_number}/input");
        tracing::info!("Fetching {day} input from {url}");
        Ok(self.client.get(url).send()?.text()?)
    }

    /// Download a day's input file to `filepath`.
    ///
    /// `day` is expected to be `day1`, `day2`, or similar, as it would be from `$CARGO_PKG_NAME`.
    pub fn download_input(&self, day: &str, filepath: &str) -> Result<(), AocError> {
        let input = self.view_input(day)?;

        let mut file = File::create(filepath)?;
        tracing::info!("Saving {day} input to {filepath}");
        Ok(write!(file, "{}", input)?)
    }

    /// Submit `solution` to AOC for `problem`.
    ///
    /// `problem` is expected to be `day1-1`, `day2-2`, or similar, as it would be from
    /// `$CARGO_BIN_NAME`.
    pub fn submit(&self, problem: &str, solution: &str) -> Result<AocResult, AocError> {
        let day_number = day_number(problem)?;
        let part_number = part_number(problem)?;
        let url = format!("{AOC_BASE_URL}/2025/day/{day_number}/answer");
        tracing::info!(?solution, "Posting to {url}");

        let formdata = BTreeMap::from_iter([
            ("level".to_string(), part_number.to_string()),
            ("answer".to_string(), solution.to_owned()),
        ]);
        tracing::info!("Form {:?}", formdata);
        let response = self
            .client
            .post(url)
            .form(&formdata)
            .header(header::USER_AGENT, USER_AGENT)
            .send()?;

        parse_submission_response(&response.text()?)
    }
}
