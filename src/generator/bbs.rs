use bitvec::prelude::*;
use num_bigint::BigUint;

pub fn generate(p: BigUint, q: BigUint, mut r: BigUint, amount: usize) -> Option<BitVec<u32>> {
    if r < BigUint::from(2u8)  {
        return None;
    }
    let n = p*q;
    let mut vec: BitVec<u32> = BitVec::with_capacity(amount);
    for _ in 0..amount {
        r = r.modpow(&BigUint::from(2u8), &n);
        vec.push(r.modpow(&BigUint::from(1u8), &BigUint::from(2u8)) == BigUint::from(1u8)); // check
    }
    Some(vec)
}