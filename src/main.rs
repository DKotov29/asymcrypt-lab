mod generator;

use bitvec::macros::internal::funty::Fundamental;
use bitvec::prelude::*;
use bitvec::view::BitViewSized;

fn main() {
    // let mut buildin = generator::buildin::generate();
    // println!("buildin: {}, len: {}", buildin, buildin.len());
    // buildin.set(0, false);
    // buildin.set(1, true);
    // println!("buildin: {}, len: {}", buildin, buildin.len());

    // println!("              {}", 211u64.view_bits::<Lsb0>());
    // let high=generator::lehmer::high(211u64, 2);
    // println!("len: {}, vec: {}", high.len(), high) ;

    // let num = 0b0000101010000000010101u32;
    // let arr = num.into_bitarray::<Lsb0>();
    // println!("len: {}, {arr}", arr.len());
    // let m = generator::l20::generate(arr, 5);
    // println!("len: {}, {m}", m.len());

    let c: char = char::from(0b1001);
    println!("\"{c}\"");
}

// тести приймають послідовності байтів