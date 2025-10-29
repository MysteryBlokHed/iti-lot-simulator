# Parking Lot Simulator

In my intro to computing class, one of our assignments was
to code a parking lot simulator in Java.
I thought that the assignment was rather poorly optimized,
so I decided to rewrite it in Rust and see how performant I could get it.

This isn't a particularly useful project—it's just a fun experiment for myself.

## Description

The task is to implement a parking lot simulator that determines the minimum viable lot size
given some average number of cars entering the lot per hour.
The lot is simulated over the course of 24 hours, using one-second ticks.

The time that a car leaves the lot is determined by a [triangular distribution](https://en.wikipedia.org/wiki/Triangular_distribution),
where $`a = 0, c = \frac{\textrm{max stay}}{2}, b = \textrm{max stay}`$.  
The max stay is 8 hours by default.

The code that is prescribed to be run each tick is:

1. Generate a random number to determine if a car should join the queue
   (done by comparing a random number to the ratio $`\frac{\textrm{hourly cars}}{3600 \textrm{ seconds}}`$).
   - If the rng returns true, add a car to the incoming queue.
2. For each car currently in the lot:
   - Determine if the car should leave the lot by:
     1. Checking if the car has been there for the max duration; or
     2. Generating a random number and comparing it to the triangular PDF at the duration the car has been parked.
   - If a car should leave the lot, remove it from the parked cars list and move it to the outgoing cars queue.  
3. If the outgoing queue is not empty, pop one car.
4. If the incoming queue is not empty, and there is space in the lot, move a car from the queue to the parked cars list.

### Deviation from the Description

The default version of the code (no flags) is functionally identical to this description,
but I have made some optimizations such as parallelizing the simulations,
replacing the incoming queue with an integer counter, and removing the outgoing queue entirely.
If an implementation that is written as described above is desired, use the `--faithful` flag.
(The faithful version also does _not_ run in parallel).

Other performance optimizations may cause varying amounts of divergence from this implementation,
most notably with the `-c` and `-e` flags described below.

### Probability Errors

It's worth noting that the simulator as described in the assignment is actually not correct,
since it causes cars to leave disproportionately later than the distribution should imply.

A continuous flag (`-c`) is available for a more performant and more correct simulation.
Since the cars leave at the proper time, the calculated lot size will be smaller than the default version.
The RNG for this version can be skewed (`-s`) to somewhat replicate the distribution
of the original probability method.

### Heap-Based Departures

In the continuous version of the code, the departure times are all precomputed.
All we need to do each tick is to simply check whether there are any cars
that need to be removed from the lot by checking if the smallest departure time
is equal to the current tick.
This is a natural use case for a min-heap, so that's what the `-p` flag uses.

This change provided one of the most significant speedups.

### Event-Based Simulation

One might notice that the simulation has a pretty substantial number of RNG calls.
In particular, a random number is sampled every single tick to determine whether a car will arrive.
Instead, we can just use a geometric distribution to pre-compute every arrival time for cars,
and then skip time in our simulation when no events are happening.

This optimization provides a _slight_ speed boost on top of using a heap for departures.

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
  <CARS_PER_HOUR>  The number of cars that should enter the lot per hour. Must be positive

Options:
  -r, --runs <RUNS>            The number of runs to do per capacity. More runs will take longer but produce more stable results [default: 10]
  -t, --threshold <THRESHOLD>  The maximum number of cars that are allowed to be waiting to enter in order for a capacity to be considered acceptable [default: 5]
  -c, --continuous             Use a continuous probability sampling method that is faster and actually correct
  -p, --continuous-heap        Use a heap-based structure for the continuous probability method. This flag implies --continuous
  -e, --event-based            Instead of simulating every single tick, precompute the arrival and departure times, and then jump to the target simulation times. This flag implies --continuous
  -s, --skew                   For use with --continuous. Determines whether the random number generator should be skewed to somewhat match the incorrect discrete probabilities
  -b, --binary-search          Uses a binary search approach to determine the best capacity, instead of just increasing by one constantly
  -m, --max-stay <MAX_STAY>    The maximum amount of time a car will stay in the lot, in seconds. Defaults to 8 hours [default: 28800]
  -d, --duration <DURATION>    The duration of time to simulate the lot for, in seconds. Defaults to 24 hours [default: 86400]
  -v, --verbose                Prints information about each simulation run rather than just the final result
      --faithful               Uses an implementation that closely matches the assignment description, rather than just a functionally identical one.
                               This only exists as a baseline to compare how much more performant the optimized code is
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

```sh
$ cargo run --release -- 1000 -cb

SIMULATION IS COMPLETE!
The smallest number of parking spots required: 4085
Total execution time: 3.036 seconds
```

```sh
$ cargo run --release -- 1000 -be

SIMULATION IS COMPLETE!
The smallest number of parking spots required: 4089
Total execution time: 0.031 seconds
```

## Benchmarks

If you want to see how the different versions compare, you can use `cargo bench`.
To see all available benchmarks, run `cargo bench -- --list`.

Run `cargo bench defaults/` to see how the nine variations
(i.e. `--faithful`, no flags, `-c`, `-cp`, `-b`, `-bc`, `-bcp`, `-e`, `-be`)
compare for 3 cars per hour.

Run `cargo bench fifty/` to see how the versions other than faithful compare for 50 cars per hour.

## License

This project is licensed under the GNU General Public License, Version 3.0
([LICENSE](LICENSE) or <https://www.gnu.org/licenses/gpl-3.0.en.html>).
