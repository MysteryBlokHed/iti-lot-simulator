use clap::Parser;

#[allow(clippy::struct_excessive_bools)]
#[derive(Parser)]
#[command(version, about = "A Rust reimplementation of one of my assignments.", long_about = None)]
pub struct Cli {
    /// The number of cars that should enter the lot per hour. Must be positive.
    pub cars_per_hour: f32,

    /// The number of runs to do per capacity. More runs will take longer but produce more stable results.
    #[arg(
        short,
        long,
        default_value_t = 10,
        value_parser = clap::value_parser!(u32).range(1..)
    )]
    pub runs: u32,

    /// The maximum number of cars that are allowed to be waiting to enter in order for a capacity to be considered acceptable.
    #[arg(short, long, default_value_t = 5.0)]
    pub threshold: f32,

    /// Use a continuous probability sampling method that is faster and actually correct.
    #[arg(short, long)]
    pub continuous: bool,

    /// Use a heap-based structure for the continuous probability method.
    /// This flag implies --continuous.
    #[arg(short = 'p', long, conflicts_with = "event_based")]
    pub continuous_heap: bool,

    /// Instead of simulating every single tick, precompute the arrival and departure times,
    /// and then jump to the target simulation times.
    /// This flag implies --continuous.
    #[arg(short, long)]
    pub event_based: bool,

    /// For use with --continuous. Determines whether the random number generator should be skewed to somewhat match the incorrect discrete probabilities.
    #[arg(short, long, requires = "continuous")]
    pub skew: bool,

    /// Uses a binary search approach to determine the best capacity, instead of just increasing by one constantly.
    #[arg(short, long)]
    pub binary_search: bool,

    /// The maximum amount of time a car will stay in the lot, in seconds. Defaults to 8 hours.
    #[arg(short, long, default_value_t = 28800)]
    pub max_stay: u32,

    /// The duration of time to simulate the lot for, in seconds. Defaults to 24 hours.
    #[arg(short, long, default_value_t = 86400)]
    pub duration: u32,

    /// Prints information about each simulation run rather than just the final result.
    #[arg(short, long)]
    pub verbose: bool,

    /// Uses an implementation that closely matches the assignment description, rather than just a functionally identical one.
    /// This only exists as a baseline to compare how much more performant the optimized code is.
    #[arg(
        long,
        // Note: The help text is manually specified here so that the line break is preserved.
        help="Uses an implementation that closely matches the assignment description, rather than just a functionally identical one.
This only exists as a baseline to compare how much more performant the optimized code is",
        conflicts_with_all(["continuous", "binary_search"])
    )]
    pub faithful: bool,
}
