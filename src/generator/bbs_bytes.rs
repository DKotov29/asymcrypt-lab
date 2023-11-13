use bitvec::prelude::*;
use malachite::num::arithmetic::traits::{Mod, ModPow};
use malachite::Natural;
use num_bigint::BigUint;
use num_integer::Integer;

pub fn generate(p: BigUint, q: BigUint, mut r: BigUint, amount: usize) -> Option<BitVec<u32>> {
    if r < BigUint::from(2u8) {
        return None;
    }
    let p = Natural::from_owned_limbs_asc(p.iter_u64_digits().collect::<Vec<_>>());
    let q = Natural::from_owned_limbs_asc(q.iter_u64_digits().collect::<Vec<_>>());
    let mut r = Natural::from_owned_limbs_asc(r.iter_u64_digits().collect::<Vec<_>>());

    let n = p * q;
    let mut vec: BitVec<u32> = BitVec::with_capacity(amount);
    for _ in 0..amount {
        r = r.clone().mod_pow(&Natural::from(2u8), &n);

        let x = r.clone().mod_op(&Natural::from(256u64));
        let bb = x.limbs().next().unwrap_or(0) as u8;
        if Natural::from(bb) != x {
            panic!("at the disco");
        }
        for bit in bb.view_bits::<Lsb0>(){
            vec.push(*bit);
        }
    }
    Some(vec)
}
