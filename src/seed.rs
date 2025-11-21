use rand::{rngs::StdRng, Rng, SeedableRng};
use chrono::prelude::*;

pub struct DailySeed {
    index: usize,
    set: i32,
    length: usize
}

impl DailySeed {
    pub fn new() -> Self {
        let today = Utc::now();
        let seed = Self::genseed(today);
        let mut rng: StdRng = SeedableRng::seed_from_u64(seed);
        
        let daily_set = rng.random_range(0..=1);

        DailySeed { index: 0, set: daily_set, length: 0 }
    }
    fn genseed(date: DateTime<Utc>) -> u64 { // generates a seed based on date
        let seed = (date.year() as u64) << 16 | (date.month() as u64) << 8 | (date.day() as u64);
        seed
    }
    pub fn process(&mut self, length: usize) { // generates a range to be used as a maximum index limit
        self.length = length;
        let seed = Self::genseed(Utc::now());
        let mut rng: StdRng = SeedableRng::seed_from_u64(seed);
        self.index = rng.random_range(1..=length);
    }

    pub fn get_index(&self) -> usize { self.index }
    pub fn get_set(&self) -> i32 { self.set }
}