use crate::{
    parking_lot::ParkingLot,
    random_generator,
    triangular_distribution::{TriangularPdf, TriangularPdfSampler},
};

enum Pdf {
    Discrete(TriangularPdf),
    Continuous(TriangularPdfSampler),
}

pub struct Simulator<P: ParkingLot> {
    lot: P,
    clock: u32,
    steps: u32,
    max_time: u32,
    incoming: usize,
    pdf: Pdf,
    cars_per_second: f32,
}

impl<P: ParkingLot> Simulator<P> {
    pub fn new(
        lot: P,
        max_time: u32,
        steps: u32,
        cars_per_hour: f32,
        continuous: bool,
        skew: bool,
    ) -> Self {
        Self {
            lot,
            steps,
            max_time,
            clock: 0,
            incoming: 0,
            pdf: if continuous {
                Pdf::Continuous(TriangularPdfSampler::new(
                    0.0,
                    (max_time / 2) as f32,
                    max_time as f32,
                    skew,
                ))
            } else {
                Pdf::Discrete(TriangularPdf::new(0, max_time / 2, max_time))
            },
            cars_per_second: cars_per_hour / 3600.0,
        }
    }

    pub fn simulate<T: rand::Rng>(&mut self, rng: &mut T) {
        while self.clock < self.steps {
            // Determine whether car likely arrives
            let car_arrived = random_generator::event_occurred(rng, self.cars_per_second);
            if car_arrived {
                self.incoming += 1;
            }

            // Iterate over cars in lot
            let mut to_remove = Vec::new();
            for (i, &leave_time) in self.lot.iter().enumerate() {
                // Discrete probability mode (assignment)
                if let Pdf::Discrete(pdf) = &self.pdf {
                    let duration = self.clock - leave_time;

                    let car_leaves = if duration == self.max_time {
                        true
                    } else {
                        random_generator::event_occurred(rng, pdf.pdf(duration))
                    };

                    if car_leaves {
                        to_remove.push(i);
                    }
                }
                // Continuous probability mode
                else if self.clock == leave_time {
                    to_remove.push(i);
                }
            }

            // Remove cars marked for deletion
            for index in to_remove.into_iter().rev() {
                self.lot.remove_index(index);
            }

            // Try to park car in queue
            if self.incoming != 0 && self.lot.can_park() {
                // Continuous probability mode
                if let Pdf::Continuous(pdf) = &self.pdf {
                    // Randomly determine the leave time for this car
                    let leave_time = pdf.sample(rng) + self.clock;
                    let _ = self.lot.try_park(leave_time);
                }
                // Discrete probability mode (assignment)
                else {
                    let _ = self.lot.try_park(self.clock);
                }
                self.incoming -= 1;
            }

            // Tick
            self.clock += 1;
        }
    }

    pub fn cars_left(&self) -> usize {
        self.incoming
    }
}
