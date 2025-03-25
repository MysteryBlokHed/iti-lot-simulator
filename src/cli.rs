use clap::Parser;

#[derive(Parser)]
#[command(version, about = "Rust rewrite of ITI 1121 assignment 3.", long_about = None)]
pub struct Cli {
    #[arg(help = "The number of cars that should enter the lot per hour. Must be positive.")]
    pub cars_per_hour: f32,
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
