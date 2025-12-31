use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};

pub use clap;

#[derive(Args, Clone)]
pub struct SolveArgs {
    /// Custom input file to use.
    ///
    /// For example, `--input test` will use `day1.test.input`.
    #[arg(short, long)]
    pub input: Option<String>,

    /// Advent of Code session cookie. If set, the solution will be submitted to AOC.
    #[arg(long)]
    pub session_cookie: Option<String>,
}

/// Plugs a package's own `CARGO_MANIFEST_DIR` and `CARGO_PKG_NAME` into [`SolveArgs::input_filepath`]
/// to construct a proper path for that package.
#[macro_export]
macro_rules! input_filepath {
    ($args:expr) => {
        $args.input_filepath(env!("CARGO_MANIFEST_DIR"), env!("CARGO_PKG_NAME"))
    };
}

/// Plugs a binary's own `CARGO_MANIFEST_DIR` and `CARGO_BIN_NAME` into [`SolveArgs::input_filepath`]
/// to construct a proper path for that package.
#[macro_export]
macro_rules! solution_filepath {
    ($args:expr) => {
        $args.solution_filepath(env!("CARGO_MANIFEST_DIR"), env!("CARGO_BIN_NAME"))
    };
}

impl SolveArgs {
    /// Given a package's manifest directory and package name, construct an input filepath for that
    /// package based on [the `--input` argument](SolveArgs::input).
    pub fn input_filepath(&self, manifest_dir: &str, package: &str) -> PathBuf {
        let input_filename = self
            .input
            .as_ref()
            .map(|input| format!("{}.{}.input", package, input))
            .unwrap_or(format!("{}.input", package));
        PathBuf::from_iter(&[manifest_dir, &input_filename])
    }

    pub fn solution_filepath(&self, manifest_dir: &str, binary: &str) -> PathBuf {
        let solution_filename = self
            .input
            .as_ref()
            .map(|input| format!("{}.{}.solution", binary, input))
            .unwrap_or(format!("{}.solution", binary));
        PathBuf::from_iter(&[manifest_dir, &solution_filename])
    }
}

#[derive(Subcommand, Clone)]
pub enum Command {
    /// Run a solution implementation.
    Solve(SolveArgs),

    /// Download problem input.
    DownloadInput {
        #[arg(long)]
        session_cookie: String,
    },
}

#[derive(Parser)]
pub struct SolutionCli {
    #[command(subcommand)]
    command: Option<Command>,

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
}
