#[inline]
pub fn event_occurred<T: rand::Rng>(random: &mut T, probability: f32) -> bool {
    random.random::<f32>() < probability
}
