mod convert;
mod criterion;
mod generator;
mod macros;

use crate::convert::WrapperBitVec;
use bitvec::macros::internal::funty::Fundamental;
use bitvec::prelude::*;
use bitvec::view::BitViewSized;

#[macro_use]
extern crate prettytable;
use prettytable::Table;

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

    let some_r = 2usize;
    let some_a = 0.1;

    // let generator_result = generator::buildin::generate();
    // let bitvec: WrapperBitVec<_> = generator_result.to_bitvec().into();
    // let byte_vec: Vec<u8> = bitvec.into();
    // test_n_print!(generator_result, some_r, some_a);
    // let res = criterion::sequence_homogeneity::test(byte_vec.as_slice(),some_r, some_a );
    // println!("{res:?}");

    let vec: Vec<u32> = vec![
        214, 168, 122, 77, 32, 244, 200, 157, 114, 72, 30, 245, 204, 163, 123, 84, 45, 6, 224, 186,
    ];
    let bitarr = BitSlice::from_slice(vec.as_slice());

    test_n_print!(bitarr, some_r, some_a, "own bytes");

    return;
}

// todo after every generator
/*if print_info:
        print("_____________Uniformity test_____________")
        print(f'{a=}')
        print(f'{actual_statistic_value=}')
        print(f'{critical_statistic_value=}')
        print("Test time: ", end-start)
*/
