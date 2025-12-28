use std::collections::hash_map::RandomState;
use std::hash::{BuildHasher, Hasher};

fn random_u64() -> u64 {
    let random_state = RandomState::new();
    let hasher = random_state.build_hasher();
    hasher.finish()
}

fn random_range(min: usize, max: usize) -> usize {
    let random = random_u64();
    min + (random as usize % (max - min))
}

pub fn grey_scale() -> [f32; 3] {
    let palette = [
        [0.0, 0.0, 0.0], // Black
        [0.2, 0.2, 0.2], // Very dark gray
        [0.4, 0.4, 0.4], // Dark gray
        [0.6, 0.6, 0.6], // Medium gray
        [0.8, 0.8, 0.8], // Light gray
        [1.0, 1.0, 1.0], // White
    ];
    palette[random_range(0, 5)]
}

#[derive(Clone)]
pub struct Face {
    pub v: [usize; 3],
    pub vt: [usize; 3],
    pub vn: [usize; 3],
    pub g_scale: [f32; 3],
}

impl Face {
    pub fn new(v: [usize; 3], vt: [usize; 3], vn: [usize; 3]) -> Self {
        Face {
            v,
            vt,
            vn,
            g_scale: grey_scale(),
        }
    }
}
