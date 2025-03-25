# Parking Lot Simulator

In my intro to computing class, one of our assignments was
to code a parking lot simulator in Java.
I thought that the assignment was rather poorly optimized,
so I decided to rewrite it in Rust and see how performant I could get it.

This repository just exists for me to track those changes separately,
in a public repository so that it's easier to share without just opening
the entire repository for that class's work.

## Description

The program takes in an average number of cars per hour from the command line.
It then simulates a parking lot over the course of 24 hours,
using discrete probability calculations to determine when cars arrive and leave the lot.

The original assignment had stipulations about the implementation
that I am not enforcing here for the sake of performance.

### Probability Errors

It's worth noting that the simulator as described in the assignment is actually not correct,
since it causes cars to leave disproportionately later than the distribution should imply.

A continuous flag (`-c`) is available for a more performant and more correct simulation.
The RNG for this version can be skewed (`-s`) to somewhat replicate the distribution
of the original probability method.

## Building

[Cargo](https://www.rust-lang.org/tools/install) is required to build.

```sh
cargo build --release
```

## Running

You can either use the built binary at `target/release/iti-lot-simulator`, or `cargo run --release --`.
Note the `--` at the end when using `cargo run`.

```sh
$ cargo run --release -- --help

A Rust reimplementation of one of my assignments.

Usage: iti-lot-simulator [OPTIONS] <CARS_PER_HOUR>

Arguments:
  <CARS_PER_HOUR>  The number of cars that should enter the lot per hour. Must be positive.

Options:
  -r, --runs <RUNS>            The number of runs to do per capacity. More runs will take longer but produce more stable results. [default: 10]
  -t, --threshold <THRESHOLD>  The maximum number of cars that are allowed to be waiting to enter in order for a capacity to be considered acceptable. [default: 5]
  -c, --continuous             Use a continuous probability sampling method that is faster and actually correct.
  -s, --skew                   For use with --continuous. Determines whether the random number generator should be skewed to somewhat match the incorrect discrete probabilities.
  -b, --binary-search          Uses a binary search approach to determine the best capacity, instead of just increasing by one constantly.
  -h, --help                   Print help
  -V, --version                Print version
```

```sh
$ cargo run --release -- 10

SIMULATION IS COMPLETE!
The smallest number of parking spots required: 55
Total execution time: 0.792 seconds
```

```sh
$ cargo run --release -- 10 -csb

SIMULATION IS COMPLETE!
The smallest number of parking spots required: 57
Total execution time: 0.029 seconds
```

## License

This project is licensed under the GNU General Public License, Version 3.0
([LICENSE](LICENSE) or <https://www.gnu.org/licenses/gpl-3.0.en.html>).
