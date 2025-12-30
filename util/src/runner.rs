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

    /// The filepath where a cached solution may be saved (e.g. `day1/day1-1.solution`)
    pub solution_filepath: PathBuf,
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
    }

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
            use util::runner::{RunArgs, run};
            use util::{input_filepath, solution_filepath};
            let args = SolutionCli::parse();

            match args.command() {
                Command::Solve(solve_args) => {
                    run(&RunArgs {
                        solve_fn: solve,
                        input_filepath: input_filepath!(solve_args),
                        solution_filepath: solution_filepath!(solve_args),
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
