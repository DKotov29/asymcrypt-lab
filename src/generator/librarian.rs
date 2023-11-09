use bitvec::prelude::*;

pub fn convert(s: String) -> BitVec<u8> {
    s.as_bytes().view_bits().to_bitvec()
}