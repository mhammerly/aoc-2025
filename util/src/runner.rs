use std::fs::File;
use std::io::{BufReader, Write};
use std::path::PathBuf;

use crate::{
    Problem,
    aoc::{Aoc, AocResult},
};

pub type SolveFn = fn(BufReader<File>) -> anyhow::Result<String>;

/// Problem-specific parameters that can plug into an otherwise generic solution runner.
pub struct RunArgs {
    /// The solution implementation function ([`SolveFn`]) to run.
    ///
    /// Example:
    /// ```
    /// # use std::fs::File;
    /// # use std::io::{BufRead, BufReader};
    /// fn solve(reader: BufReader<File>) -> anyhow::Result<String> {
    ///     for line in reader.lines() {
    ///         tracing::info!("{line:?}");
    ///     }
    ///     Ok("".into())
    /// }
    /// ```
    pub solve_fn: SolveFn,

    /// The [`Problem`] whose solution is being run (e.g. day1-1, day3-2).
    pub problem: Problem,

    /// The input filepath (e.g. `day1/day1.input`).
    pub input_filepath: PathBuf,

    /// The filepath where a cached solution may be saved (e.g. `day1/day1-1.solution`)
    pub solution_filepath: PathBuf,

    /// Advent of Code client. Will submit solutions if set.
    pub aoc_client: Option<Aoc>,
}

/// Run a solution function according to [`RunArgs`].
pub fn run(args: &RunArgs) -> anyhow::Result<String> {
    let input_file = File::open(&args.input_filepath)?;
    let reader = BufReader::new(input_file);

    tracing::info!("Running solution on `{:?}`", &args.input_filepath);
    let solution = (args.solve_fn)(reader)?;
    tracing::info!("Solution finished: {solution}");

    if let Ok(cached_solution) = std::fs::read_to_string(&args.solution_filepath) {
        tracing::info!("Cached solution found in `{:?}`", &args.solution_filepath);
        let cached_solution = cached_solution.trim();
        if cached_solution == solution {
            tracing::info!("Correct! (`{}` == `{}`)", solution, cached_solution);
        } else {
            tracing::error!("Incorrect! (`{}` != `{}`)", solution, cached_solution);
        }
    } else if let Some(aoc) = &args.aoc_client {
        tracing::info!("Submitting solution to AOC");
        let aoc_result = aoc.submit(&args.problem, &solution)?;
        if aoc_result == AocResult::Correct {
            let mut file = File::create(&args.solution_filepath)?;
            write!(file, "{}", &solution)?;
        } else {
            tracing::error!("Incorrect! (`{}`)", solution);
        }
    }

    Ok(solution)
}
