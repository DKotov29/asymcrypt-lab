use bitvec::prelude::*;
use malachite::num::arithmetic::traits::{ModPow, Parity};
use malachite::Natural;
use num_bigint::BigUint;

pub fn generate(p: BigUint, q: BigUint, r: BigUint, amount: usize) -> Option<BitVec<u32>> {
    if r < BigUint::from(2u8) {
        return None;
    }
    let p = Natural::from_owned_limbs_asc(p.iter_u64_digits().collect::<Vec<_>>());
    let q = Natural::from_owned_limbs_asc(q.iter_u64_digits().collect::<Vec<_>>());
    let mut r = Natural::from_owned_limbs_asc(r.iter_u64_digits().collect::<Vec<_>>());
    let n = p * q;
    let mut vec: BitVec<u32> = BitVec::with_capacity(amount);
    for _ in 0..amount {
        r = r.mod_pow(&Natural::from(2u8), &n);
        vec.push(r.odd()); //check
    }
    Some(vec)
}
