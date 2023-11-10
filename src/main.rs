mod generator;
mod criterion;

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
    let res = criterion::sign_independency::test(vec![214,168,122,77, 32, 244,200,157,114,72,30,245,204,163,123,84,45,6,224,186], 0.01);
    println!("{res:?}");
    return;
    criterion::sign_independency::test(vec![], 5.0);
    return;
    // bitvec into byte vec
    let b = generator::buildin::generate();
    let mut cc = b.clone();
    cc.reverse();
    let cc: Vec<_> = cc.chunks(8).collect();
    for i in cc {
        println!("{i}");
    }
    let b = b.to_bitvec();
    let d = b.as_bitptr().pointer() as *const u8;
    println!("{d:?}");
    let mut leen = b.len();
    let mut start = 0isize;
    while leen >= 8 {
        let on_pos = unsafe {*(d.offset(start))};
        println!("{on_pos:#3} bin: {on_pos:08b}");
        leen -= 8;
        start+=1;
    }
    if leen > 0 {
        panic!("something left to read, so something failed");
    }
}
// todo after every generator
/*if print_info:
        print("_____________Uniformity test_____________")
        print(f'{a=}')
        print(f'{actual_statistic_value=}')
        print(f'{critical_statistic_value=}')
        print("Test time: ", end-start)
*/