use bitvec::prelude::*;

pub fn generate(array: BitArray<u32>, amount: usize) -> Option<BitVec<u32>> {
    //bits after 2^19 will be not used
    if array.len() < 20 {
        return None;
    }
    let mut vec = array[0..20].to_bitvec();
    for i in 0..amount {
        vec.push(vec[vec.len()-3] ^ vec[vec.len()-5] ^ vec[vec.len()-9] ^ vec[vec.len()-20]);
    }
    Some(vec)
}