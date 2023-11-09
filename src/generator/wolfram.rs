use bitvec::prelude::*;
use num_integer::Integer;

pub fn generate(start: u32, amount: usize) -> Option<BitVec<u32>> {
    let mut vec: BitVec<u64> = BitVec::with_capacity(amount);
    let mut num = start;
    for _ in 0..amount {

        vec.push(num.is_odd()); // check
        num = (num.rotate_left(1)) ^ (num | (num.rotate_right(1)));
    }
    todo!()
}