use std::fmt::{self, Display, Formatter};
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use std::str::FromStr;

pub mod aoc;
pub mod cli;
pub mod range;
pub mod runner;

#[derive(thiserror::Error, Debug)]
#[error("failed to parse problem: {0}")]
pub struct ParseProblemError(String);

/// Describes a specific Advent of Code problem (e.g. day 1 part 2).
///
/// Problems are expected to be created from strings like `"day1-2"`.
///
/// ```
/// # use util::Problem;
/// let p1_2: Problem = "day1-2".parse().unwrap();
/// assert_eq!(p1_2.day, "1");
/// assert_eq!(p1_2.part, "2");
///
/// let p12_1: Problem = "day12-1".parse().unwrap();
/// assert_eq!(p12_1.day, "12");
/// assert_eq!(p12_1.part, "1");
/// ```
#[derive(Clone, Debug)]
pub struct Problem {
    /// Which AOC day (year-agnostic) this problem is from.
    pub day: String,

    /// Which part of a given day's problem this is.
    pub part: String,
}

impl FromStr for Problem {
    type Err = ParseProblemError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let hyphen_pos = s
            .bytes()
            .position(|c| c == b'-')
            .ok_or(ParseProblemError("cannot find day/part delimiter".into()))?;
        Ok(Problem {
            day: s[3..hyphen_pos].into(),
            part: s[hyphen_pos + 1..].into(),
        })
    }
}

impl Display for Problem {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "day{}-{}", self.day, self.part)
    }
}

/// Type alias for solution functions. Solutions should return their results as a [`String`].
pub type SolveFn = fn(BufReader<File>) -> anyhow::Result<String>;

#[derive(Debug)]
pub struct Solution {
    pub problem: Problem,

    pub solve_fn: SolveFn,

    pub working_dir: PathBuf,
}

impl Solution {
    /// Return the path to the input file that should be used for this solution.
    ///
    /// By default, the path for a day 1 problem will be `day1/day1.input`. However, if
    /// `custom_input` is `Some("abc")` (for example), the path will instead be
    /// `day1/day1.abc.input`.
    pub fn input_file(&self, custom_input: &Option<String>) -> PathBuf {
        let input_filename = custom_input
            .as_ref()
            .map(|input| format!("day{}.{}.input", self.problem.day, input))
            .unwrap_or(format!("day{}.input", self.problem.day));

        let mut input_filepath = self.working_dir.clone();
        input_filepath.push(input_filename);
        input_filepath
    }

    /// Return the path to a file that may be used to cache correct solutions for this problem.
    ///
    /// By default, the path for day 1 part 1 will be `day1/day1-1.solution`. However, if
    /// `custom_input` is `Some("abc")` (for example), the path will instead be
    /// `day1/day1-1.abc.solution`.
    pub fn solution_file(&self, custom_input: &Option<String>) -> PathBuf {
        let solution_filename = custom_input
            .as_ref()
            .map(|input| format!("{}.{}.solution", self.problem, input))
            .unwrap_or(format!("{}.solution", self.problem));

        let mut solution_filepath = self.working_dir.clone();
        solution_filepath.push(solution_filename);
        solution_filepath
    }
}

/// Define a `main` function for solutions. Assumes the solution function ([`SolveFn`]) is named
/// `solve()`.
///
/// Example:
/// ```ignore
/// # // This doctest fails because it doesn't depend on `tracing_subscriber`
/// use std::fs::File;
/// use std::io::{BufRead, BufReader};
///
/// fn solve(reader: BufReader<File>) -> anyhow::Result<String> {
///     for line in reader.lines() {
///         tracing::info!("{line:?}");
///     }
///     Ok("123".into())
/// }
///
/// util::main!();
/// ```
#[macro_export]
macro_rules! main {
    () => {
        pub fn main() -> anyhow::Result<()> {
            tracing_subscriber::fmt::init();

            use std::path::PathBuf;
            use std::str::FromStr;
            use util::{Problem, Solution};
            let solution = Solution {
                problem: env!("CARGO_BIN_NAME").parse()?,
                solve_fn: solve,
                working_dir: PathBuf::from(env!("CARGO_MANIFEST_DIR")),
            };

            use util::cli::{SolutionCli, clap::Parser};
            let cli = SolutionCli::parse();
            cli.run(&solution)?;
            Ok(())
        }
    };
}
