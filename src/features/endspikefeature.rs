
use crate::random::jrandom::*;
use crate::random::shuffle::*;

use std::fmt::Display;

pub struct EndSpikeFeature {
    pub spike_seed: u64
}

impl EndSpikeFeature {
    pub fn new(world_seed: u64) -> EndSpikeFeature {
        EndSpikeFeature {
            spike_seed: (Random::new(world_seed).next_i64() & 65535) as u64
        }
    }

    pub fn generate(&self) -> Vec<EndSpike> {
        let mut height_map: Vec<i32> = (0..10).collect();
        
        let random = Random::new(self.spike_seed);

        height_map.shuffle_java(random);
        let mut spikes: Vec<EndSpike> = Vec::with_capacity(10);

        for (i, l) in height_map.iter().enumerate() {
            spikes.push(EndSpike{
                x: (42.0 * ( 2.0 * (-3.141592653589793 + 0.3141592653589793 * (i as f64)) ).cos()).floor() as i32,
                z: (42.0 * ( 2.0 * (-3.141592653589793 + 0.3141592653589793 * (i as f64)) ).sin()).floor() as i32,
                radius: 2 + l / 3,
                height: 76 + l * 3,
                boxed: *l == 1 || *l == 2
            });
        }

        return spikes;
    }
}

pub struct EndSpike {
    x: i32,
    z: i32,
    radius: i32,
    height: i32,
    boxed: bool
}

impl Display for EndSpike {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Spike -- x:{}, y:{}, z:{}, b:{}, r:{}", self.x, self.height, self.z, self.boxed, self.radius)
    }
}