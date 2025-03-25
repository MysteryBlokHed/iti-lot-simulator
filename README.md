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

## License

This project is licensed under the GNU General Public License, Version 3.0
([LICENSE](LICENSE) or <https://www.gnu.org/licenses/gpl-3.0.en.html>).
