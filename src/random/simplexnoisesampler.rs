
// net.minecraft.util.math.noise

use crate::jrandom::AbstractRandom;

const GRADIENTS: [[i32;3]; 16] = [[1, 1, 0], [-1, 1, 0], [1, -1, 0], [-1, -1, 0], [1, 0, 1], [-1, 0, 1], [1, 0, -1], [-1, 0, -1], [0, 1, 1], [0, -1, 1], [0, 1, -1], [0, -1, -1], [1, 1, 0], [0, -1, 1], [-1, 1, 0], [0, -1, -1]];
const SQRT_3: f64 = 1.7320508075688772;
const SKEW_FACTOR_2D: f64 = 0.5 * (SQRT_3 - 1.0);
const UNSKEW_FACTOR_2D: f64 = (3.0 - SQRT_3) / 6.0;


pub struct SimplexNoiseSampler {
    pub origin_x: f64,
    pub origin_y: f64,
    pub origin_z: f64,
    permutations: [i32; 512]
}


impl SimplexNoiseSampler {
    // from java
    pub fn new<T: AbstractRandom>(mut random: T) -> Self {
        let mut noise = SimplexNoiseSampler {
            origin_x: random.next_f64() * 456.0,
            origin_y: random.next_f64() * 256.0,
            origin_z: random.next_f64() * 256.0,
            permutations: [0; 512],
        };

        for i in 0..256 {
            noise.permutations[i] = i as i32;
        }

        for i in 0..256 {
            let j = random.next_i32_bound(256 - i);
            let k = noise.permutations[i as usize];

            noise.permutations[i as usize] = noise.permutations[(j+i) as usize];
            noise.permutations[(j + i) as usize] = k;
        }

        return noise;
    }

    pub fn get_gradient(&self, hash: i32) -> i32 {
        self.permutations[(hash & 255) as usize]
    }

    pub fn dot(garr: [i32; 3], x: f64, y: f64, z: f64) -> f64 {
        garr[0] as f64 * x + garr[1] as f64 * y + garr[2] as f64 * z
    }

    fn grad(&self, hash: i32, x: f64, y: f64, z: f64, distance: f64) -> f64 {
        let mut d: f64 = distance - x * x - y * y - z * z;
        let e: f64;

        if d < 0.0 {
            e = 0.0;
        } else {
            d = d * d;
            e = d * d * Self::dot(GRADIENTS[hash as usize], x, y, z);
        }

        e
    }

    pub fn sample_2d(&self, x: f64, y: f64) -> f64{
        let d: f64 = (x + y) * SKEW_FACTOR_2D;
        let i = (x + d).floor() as i32;
        let j = (y + d).floor() as i32;
        let e = (i + j) as f64 * UNSKEW_FACTOR_2D;
        let f = i as f64 - e;
        let g = j as f64 - e;
        let h = x - f;
        let k = y - g;
        let l: i8;
        let m: i8;

        if h > k {
            l = 1;
            m = 0;
        } else {
            l = 0;
            m = 1;
        }

        let n = h - l as f64 + UNSKEW_FACTOR_2D;
        let o = k - m as f64 + UNSKEW_FACTOR_2D;
        let p = h - 1.0 + 2.0 * UNSKEW_FACTOR_2D;
        let q = k - 1.0 + 2.0 * UNSKEW_FACTOR_2D;
        let r = i & 255;
        let s = j & 255;
        let t = self.get_gradient(r + self.get_gradient(s)) % 12;
        let u = self.get_gradient(r + l as i32 + self.get_gradient(s + m as i32)) % 12;
        let v = self.get_gradient(r + 1 + self.get_gradient(s + 1)) % 12;
        
        let w  = self.grad(t, h, k, 0.0, 0.5);
        let z  = self.grad(u, n, o, 0.0, 0.5);
        let aa = self.grad(v, p, q, 0.0, 0.5);

        return 70.0 * (w + z + aa);
    }
}