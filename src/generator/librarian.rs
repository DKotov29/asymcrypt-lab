use bitvec::prelude::*;

pub fn convert(s: &str) -> BitVec<u8> {
    s.as_bytes().view_bits().to_bitvec()
}
