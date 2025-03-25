use crate::MAX_CAPACITY;

pub type Spot = u32;

#[allow(dead_code)]
pub trait ParkingLot {
    fn can_park(&self) -> bool;
    #[must_use]
    fn try_park(&mut self, timestamp: u32) -> bool;
    fn get_occupancy(&self) -> usize;
    fn remove_index(&mut self, index: usize) -> Spot;
    fn iter(&self) -> impl Iterator<Item = &Spot>;
}

/// Parking lot implementation that uses a [`Vec`] for its lot.
pub struct VecParkingLot {
    pub occupancy: Vec<Spot>,
    pub capacity: usize,
}

impl VecParkingLot {
    #[inline]
    #[allow(dead_code)]
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity,
            occupancy: Vec::with_capacity(capacity),
        }
    }
}

impl ParkingLot for VecParkingLot {
    #[inline]
    fn can_park(&self) -> bool {
        self.occupancy.len() != self.capacity
    }

    fn try_park(&mut self, timestamp: u32) -> bool {
        if !self.can_park() {
            return false;
        }
        self.occupancy.push(timestamp);
        true
    }

    #[inline]
    fn remove_index(&mut self, index: usize) -> Spot {
        self.occupancy.swap_remove(index)
    }

    #[inline]
    fn get_occupancy(&self) -> usize {
        self.occupancy.len()
    }

    #[inline]
    fn iter(&self) -> impl Iterator<Item = &Spot> {
        self.occupancy.iter()
    }
}

/// Parking lot implementation that uses a fixed-size array for its lot (i.e. no heap allocation).
/// Maximum capacity is defined in `main.rs`.
pub struct ArrayParkingLot {
    pub occupancy: [Spot; MAX_CAPACITY],
    pub length: usize,
    pub capacity: usize,
}

impl ArrayParkingLot {
    #[inline]
    #[allow(dead_code)]
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity,
            length: 0,
            occupancy: [0; MAX_CAPACITY],
        }
    }
}

impl ParkingLot for ArrayParkingLot {
    #[inline]
    fn can_park(&self) -> bool {
        self.length != self.capacity
    }

    fn try_park(&mut self, timestamp: u32) -> bool {
        if !self.can_park() {
            return false;
        }
        self.occupancy[self.length] = timestamp;
        self.length += 1;
        true
    }

    fn remove_index(&mut self, index: usize) -> Spot {
        let old = self.occupancy[index];
        self.length -= 1;
        self.occupancy[index] = self.occupancy[self.length];
        old
    }

    #[inline]
    fn get_occupancy(&self) -> usize {
        self.length
    }

    #[inline]
    fn iter(&self) -> impl Iterator<Item = &Spot> {
        self.occupancy[0..self.length].iter()
    }
}
