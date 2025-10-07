use std::{cmp::Reverse, collections::BinaryHeap};

use crate::{
    parking_lot::{CounterParkingLot, ParkingLot},
    simulator::Simulator,
    triangular_distribution::TriangularPdfSampler,
};

pub struct EventSimulator {
    lot: CounterParkingLot,
    clock: u32,
    steps: u32,
    arrival_times: Vec<u32>,
    arrival_index: usize,
    departure_times: BinaryHeap<Reverse<u32>>,
    incoming: usize,
    pdf: TriangularPdfSampler,
    cars_per_second: f32,
}

impl EventSimulator {
    pub fn new<R: rand::Rng>(
        capacity: usize,
        max_time: u32,
        steps: u32,
        cars_per_hour: f32,
        skew: bool,
        rng: &mut R,
    ) -> Self {
        let mut sim = Self {
            lot: CounterParkingLot::new(capacity),
            steps,
            clock: 0,
            arrival_times: Vec::new(),
            arrival_index: 0,
            departure_times: BinaryHeap::new(),
            incoming: 0,
            pdf: TriangularPdfSampler::new(0.0, (max_time / 2) as f32, max_time as f32, skew),
            cars_per_second: cars_per_hour / 3600.0,
        };

        sim.precompute_arrivals(rng);

        sim
    }

    /// Precomputes arrival times using a geometric distribution to generate time between events,
    /// where the number of cars per second is the chance of the event happening.
    /// This function guarantees that two cars will not arrive in the same second,
    /// i.e. there will never be a need to add a check for whether there are multiple arrivals
    /// for a single timestamp.
    fn precompute_arrivals<R: rand::Rng>(&mut self, rng: &mut R) {
        let mut clock = 0;
        let ln_1_p = (1.0 - self.cars_per_second).ln();
        while clock < self.steps {
            // Calculate the time until the next car arrival
            let x = rng.random::<f32>();
            let delta_time = ((x.ln() / ln_1_p) as i64).max(0) as u32;
            // Increment clock by that time
            clock = clock.saturating_add(delta_time);
            if clock >= self.steps {
                break;
            }
            self.arrival_times.push(clock);
            // Advance by one additional second so that no two cars arrive at the same time
            clock = clock.saturating_add(1);
        }
    }

    /// Generates a departure time for a car, add it to the [`Self::departure_times`] list.
    /// Does **not** change [`Self::incoming`].
    fn park_car<R: rand::Rng>(&mut self, rng: &mut R) {
        // Generate a departure time and add it to the heap
        let departure_time = self.clock + self.pdf.sample(rng);
        let _ = self.lot.try_park(departure_time);
        self.departure_times.push(Reverse(departure_time));
    }

    /// Handles any departures for a given timestamp.
    fn handle_departures(&mut self, timestamp: u32) {
        while let Some(&Reverse(time)) = self.departure_times.peek() {
            if time != timestamp {
                break;
            }

            self.departure_times.pop();
            self.lot.remove_index(0);
        }
    }
}

impl Simulator for EventSimulator {
    fn simulate<T: rand::Rng>(&mut self, rng: &mut T) {
        // Whether there are currently any queued cars, and the lot has space for them to park.
        // Used to make sure that we don't skip time when there is an empty spot available
        // for a queued car
        let mut cars_can_park = false;

        while self.clock < self.steps {
            let next_arrival = self.arrival_times.get(self.arrival_index).copied();
            let next_departure = self.departure_times.peek().map(|r| r.0);

            // Find the next event and its timestamp
            let (mut next_time, mut has_arrival, mut has_departure) =
                match (next_arrival, next_departure) {
                    // Both an arrival and departure are available; pick the one that comes first
                    (Some(arrival), Some(departure)) => {
                        let min = arrival.min(departure);
                        (min, arrival == min, departure == min)
                    }
                    (Some(arrival), None) => (arrival, true, false),
                    (None, Some(departure)) => (departure, false, true),
                    // No events left; end the simulation
                    (None, None) => break,
                };

            // If there are already cars waiting to park, and we have an empty spot,
            // then just increase the clock by one tick
            if cars_can_park {
                let actual_next_time = self.clock + 1;
                // If the next event doesn't also happen at this time, then there will not be an
                // arrival nor a departure
                if next_time != actual_next_time {
                    has_arrival = false;
                    has_departure = false;
                }
                next_time = actual_next_time;
            }

            self.clock = next_time;

            // If we have an arrival, add a car to the queue and move to the next arrival time
            if has_arrival {
                self.arrival_index += 1;
                self.incoming += 1;
            }

            // Handle any departures for this timestamp
            if has_departure {
                self.handle_departures(self.clock);
            }

            // Park a car in the queue if there is space
            if self.incoming != 0 && self.lot.can_park() {
                self.park_car(rng);
                self.incoming -= 1;
                // If there are more cars and we have space for them to park,
                // set this flag so that we don't accidentally skip too much time
                cars_can_park = self.incoming != 0 && self.lot.can_park();
            }
        }
    }

    fn cars_left(&self) -> usize {
        self.incoming
    }
}
