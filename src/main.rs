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

const RUNS_PER_SIZE: u32 = 10;
const THRESHOLD: f32 = 5.0;
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
    smallest: Arc<Mutex<usize>>,
    done: Arc<RwLock<bool>>,
    capacity: usize,
) {
    let average = par_simulate_capacity(capacity, cli.cars_per_hour, cli.continuous, cli.skew);
    if average <= THRESHOLD {
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
        .for_each(|capacity| par_simulate_inner(cli, smallest.clone(), done.clone(), capacity));

    *smallest.lock().unwrap()
}

fn par_simulate_capacity(capacity: usize, cars_per_hour: f32, continuous: bool, skew: bool) -> f32 {
    let inner_loop = |rng: &mut rand::rngs::ThreadRng, i: u32| {
        let mut sim = Simulator::new(
            VecParkingLot::new(capacity),
            // ArrayParkingLot::new(capacity),
            8 * 3600,
            24 * 3600,
            cars_per_hour,
            continuous,
            skew,
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

    let final_size_sum = (1..=RUNS_PER_SIZE)
        .into_par_iter()
        .map_init(rand::rng, inner_loop)
        .sum::<usize>();

    (final_size_sum as f32) / (RUNS_PER_SIZE as f32)
}

fn main() {
    let cli = cli::Cli::parse();
    assert!(
        cli.cars_per_hour > 0.0,
        "There must be a positive number of cars per hour."
    );

    let start_time = Instant::now();

    let capacity = par_simulate(&cli);
    let end_time = Instant::now();
    let runtime = end_time - start_time;
    eprintln!(
        "\nSIMULATION IS COMPLETE!\nThe smallest number of parking spots required: {capacity}\nTotal execution time: {:.3} seconds",
        runtime.as_secs_f32(),
    );
}
