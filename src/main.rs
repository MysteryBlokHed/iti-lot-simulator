#![warn(clippy::pedantic)]
#![allow(
    clippy::cast_lossless,
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss
)]
use clap::Parser;
use rayon::prelude::*;
use std::{
    sync::{Arc, Mutex, RwLock},
    time::Instant,
};

#[allow(unused_imports)]
use parking_lot::{ArrayParkingLot, VecParkingLot};
use simulator::Simulator;

mod cli;
mod parking_lot;
mod random_generator;
mod simulator;
mod triangular_distribution;

pub const MAX_CAPACITY: usize = 512;

/// An iterator that simply returns the results of the iterator it wraps,
/// until its `done` flag is set to `true`.
/// This is used to test multiple potential lot sizes at once.
///
/// Once a lot capacity that works is found, no larger capacities will be tested.
/// Any ongoing iterations will be allowed to continue
/// in case they find a smaller capacity that also works,
/// in which case that smaller capacity will be the one returned as a final answer.
struct IterUntilDone<I: Iterator<Item = usize>> {
    done: Arc<RwLock<bool>>,
    iterator: I,
}

impl<I: Iterator<Item = usize>> IterUntilDone<I> {
    fn new(iterator: I) -> Self {
        Self {
            done: Arc::new(RwLock::new(false)),
            iterator,
        }
    }
}

impl<I: Iterator<Item = usize>> Iterator for IterUntilDone<I> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if *self.done.read().unwrap() {
            return None;
        }

        self.iterator.next()
    }
}

fn par_simulate_inner(
    cli: &cli::Cli,
    smallest: &Arc<Mutex<usize>>,
    done: &Arc<RwLock<bool>>,
    capacity: usize,
) {
    let average = par_simulate_capacity(capacity, cli);
    if average <= cli.threshold {
        *done.write().unwrap() = true;
        let mut smallest = smallest.lock().unwrap();
        if capacity < *smallest {
            *smallest = capacity;
        }
    }
}

fn par_simulate(cli: &cli::Cli) -> usize {
    let smallest = Arc::new(Mutex::new(usize::MAX));
    let iter = IterUntilDone::new(1..);
    let done = iter.done.clone();

    iter.par_bridge()
        .for_each(|capacity| par_simulate_inner(cli, &smallest.clone(), &done.clone(), capacity));

    *smallest.lock().unwrap()
}

fn par_simulate_capacity(capacity: usize, cli: &cli::Cli) -> f32 {
    let inner_loop = |rng: &mut rand::rngs::ThreadRng, i: u32| {
        let mut sim = Simulator::new(
            VecParkingLot::new(capacity),
            // ArrayParkingLot::new(capacity),
            cli.max_stay,
            cli.duration,
            cli.cars_per_hour,
            cli.continuous,
            cli.skew,
        );
        let start = Instant::now();
        sim.simulate(rng);
        let end = Instant::now();
        let runtime = end - start;

        let cars_left = sim.cars_left();

        eprintln!(
            "Capacity {capacity}, simulation run {i} ({} ms): Queue length at the end of simulation run: {cars_left}",
            runtime.as_millis(),
        );

        cars_left
    };

    let final_size_sum = (1..=cli.runs)
        .into_par_iter()
        .map_init(rand::rng, inner_loop)
        .sum::<usize>();

    (final_size_sum as f32) / (cli.runs as f32)
}

fn binary_search_simulate(cli: &cli::Cli) -> usize {
    // Start by doubling the tested capacity until we reach one that works
    let mut upper_bound = 1usize;
    loop {
        let average = par_simulate_capacity(upper_bound, cli);
        if average <= cli.threshold {
            break;
        }
        upper_bound <<= 1;
    }

    let lower_bound = (upper_bound >> 1) + 1;

    // Binary search
    let mut low = lower_bound;
    let mut high = upper_bound;
    let mut mid;

    while low <= high {
        mid = (high + low) / 2;
        // Run the simulation
        let average = par_simulate_capacity(mid, cli);
        let too_high = average <= cli.threshold;

        // Try smaller capacities if we overestimated, larger if we underestimated
        if too_high {
            high = mid - 1;
        } else {
            low = mid + 1;
        }
    }

    // Whatever our `low` value is at the end of the loop is the result
    low
}

fn main() {
    let cli = cli::Cli::parse();
    assert!(
        cli.cars_per_hour > 0.0,
        "There must be a positive number of cars per hour."
    );
    assert!(
        cli.threshold > 0.0,
        "The threshold must be a positive number."
    );

    let start_time = Instant::now();

    let capacity = if cli.binary_search {
        binary_search_simulate(&cli)
    } else {
        par_simulate(&cli)
    };

    let end_time = Instant::now();
    let runtime = end_time - start_time;

    eprintln!(
        "\nSIMULATION IS COMPLETE!\nThe smallest number of parking spots required: {capacity}\nTotal execution time: {:.3} seconds",
        runtime.as_secs_f32(),
    );
}
