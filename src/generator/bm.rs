use bitvec::prelude::*;
use malachite::Natural;
use malachite::num::arithmetic::traits::ModPow;
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
    let p = Natural::from_owned_limbs_asc(p.iter_u64_digits().collect::<Vec<_>>());
    let a = Natural::from_owned_limbs_asc(a.iter_u64_digits().collect::<Vec<_>>());
    let q = Natural::from_owned_limbs_asc(q.iter_u64_digits().collect::<Vec<_>>());
    let mut t = Natural::from_owned_limbs_asc(t.iter_u64_digits().collect::<Vec<_>>());

    let mut vec: BitVec<u32> = BitVec::with_capacity(amount);
    for _ in 0..amount {

        if t < q {
            vec.push(true);
        } else {
            vec.push(false);
        }
        t = (&a).mod_pow(&t, &p);

    }
    Some(vec)
}
