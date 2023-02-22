use std::fmt::Debug;

use bitvec::vec::BitVec;

use crate::Scalar;

use super::{Array, ArrayBuilder, ArrayIterator};

pub trait PrimitiveType: Copy + Send + Sync + Default + Debug + 'static {}

pub type I32Array = PrimitiveArray<i32>;
pub type F32Array = PrimitiveArray<f32>;

impl PrimitiveType for i32 {}
impl PrimitiveType for f32 {}

pub struct PrimitiveArray<T: PrimitiveType> {
    /// The actual data of this array
    data: Vec<T>,
    /// The null bitmap of this array
    bitmap: BitVec,
}

impl<T> Array for PrimitiveArray<T>
where
    T: PrimitiveType,
    T: Scalar<ArrayType = Self>,
{
    type RefItem<'a> = T;
    type Builder = PrimitiveArrayBuilder<T>;
    type OwnedItem = T;

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

    fn iter(&self) -> ArrayIterator<Self> {
        ArrayIterator::new(self)
    }
}

/// [`ArrayBuilder`] for [`PrimitiveArray`]
pub struct PrimitiveArrayBuilder<T: PrimitiveType> {
    /// The actual data of this array
    data: Vec<T>,
    /// The null bitmap of this array
    bitmap: BitVec,
}

impl<T> ArrayBuilder for PrimitiveArrayBuilder<T>
where
    T: PrimitiveType,
    T: Scalar<ArrayType = PrimitiveArray<T>>,
{
    type Array = PrimitiveArray<T>;

    fn with_capacity(capacity: usize) -> Self {
        Self {
            data: Vec::with_capacity(capacity),
            bitmap: BitVec::with_capacity(capacity),
        }
    }

    fn push(&mut self, value: Option<<Self::Array as Array>::RefItem<'_>>) {
        match value {
            Some(v) => {
                self.data.push(v);
                self.bitmap.push(true);
            }
            None => {
                self.data.push(Default::default());
                self.bitmap.push(false);
            }
        }
    }

    fn finish(self) -> Self::Array {
        PrimitiveArray {
            data: self.data,
            bitmap: self.bitmap,
        }
    }
}
