use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

pub type SolveFn = fn(BufReader<File>) -> anyhow::Result<String>;

pub struct RunArgs {
    /// The solution implementation function to run.
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

    /// The input filepath (e.g. `day1/day1.input`).
    pub input_filepath: PathBuf,
}

/// Run a solution function according to [`RunArgs`].
pub fn run(args: &RunArgs) -> anyhow::Result<String> {
    let input_file = File::open(&args.input_filepath)?;
    let reader = BufReader::new(input_file);

    let solution = (args.solve_fn)(reader)?;

    tracing::info!("Solution finished: {solution}");
    Ok(solution)
}

/// Define a `main` function for solutions.
#[macro_export]
macro_rules! main {
    () => {
        pub fn main() -> anyhow::Result<()> {
            tracing_subscriber::fmt::init();
            use util::aoc::Aoc;
            use util::cli::{Command, SolutionCli, clap::Parser};
            use util::input_filepath;
            use util::runner::{RunArgs, run};
            let args = SolutionCli::parse();

            match args.command() {
                Command::Solve(solve_args) => {
                    run(&RunArgs {
                        solve_fn: solve,
                        input_filepath: input_filepath!(solve_args),
                    })?;
                }
                Command::DownloadInput { session_cookie } => {
                    let filepath = concat!(
                        env!("CARGO_MANIFEST_DIR"),
                        "/",
                        env!("CARGO_PKG_NAME"),
                        ".input"
                    );
                    Aoc::new(&session_cookie)?.download_input(env!("CARGO_PKG_NAME"), filepath)?;
                }
            }

            Ok(())
        }
    };
}
