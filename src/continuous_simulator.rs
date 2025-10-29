use std::{cmp::Reverse, collections::BinaryHeap};

use crate::{
    random_generator, simulator::Simulator, triangular_distribution::TriangularPdfSampler,
};

pub struct ContinuousHeapSimulator {
    occupancy: usize,
    capacity: usize,
    clock: u32,
    steps: u32,
    departure_times: BinaryHeap<Reverse<u32>>,
    incoming: usize,
    pdf: TriangularPdfSampler,
    cars_per_second: f32,
}

impl ContinuousHeapSimulator {
    pub fn new(capacity: usize, max_time: u32, steps: u32, cars_per_hour: f32, skew: bool) -> Self {
        Self {
            occupancy: 0,
            capacity,
            clock: 0,
            steps,
            departure_times: BinaryHeap::new(),
            incoming: 0,
            pdf: TriangularPdfSampler::new(0.0, (max_time / 2) as f32, max_time as f32, skew),
            cars_per_second: cars_per_hour / 3600.0,
        }
    }

    fn can_park(&self) -> bool {
        self.occupancy != self.capacity
    }

    /// Generates a departure time for a car, add it to the [`Self::departure_times`] list.
    /// Does **not** change [`Self::incoming`].
    /// This function assumes that the caller has already checked that the lot is not full.
    fn park_car<R: rand::Rng>(&mut self, rng: &mut R) {
        // Generate a departure time and add it to the heap
        let departure_time = self.clock + self.pdf.sample(rng);
        self.occupancy += 1;
        self.departure_times.push(Reverse(departure_time));
    }

    /// Handles any departures for a given timestamp.
    fn handle_departures(&mut self, timestamp: u32) {
        while let Some(&Reverse(time)) = self.departure_times.peek() {
            if time != timestamp {
                break;
            }

            self.departure_times.pop();
            self.occupancy -= 1;
        }
    }
}

impl Simulator for ContinuousHeapSimulator {
    fn simulate<T: rand::Rng>(&mut self, rng: &mut T) {
        while self.clock < self.steps {
            // Determine whether car likely arrives
            let car_arrived = random_generator::event_occurred(rng, self.cars_per_second);
            if car_arrived {
                self.incoming += 1;
            }

            // Handle any departures for this timestamp
            self.handle_departures(self.clock);

            // Try to park car in queue
            if self.incoming != 0 && self.can_park() {
                self.park_car(rng);
                self.incoming -= 1;
            }

            self.clock += 1;
        }
    }

    fn cars_left(&self) -> usize {
        self.incoming
    }
}
