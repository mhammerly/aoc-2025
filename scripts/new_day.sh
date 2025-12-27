#!/bin/sh

day="day$1"

repo_root=$(git rev-parse --show-toplevel)

# go to git repository root to create our new day
cd $repo_root

# create the cargo project and fill in its `Cargo.toml`
cargo new $day
cat << EOF > $day/Cargo.toml
[package]
name = "$day"
version = "0.1.0"
edition = "2024"

[lib]
name = "$day"
path = "src/lib.rs"

[[bin]]
name = "$day-1"
path = "src/part_1.rs"

[[bin]]
name = "$day-2"
path = "src/part_2.rs"

[dependencies]
util = { workspace = true }

anyhow = { workspace = true }
thiserror = { workspace = true }
tracing = { workspace = true }
tracing-subscriber = { workspace = true }
EOF

# placeholder test files (need to fill in manually)
touch $day/$day.input
touch $day/$day.test.input

# create starter `lib.rs`, `part_1.rs`, and `part_2.rs`
touch $day/src/lib.rs
cat << EOF > $day/src/part_1.rs
use std::env;
use std::fs::File;
use std::io::{BufReader, prelude::*};

use $day::*;

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let input_file = File::open(format!("{}/$day.input", env!("CARGO_MANIFEST_DIR")))?;
    let _reader = BufReader::new(input_file);

    panic!("not implemented");
}
EOF
cp $day/src/part_1.rs $day/src/part_2.rs

# go back to wherever we were before
cd -
