### 2025 [advent of code](https://adventofcode.com/) solutions

advent of code solutions. if the `$AOC_SESSION_COOKIE` env var is set, solutions can be submitted
and input files can be downloaded automatically; keep reading for more detail.

### daily problem setup

each day has a package in the repo-wide Cargo workspace. create package for a new day with a script:
```
$ scripts/new_day.sh 5
```

a `day5` package will be created with a library and two binaries:
- `day5`, a library which is built from `day5/src/lib.rs`
- `day5-1`, a binary which is built from `day5/src/part_1.rs`
- `day5-2`, a binary which is built from `day5/src/part_2.rs`

if the `$AOC_SESSION_COOKIE` env var is set, the script will download day 5's input to
`day5/day5.input`. otherwise, you should put the input file there yourself.

`part_1.rs` and `part_2.rs` will both contain a skeleton for a solution:
```
use std::fs::File;
use std::io::{BufRead, BufReader};

use day5::*;

fn solve(reader: BufReader<File>) -> anyhow::Result<String> {
    for line in reader.lines() {
        tracing::trace!("{line:?}");
    }

    panic!("not implemented");
}

util::main!();
```

all you have to do is fill in the implementation for `solve()`. read the input file from the provided
`reader` handle and return the result as a `String`. code that `part_1.rs` and `part_2.rs` both use
can be written in the `day5` library.

the `util::main!();` at the end wires the `solve()` function into a pluggable CLI defined in the
`util` crate.

### running solutions

each day has two binaries which you can run like so:
```
$ # run the solution for day 5 part 1
$ cargo run --bin day5-1

$ # run the solution for day 5 part 2
$ cargo run --bin day5-2

$ # run the solution for day 5 part 2 with extra logs
$ # (zoom to the end of the `less` buffer and then back up to avoid "broken pipe" spew)
$ RUST_LOG=trace cargo run --bin day5-2 | less -R
```

### custom test inputs

each binary can accept an optional `--input <test>` argument which will cause the problem input to be
read from a different file. for example:
```
$ # run the solution for day 5 part 1 on `day5/day5.abc.input`
$ cargo run --bin day5-1 -- --input abc

$ # run the solution for day 5 part 2 on `day5/day5.testing.input`
$ cargo run --bin day5-2 -- --input testing
```

### downloading inputs

if the `$AOC_SESSION_COOKIE` env var is set, solution binaries can be given a `download-input` command
which will fetch the day's input file from AoC and save it in `dayX/dayX.input`.
```
$ # download the input file for day 3 to `day3/day3.input`
$ export AOC_SESSION_COOKIE="session=<redacted>"
$ cargo run --bin day3-1 -- download-input
```

the `scripts/new_day.sh` script runs this automatically if `$AOC_SESSION_COOKIE` is set.

### submitting results

if the `$AOC_SESSION_COOKIE` env var is set, solution binaries can be given an optional `--submit`
argument which will send the solution's output to AoC. if the output is correct, it will be cached
locally in `dayX/dayX-Y.solution`.
```
$ # run the solution for day 5 part 2 and submit to AoC
$ # if the solution is correct, save it to `day5/day5-2.solution`
$ export AOC_SESSION_COOKIE="session=<redacted>"
$ cargo run --bin day5-2 -- --submit
```

### caching solutions

after running the solution implementation, each solution binary will check for a cached correct
solution. if found, it will be compared to the output of the current run, and submission to AoC
will be skipped.

the file where a cached solution is stored also depends on the `--input` argument used for custom
test cases:
```
$ # run the solution for day 5 part 2.
$ # use `day5/day5.input` as the input file.
$ # check for a cached solution in `day5/day5-2.solution`
$ cargo run --bin day5-2

$ # run the solution for day 5 part 2.
$ # use `day5/day5.abc.input` as the input file.
$ # check for a cached solution in `day5/day5-2.abc.solution`
$ cargo run --bin day5-2 -- --input abc
```
