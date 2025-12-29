### 2025 [advent of code](https://adventofcode.com/) solutions

put day 1's input file in `day1/day1.input`, day 2's input file in `day2/day2.input`, and so on.
run a solution like so:

```
$ # run the solution for day 1 part 1
$ cargo run --bin day1-1
$ # run the solution for day 1 part 2
$ cargo run --bin day1-2

$ # run the solution for day 1 part 2 on `day1/day1.abc.input`
$ cargo run --bin day1-2 -- --input=abc

$ # run the solution for day 2 part 1 with more logs
$ # (zoom to the bottom of the output in `less` or it will spew broken pipe errors)
$ RUST_LOG=trace cargo run --bin day2-1 | less -R
```
