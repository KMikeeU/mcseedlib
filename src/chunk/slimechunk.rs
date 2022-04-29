
use crate::random::jrandom::*;
use std::num::Wrapping;

pub struct SlimeChunk {
    seed: u64,
    scrambler: i64
}

impl SlimeChunk {
    pub fn new(world_seed: u64) -> SlimeChunk {
        return SlimeChunk {
            seed: world_seed,
            scrambler: 987234911
        }
    }

    pub fn is_slime_chunk(&self, chunk_x: i32, chunk_z: i32) -> bool {
        let chunk_x = Wrapping(chunk_x as i64);
        let chunk_z = Wrapping(chunk_z as i64);
        let seed = Wrapping(self.seed as i64);
        let scrambler = Wrapping(self.scrambler as i64);

        let mut random = Random::new(
            (
                seed
                + (chunk_x * chunk_x * Wrapping(4987142 as i64))
                + chunk_x * Wrapping(5947611 as i64)
                + chunk_z * chunk_z * Wrapping(4392871)
                + (chunk_z * Wrapping(389711)) ^ scrambler
            ).0 as u64
        );

        random.next_i32_bound(10) == 0
    }


}