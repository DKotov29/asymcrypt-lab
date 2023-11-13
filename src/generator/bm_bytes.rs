use bitvec::prelude::*;
use num_bigint::BigUint;
use num_integer::Integer;
use std::ops::{Mul, Sub};

pub fn generate(
    p: BigUint,
    a: BigUint,
    q: BigUint,
    mut t: BigUint,
    amount: usize,
) -> Option<BitVec<u32>> {
    if (BigUint::from(2u8) * q.clone() + BigUint::from(1u8) != p)
        || t > p.clone() - BigUint::from(1u8)
    {
        return None;
    }
    let mut vec: BitVec<u32> = BitVec::with_capacity(amount * 8);
    for _ in 0..amount {
        let byte = t
            .clone()
            .mul(256usize)
            .div_floor(&p.clone().sub(BigUint::from(1u32)));
        let bb = *byte.to_bytes_le().first().unwrap();
        if BigUint::from(bb) != byte {
            panic!("at the disco2 (bm bytes gived more)");
        }
        for bit in bb.view_bits::<Lsb0>() {
            vec.push(*bit);
        }
        t = a.modpow(&t, &p);
    }
    Some(vec)
}
