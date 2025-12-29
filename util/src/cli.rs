use std::path::PathBuf;

pub use clap;

#[derive(clap::Parser)]
pub struct Args {
    #[arg(short = 'i', long = "input")]
    pub input: Option<String>,
}

/// Plugs a package's own `CARGO_MANIFEST_DIR` and `CARGO_PKG_NAME` into [`Args::input_filepath`]
/// to construct a proper path for that package.
#[macro_export]
macro_rules! input_filepath {
    ($args:ident) => {
        $args.input_filepath(env!("CARGO_MANIFEST_DIR"), env!("CARGO_PKG_NAME"))
    };
}

impl Args {
    /// Given a package's manifest directory and package name, construct an input filepath for that
    /// package based on [the `--input` argument](Args::input).
    pub fn input_filepath(&self, manifest_dir: &str, package: &str) -> PathBuf {
        let input_filename = self
            .input
            .as_ref()
            .map(|input| format!("{}.{}.input", package, input))
            .unwrap_or(format!("{}.input", package));
        PathBuf::from_iter(&[manifest_dir, &input_filename])
    }
}
