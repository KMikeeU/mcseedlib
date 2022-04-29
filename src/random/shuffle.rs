use crate::random::jrandom::*;

pub trait ShuffleJava {
    fn shuffle_java(&mut self, random: Random);
}

impl<T> ShuffleJava for Vec<T> {
    fn shuffle_java(&mut self, mut random: Random) {
        // TODO implement case if size < SHUFFLE_THRESHOLD

        for i in (1..self.len()).rev() {
            self.swap(i, random.next_i32_bound( (i + 1).try_into().unwrap()).try_into().unwrap());
        }
    }
}