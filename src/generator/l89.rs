use bitvec::prelude::*;

pub fn generate(array: BitVec<u32>, amount: usize) -> Option<BitVec<u32>> {
    if array.len() < 89 {
        return None;
    }
    let mut vec = array[0..89].to_bitvec();
    for i in 0..amount {
        vec.push(vec[vec.len() - 38] ^ vec[vec.len() - 89]);
    }
    Some(vec)
}
