use clap::Parser;

#[derive(Parser)]
#[command(version, about = "Rust rewrite of ITI 1121 assignment 3.", long_about = None)]
pub struct Cli {
    #[arg(help = "The number of cars that should enter the lot per hour. Must be positive.")]
    pub cars_per_hour: f32,
    #[arg(
        short,
        long,
        help = "The number of runs to do per capacity. More runs will take longer but produce more stable results.",
        default_value_t = 10,
        value_parser = clap::value_parser!(u32).range(1..)
    )]
    pub runs: u32,
    #[arg(
        short,
        long,
        help = "The maximum number of cars that are allowed to be waiting to enter in order for a capacity to be considered acceptable.",
        default_value_t = 5.0
    )]
    pub threshold: f32,
    #[arg(
        short,
        long,
        help = "Use a continuous probability sampling method that is faster and actually correct."
    )]
    pub continuous: bool,
    #[arg(
        short,
        long,
        requires = "continuous",
        help = "For use with --continuous. Determines whether the random number generator should be skewed to somewhat match the incorrect discrete probabilities."
    )]
    pub skew: bool,
    #[arg(
        short,
        long,
        help = "Uses a binary search approach to determine the best capacity, instead of just increasing by one constantly."
    )]
    pub binary_search: bool,
}
