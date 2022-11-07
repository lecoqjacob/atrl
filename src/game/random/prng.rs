use crate::game::prelude::*;
use std::ops::RangeBounds;

use rand::RngCore;
use rand::{Rng as RandRng, SeedableRng};
use rand_pcg::Pcg64;

#[derive(Serialize, Deserialize)]
pub struct Prng {
    seed: u64,
    rng: Pcg64,
}

impl Prng {
    pub fn new(seed: u64) -> Self { Self { seed, rng: Pcg64::seed_from_u64(seed) } }

    pub fn from_entropy() -> Self { Self::new(Pcg64::from_entropy().next_u64()) }

    pub fn advance(&mut self, delta: u128) -> &mut Self {
        self.rng.advance(delta);
        self
    }

    pub fn seed(&self) -> u64 { self.seed }

    pub fn coin(&mut self) -> bool { self.max_inclusive(1) == 1 }

    pub fn next(&mut self) -> u32 { self.rng.next_u32() }

    pub fn max(&mut self, max: u32) -> u32 {
        if max == 0 {
            return max;
        }
        self.max_inclusive(max - 1)
    }

    pub fn max_inclusive(&mut self, max: u32) -> u32 {
        if max == 0 {
            return max;
        }

        let mut x;
        let top = max as u64 + 1;
        let buckets = u32::MAX as u64 / top;
        let limit = buckets * top;

        loop {
            x = self.next() as u64;
            if x < limit {
                break;
            }
        }

        (x / buckets) as u32
    }

    pub fn range<R: RangeBounds<u32>>(&mut self, range: R) -> u32 {
        let (start, end) = get_range_bounds(range, u32::MIN, u32::MAX);

        self.max_inclusive(end - start) + start
    }

    pub fn next_u64(&mut self) -> u64 { self.rng.next_u64() }

    pub fn max_u64(&mut self, max: u64) -> u64 {
        if max == 0 {
            return max;
        }
        self.max_inclusive_u64(max - 1)
    }

    pub fn max_inclusive_u64(&mut self, max: u64) -> u64 {
        if max == 0 {
            return max;
        }

        let mut x;
        let top = max as u128 + 1;
        let buckets = u64::MAX as u128 / top;
        let limit = buckets * top;

        loop {
            x = self.next_u64() as u128;
            if x < limit {
                break;
            }
        }

        (x / buckets) as u64
    }

    pub fn range_u64<R: RangeBounds<u64>>(&mut self, range: R) -> u64 {
        let (start, end) = get_range_bounds(range, u64::MIN, u64::MAX);

        self.max_inclusive_u64(end - start) + start
    }

    pub fn next_f32(&mut self) -> f32 { (self.next() as f64 / (u32::MAX as u64 + 1) as f64) as f32 }

    pub fn next_f64(&mut self) -> f64 {
        (self.next_u64() as f64 / (u64::MAX as u128 + 1) as f64) as f64
    }

    pub fn entropy() -> u32 { Pcg64::from_entropy().gen::<u32>() }

    pub fn entropy_u64() -> u64 { Pcg64::from_entropy().gen::<u64>() }

    pub fn entropy_f32() -> f32 {
        (Pcg64::from_entropy().gen::<u32>() as f64 / (u32::MAX as u64 + 1) as f64) as f32
    }

    pub fn entropy_f64() -> f64 {
        (Pcg64::from_entropy().gen::<u64>() as f64 / (u64::MAX as u128 + 1) as f64) as f64
    }
}
