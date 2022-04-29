
use crate::jrandom::{Random, AbstractRandom};
use std::num::Wrapping;

// NOTE
// java baseRandom is random (!) here!!
// base_random currenty unused!

pub struct ChunkRandom {
    sample_count: u32,
    random: Random,
    base_random: Random,
    seed: u64
}

// this is vastly stupid
impl AbstractRandom for ChunkRandom {
    fn set_seed(&mut self, seed: u64){
        self.random.set_seed(seed);
    }
    fn next_i32(&mut self) -> i32 {
        self.random.next_i32()
    }
    fn next_i32_bound(&mut self, bound: i32) -> i32 {
        self.random.next_i32_bound(bound)
    }
    fn next_i64(&mut self) -> i64 {
        self.random.next_i64()
    }
    fn next_f32(&mut self) -> f32 {
        self.random.next_f32()
    }
    fn next_f64(&mut self) -> f64 {
        self.random.next_f64()
    }      
}

impl ChunkRandom {
    pub fn new(seed: u64, base_random: Random) -> Self {
        ChunkRandom {
            random: Random::new(seed),
            base_random,
            sample_count: 0,
            seed
        }
    }

    pub fn set_population_seed(&mut self, world_seed: u64, blockx: i32, blockz: i32) -> u64 {
        self.random.set_seed(world_seed);
        
        let l = Wrapping(self.random.next_i64() | 1);
        let m = Wrapping(self.random.next_i64() | 1);
        let n = 
            (Wrapping(blockx as i64) * l
            + Wrapping(blockz as i64) * m ^ Wrapping(world_seed as i64)).0 as u64;
        
        self.random.set_seed(n);
        return n;
    }

    pub fn set_decorator_seed(&mut self, population_seed: u64, index: i32, step: i32) {
        let l = population_seed + index as u64 + (10000 * step as u64);
        self.random.set_seed(l);
    }

    pub fn set_carver_seed(&mut self, world_seed: u64, chunkx: i32, chunkz: i32) {
        self.random.set_seed(world_seed);

        let l = Wrapping(self.random.next_i64());
        let m = Wrapping(self.random.next_i64());
        let n = Wrapping(chunkx as i64) * l ^ Wrapping(chunkz as i64) * m ^ Wrapping(world_seed as i64);

        self.random.set_seed(n.0 as u64);
    }
}