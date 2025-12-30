use std::path::PathBuf;

use clap::{Args, Parser};

pub use clap;

#[derive(Args, Default)]
pub struct SolveArgs {
    /// Custom input file to use.
    ///
    /// For example, `--input test` will use `day1.test.input`.
    #[arg(short = 'i', long = "input")]
    pub input: Option<String>,
}

/// Plugs a package's own `CARGO_MANIFEST_DIR` and `CARGO_PKG_NAME` into [`SolveArgs::input_filepath`]
/// to construct a proper path for that package.
#[macro_export]
macro_rules! input_filepath {
    ($args:expr) => {
        $args.input_filepath(env!("CARGO_MANIFEST_DIR"), env!("CARGO_PKG_NAME"))
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
}

#[derive(Parser)]
pub struct SolutionCli {
    #[clap(flatten)]
    pub solve_args: SolveArgs,
}
