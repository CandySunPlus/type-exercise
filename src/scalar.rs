use std::fmt::Debug;

use crate::{Array, F32Array, I32Array, StringArray};

/// An owned single value
///
/// For example, `i32`, `String` both implements [`Scalar`]
pub trait Scalar: Debug + Clone + Send + Sync + 'static {
    type ArrayType: Array<OwnedItem = Self>;
}

impl Scalar for i32 {
    type ArrayType = I32Array;
}
impl Scalar for f32 {
    type ArrayType = F32Array;
}

impl Scalar for String {
    type ArrayType = StringArray;
}
