use std::fmt::Debug;

use bitvec::vec::BitVec;

use super::Array;

pub trait PrimitiveType: Copy + Send + Sync + Default + Debug + 'static {}

pub struct PrimitiveArray<T: PrimitiveType> {
    /// The actual data of this array
    data: Vec<T>,
    /// The null bitmap of this array
    bitmap: BitVec,
}

impl<T: PrimitiveType> Array for PrimitiveArray<T> {
    type RefItem<'a> = T;

    fn get(&self, idx: usize) -> Option<Self::RefItem<'_>> {
        if self.bitmap[idx] {
            Some(self.data[idx])
        } else {
            None
        }
    }

    fn len(&self) -> usize {
        self.data.len()
    }
}
