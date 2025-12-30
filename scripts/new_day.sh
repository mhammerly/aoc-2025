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
use std::fs::File;
use std::io::{BufRead, BufReader};

use $day::*;

fn solve(reader: BufReader<File>) -> anyhow::Result<String> {
    for line in reader.lines() {
        tracing::trace!("{line:?}");
    }

    panic!("not implemented");
}

util::main!();
EOF
cp $day/src/part_1.rs $day/src/part_2.rs

if [ -n $aoc_cookie ]; then
    echo "Session cookie set, downloading input for $day"
    cargo run --bin $day-1 -- download-input --session-cookie $aoc_cookie
fi

# go back to wherever we were before
cd -
