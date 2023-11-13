use bitvec::prelude::Lsb0;
use bitvec::vec::BitVec;
use bitvec::view::BitView;

const M: u64 = 2u64.pow(32u32);
const A: u32 = 65537; // 2^16+1
const C: u32 = 119;

pub fn high(num: u32, add_bytes: usize) -> BitVec<u32> {
    let mut vec = num.view_bits().to_bitvec();
    let mut num = num;
    for _ in 0..add_bytes {
        num = ((A as u64 * num as u64 + C as u64) % M) as u32;
        ((num >> 24) as u8)
            .view_bits::<Lsb0>()
            .iter()
            .for_each(|each| vec.push(*each));
    }
    vec
}

pub fn low(num: u32, add_bytes: usize) -> BitVec<u32> {
    let mut vec = num.view_bits().to_bitvec();
    let mut num = num;
    for _ in 0..add_bytes {
        num = ((A as u64 * num as u64 + C as u64) % M) as u32;

        (num as u8)
            .view_bits::<Lsb0>()
            .iter()
            .for_each(|each| vec.push(*each));
    }
    vec
}
