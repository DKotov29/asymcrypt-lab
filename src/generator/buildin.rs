use bitvec::prelude::*;
use rand::Rng;

pub fn generate() -> BitArray<u32, Lsb0> {
    let mut rng = rand::thread_rng();
    let num: u32 = rng.gen();
    num.into_bitarray()
}