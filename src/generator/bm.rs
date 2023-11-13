use bitvec::prelude::*;
use num_bigint::BigUint;

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
    let mut vec: BitVec<u32> = BitVec::with_capacity(amount);
    for _ in 0..amount {
        if t < q {
            vec.push(true);
        } else {
            vec.push(false);
        }
        t = a.modpow(&t, &p);
    }
    Some(vec)
}
