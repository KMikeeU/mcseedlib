
use crate::biomes::biome;
use crate::random::simplexnoisesampler::SimplexNoiseSampler;
use crate::random::chunkrandom::ChunkRandom;
use crate::random::jrandom::AbstractRandom;
use crate::random::jrandom::Random;

pub struct TheEndBiomeSource {
    seed: u64,
    pub noise: SimplexNoiseSampler
}

impl TheEndBiomeSource {
    pub fn new(seed: u64) -> Self {
        let random = Random::new(seed);
        let mut chunk_random = ChunkRandom::new(seed, random);
        chunk_random.skip(17292);

        TheEndBiomeSource {
            seed: seed,
            noise: SimplexNoiseSampler::new(chunk_random)
        }
    }

    pub fn get_biome(&self, x: i32, y: i32, z: i32, noise: &SimplexNoiseSampler) -> biome::TheEnd {
        let i: i32 = x >> 2;
        let j: i32 = z >> 2;

        if i * i + j * j <= 4096 {
            return biome::TheEnd::Center;
        } else {
            let f: f32 = Self::get_noise_at(&self.noise, i * 2 + 1, j * 2 + 1);

            if f > 40.0 {
                return biome::TheEnd::Highlands;
            } else if f >= 0.0 {
                return biome::TheEnd::Midlands;
            } else {
                if f < -20.0 {
                    return biome::TheEnd::SmallIslands;
                } else {
                    return biome::TheEnd::Barrens;
                }
            }
        }
    }

    pub fn get_noise_at(noise: &SimplexNoiseSampler, i: i32, j: i32) -> f32 {
        let k: i32 = (i as f32 / 2.0).floor() as i32;
        let l: i32 = (j as f32 / 2.0).floor() as i32;
        let m: i32 = i % 2;
        let n: i32 = j % 2;

        let mut f: f32 = 100.0 - ((i * i + j * j) as f32).sqrt() * 8.0;
        f = f.clamp(-100.0, 80.0);

        for o in -12..13 {
            for p in -12..13 {
                let q: i64 = (k + o) as i64;
                let r: i64 = (l + p) as i64;

                if q * q + r * r > 4096 && noise.sample_2d(q as f64, r as f64) < -0.8999999761581421 {
                    let g: f32 = (q.abs() as f32 * 3439.0 + r.abs() as f32 * 147.0) % 13.0 + 9.0;
                    let h: f32 = (m - o * 2) as f32;
                    let s: f32 = (n - p * 2) as f32;
                    let mut t: f32 = 100.0 - (h * h + s * s).sqrt() * g;
                    t = t.clamp(-100.0, 80.0);
                    f = if t > f { t } else { f };
                }
            }
        }

        f
    }
}