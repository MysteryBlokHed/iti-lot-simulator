use std::collections::VecDeque;

use crate::{
    parking_lot::ParkingLot, random_generator, simulator::Simulator,
    triangular_distribution::TriangularPdf,
};

pub struct FaithfulSimulator<P: ParkingLot> {
    lot: P,
    clock: u32,
    steps: u32,
    max_time: u32,
    incoming: VecDeque<u32>,
    outgoing: VecDeque<u32>,
    pdf: TriangularPdf,
    cars_per_second: f32,
}

impl<P: ParkingLot> FaithfulSimulator<P> {
    pub fn new(lot: P, max_time: u32, steps: u32, cars_per_hour: f32) -> Self {
        Self {
            lot,
            steps,
            max_time,
            clock: 0,
            incoming: VecDeque::new(),
            outgoing: VecDeque::new(),
            pdf: TriangularPdf::new(0, max_time / 2, max_time),
            cars_per_second: cars_per_hour / 3600.0,
        }
    }
}

impl<P: ParkingLot> Simulator<P> for FaithfulSimulator<P> {
    fn simulate<T: rand::Rng>(&mut self, rng: &mut T) {
        while self.clock < self.steps {
            // Determine whether car likely arrives
            let car_arrived = random_generator::event_occurred(rng, self.cars_per_second);
            if car_arrived {
                self.incoming.push_back(self.clock);
            }

            // Iterate over cars in lot
            let mut to_remove = Vec::new();
            for (i, &leave_time) in self.lot.iter().enumerate() {
                let duration = self.clock - leave_time;

                let car_leaves = if duration == self.max_time {
                    true
                } else {
                    random_generator::event_occurred(rng, self.pdf.pdf(duration))
                };

                if car_leaves {
                    to_remove.push(i);
                }
            }

            // Remove cars marked for deletion and add them to the outgoing queue
            for &index in to_remove.iter().rev() {
                self.lot.remove_index(index);
                self.outgoing.push_back(self.clock);
            }

            // Pop elements from the outgoing queue, if there are any.
            // You may notice that the outgoing queue's entire existence is essentially a noop.
            // This, I assume, is a remnant from how logging worked in the _previous_ assignment that simply wasn't removed
            self.outgoing.pop_front();

            // Try to park car in queue
            if !self.incoming.is_empty() && self.lot.can_park() {
                let _ = self.lot.try_park(self.clock);
                self.incoming.pop_front();
            }

            // Tick
            self.clock += 1;
        }
    }

    fn cars_left(&self) -> usize {
        self.incoming.len()
    }
}
