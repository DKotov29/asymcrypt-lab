use bitvec::prelude::*;
use rand::Rng;

pub fn generate(amount: usize) -> BitVec<u8> {
    let mut vec = BitVec::with_capacity(amount.checked_mul(8usize).unwrap_or(usize::MAX));
    let mut rng = rand::thread_rng();
    for _ in 0..amount {
        let num: u8 = rng.gen();
        num.view_bits::<Lsb0>()
            .iter()
            .for_each(|each| vec.push(*each));
    }
    vec
}
