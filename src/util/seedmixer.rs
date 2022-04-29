// Unused as of now.

pub struct SeedMixer {

}

impl SeedMixer {
    fn mix_seed(seed: u64, salt: u64) -> u64 {
        let seed = Wrapping(seed);
        let salt = Wrapping(salt);

        seed = seed * seed * Wrapping(6364136223846793005) + Wrapping(1442695040888963407);
        seed = seed + salt;
        seed
    }
}