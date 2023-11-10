use bitvec::prelude::Lsb0;
use bitvec::vec::BitVec;
use bitvec::view::BitView;

const M: u64 = 4294967296; // 2^32
const A: u64 = 65537; // 2^16+1
const C: u64 = 119;

pub fn high(mut num: u64, add_bytes: usize) -> BitVec<u64> {
    let mut vec = num.view_bits().to_bitvec();
    for _ in 0..add_bytes {
        num = ((A * num + C) % M) & 0xFF;
        num.view_bits::<Lsb0>()
            .iter()
            .take(8)
            .for_each(|each| vec.push(*each));
    }
    vec
}

pub fn low(mut num: u64, add_bytes: usize) -> BitVec<u64> {
    let mut vec = num.view_bits().to_bitvec();
    for _ in 0..add_bytes {
        num = (((A * num + C) % M) & 0xFF000000) >> 24;
        num.view_bits::<Lsb0>()
            .iter()
            .take(8)
            .for_each(|each| vec.push(*each));
    }
    vec
}
