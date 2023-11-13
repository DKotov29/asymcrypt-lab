use bitvec::array::BitArray;
use bitvec::order::Lsb0;
use bitvec::vec::BitVec;

use malachite::num::conversion::traits::FromStringBase;
use malachite::num::logic::traits::BitIterable;
use malachite::Natural;

pub fn main_main() {
    // let bitvec: BitVec<u32, Lsb0> = BitVec::from(BitArray::from(0b10100010100101010101011100010101u32));
    //
    // println!("{bitvec}");
    println!("{:b}", 0xABCD);
    let n = Natural::from_string_base(16, "ABCD").unwrap();
    let a = n.bits().take(8);

    println!();
    println!("{n:?}");
}
