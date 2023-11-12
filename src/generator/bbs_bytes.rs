use bitvec::prelude::*;
use num_bigint::BigUint;
use num_integer::Integer;

pub fn generate(p: BigUint, q: BigUint, mut r: BigUint, amount: usize) -> Option<BitVec<u32>> {
    if r < BigUint::from(2u8) {
        return None;
    }
    let n = p * q;
    let mut vec: BitVec<u32> = BitVec::with_capacity(amount);
    for _ in 0..amount {
        r = r.modpow(&BigUint::from(2u8), &n);
        let x = r.modpow(&BigUint::from(1u8), &BigUint::from(256u16));

        let bb = *x.to_bytes_le().first().unwrap();
        if BigUint::from(bb) != x {
            panic!("at the disco");
        }
        for bit in bb.view_bits::<Lsb0>(){
            vec.push(*bit);
        }
    }
    Some(vec)
}
