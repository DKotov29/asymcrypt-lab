use bitvec::prelude::*;

pub fn generate(
    mut array_x: BitVec<u32>,
    mut array_y: BitVec<u32>,
    mut array_s: BitVec<u32>,
    amount: usize,
) -> Option<BitVec<u32>> {
    if array_x.len() != 11 || array_y.len() != 9 || array_s.len() != 10 {
        return None;
    }
    fn next_x(array: &mut BitVec<u32>) -> bool {
        let returnn = array[0];
        array.push(array[0] ^ array[2]);
        array.remove(0);
        returnn
    }

    fn next_y(array: &mut BitVec<u32>) -> bool {
        let returnn = array[0];
        array.push(array[0] ^ array[1] ^ array[3] ^ array[4]);
        array.remove(0);
        returnn
    }

    fn next_s(array: &mut BitVec<u32>) -> bool {
        let returnn = array[0];
        array.push(array[0] ^ array[3]);
        array.remove(0);
        returnn
    }

    let mut vec = BitVec::new(); // it is z

    for _ in 0..amount {
        let (next_x, next_y, next_s) = (
            next_x(&mut array_x),
            next_y(&mut array_y),
            next_s(&mut array_s),
        );
        vec.push(next_s & next_x ^ (true ^ next_s) & next_y);
    }
    Some(vec)
}
