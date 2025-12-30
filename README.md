### 2025 [advent of code](https://adventofcode.com/) solutions

advent of code solutions which can interact with the site using a session cookie.

put day 1's input file in `day1/day1.input` and so on. you can provide a binary with
an additional `--input abc` argument to use `day1/day1.abc.input` instead for custom
test cases.

run like so:
```
$ # run the solution for day 1 part 1
$ cargo run --bin day1-1

$ # run the solution for day 2 part 2 with `day2/day2.test.input`
$ cargo run --bin day2-2 -- --input test

$ # download the input file for day 3
$ export aoc_cookie="session=<redacted>"
$ cargo run --bin day3-1 -- download-input --session-cookie $aoc_cookie

$ # run the solution for day 4 part 1 with additional logs
$ # (zoom to the bottom of the `less` feed and then back up or it will spew broken pipe errors)
$ RUST_LOG=trace cargo run --bin day4-1 | less -R
```

# caching solutions
the correct solution for the `day1-1` binary with the default input file can be saved
in `day1/day1-1.solution`. if, for example, `--input test` is provided, the
corresponding solution file would be `day1/day1-1.test.solution`. if a solution file
is found, the runner will compare its latest result against the content of the saved
solution.
