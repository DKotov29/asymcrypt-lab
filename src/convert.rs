use bitvec::vec::BitVec;

pub struct WrapperBitVec<T>
where
    T: bitvec::store::BitStore,
{
    pub array: BitVec<T>,
}

impl<T: bitvec::store::BitStore> WrapperBitVec<T> {
    pub fn new(array: BitVec<T>) -> WrapperBitVec<T> {
        WrapperBitVec { array }
    }
}

impl<T: bitvec::store::BitStore> From<BitVec<T>> for WrapperBitVec<T> {
    fn from(value: BitVec<T>) -> Self {
        WrapperBitVec::new(value)
    }
}

// if it will take so long time, change to vec from raw parts or smth like this
impl<T> From<WrapperBitVec<T>> for Vec<u8>
where
    T: bitvec::store::BitStore,
{
    fn from(value: WrapperBitVec<T>) -> Self {
        let b = value.array.to_bitvec();
        let d = b.as_bitptr().pointer() as *const u8;
        let mut leen = b.len();
        let mut start = 0isize;
        let mut vec = Vec::with_capacity((leen / 8) + 1);
        while leen >= 8 {
            let on_pos = unsafe { *(d.offset(start)) };
            vec.push(on_pos);
            leen -= 8;
            start += 1;
        }
        if leen > 0 {
            unreachable!("something left to read, so something failed");
        }
        vec
    }
}
