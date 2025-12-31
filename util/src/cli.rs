use clap::{Args, Parser, Subcommand};

use crate::{
    Solution,
    aoc::Aoc,
    runner::{RunArgs, run},
};

pub use clap;

/// Command line arguments related to the [`Command::Solve`] command.
#[derive(Args, Clone)]
pub struct SolveArgs {
    /// Custom input file to use.
    ///
    /// For example, `--input test` will use `day1.test.input`.
    #[arg(short, long)]
    pub input: Option<String>,

    /// Whether to submit the solution to Advent of Code.
    ///
    /// Expects `$AOC_SESSION_COOKIE` env var to be set; see [`crate::aoc::Aoc`].
    #[arg(short, long)]
    #[clap(default_value_t = false)]
    pub submit: bool,
}

/// Actions that an AoC solution binary can perform.
#[derive(Subcommand, Clone)]
pub enum Command {
    /// Run a solution implementation.
    Solve(SolveArgs),

    /// Download problem input.
    ///
    /// Expects `$AOC_SESSION_COOKIE` env var to be set; see [`crate::aoc::Aoc`].
    DownloadInput,
}

/// CLI for Advent of Code solutions.
///
/// Supports:
/// - downloading input files from AoC
/// - running solutions locally
/// - submitting solutions to AoC
/// - saving correct solutions locally
///
/// Provide an AoC session cookie via the `$AOC_SESSION_COOKIE` environment variable. The cookie
/// must begin with `session=`.
#[derive(Parser)]
pub struct SolutionCli {
    /// The command that should be run for this invocation. If not specified, the `command()`
    /// method implementation will default to [`Command::Solve`].
    #[command(subcommand)]
    command: Option<Command>,

    /// The arguments for the [`Command::Solve`] command, but accepted as top-level arguments. This
    /// allows solution binaries to omit the `solve` command.
    #[clap(flatten)]
    solve_args: SolveArgs,
}

impl SolutionCli {
    /// If a command was specified, returns it. Otherwise, uses the top-level [`SolveArgs`] options
    /// to create a [`Command::Solve`] command.
    pub fn command(&self) -> Command {
        self.command
            .clone()
            .unwrap_or(Command::Solve(self.solve_args.clone()))
    }

    /// Run the CLI for an Advent of Code solution.
    pub fn run(&self, solution: &Solution) -> anyhow::Result<String> {
        match self.command() {
            Command::Solve(solve_args) => {
                let aoc_client = solve_args.submit.then_some(Aoc::new()).transpose()?;
                run(&RunArgs {
                    problem: solution.problem.clone(),
                    solve_fn: solution.solve_fn,
                    input_filepath: solution.input_file(&solve_args.input),
                    solution_filepath: solution.solution_file(&solve_args.input),
                    aoc_client,
                })?;
            }
            Command::DownloadInput => {
                Aoc::new()?.download_input(&solution.problem, solution.input_file(&None))?;
            }
        }
        Ok("".into())
    }
}
