# Advent of Code 2020 in Rust

These are my solutions for [Advent of Code 2020](https://adventofcode.com/2020/) written in [Rust](https://www.rust-lang.org/).

#### To run solution for a given day: (day 01 below)

```
./run.sh 01
```
Optionally `print` can be passed as the second argument for debug output.
Also `timeit` can be passed as the second argument where the solution will be run 10-100 times and measured as avg.

#### To run tests for a solution:

```
./test.sh 01
```
Optionally `print` can be passed as the second argument for debug output here as well.

#### Prerequisites

* Rust installation (tested with v1.43.1), see: https://www.rust-lang.org/tools/install
* For test script [cargo-watch](https://github.com/passcod/cargo-watch) is used. Just `cargo install cargo-watch` is needed.

#### Benchmarks

Benchmarks obtained using:
```
./benchmark.sh
```

**CPU:** Intel(R) Core(TM) i7-8750H CPU @ 2.20GHz

| Day | Least runtime both parts |
| --- | ------------------------:|
| 01 | 7.843306ms |
| 02 | 0.610408ms |
| 03 | 0.062872ms |
| 04 | 0.67993ms |
| 05 | 0.226303ms |
| 06 | 0.613039ms |
| 07 | 3.319827ms |
| 08 | 1.220359ms |
| 09 | 0.462822ms |
| 10 | 0.012944ms |
| 11 | 42.627841ms |
| 12 | 0.064288ms |
| 13 | 0.01835ms |
| 14 | 5.615599ms |
| 15 | 640.904474ms |
| 16 | 0.570139ms |
| 17 |  |
| 18 |  |
| 19 |  |
| 20 |  |
| 21 |  |
| 22 |  |
| 23 |  |
| 24 |  |
| 25 |  |
