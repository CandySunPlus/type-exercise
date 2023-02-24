use std::marker::PhantomData;

use crate::{Array, ArrayImpl, TypeMismatch};
use anyhow::Result;

mod cmp;
mod string;

/// Represents a binary expression which taks `I1` and `I2` as input parameter. and outputs aray of
/// type `O`.
///
/// [`BinaryExpression`] automatically vectorizes the scalar function to a vectorized one, while
/// erasing the concreate array type. Therefore, users simple call
/// `BinaryExpression::eval(ArrayImpl, ArrayImpl)`, while developers only need to provide
/// implementation for functions link `cmp_le(i32, i32)`.
pub struct BinaryExpression<I1: Array, I2: Array, O: Array, F> {
    func: F,
    _phantom: PhantomData<(I1, I2, O)>,
}

/// Implement [`BinaryExpression`] for any given scalar function `F`.
///
/// Note that as we cannot add `From<&'a ArrayImpl>` bound on [`Array`], so we have to specify them
/// here.
impl<'a, I1: Array, I2: Array, O: Array, F> BinaryExpression<I1, I2, O, F>
where
    &'a I1: TryFrom<&'a ArrayImpl, Error = TypeMismatch>,
    &'a I2: TryFrom<&'a ArrayImpl, Error = TypeMismatch>,
    F: Fn(I1::RefItem<'a>, I2::RefItem<'a>) -> O::OwnedItem,
{
    /// Create a binary expression from existing function
    ///
    /// Previously (maybe nighly toolchain 2021-12-15), this function is not possible to be
    /// compiled due to some lifetime diagnose bug in the Rust compiler.
    pub fn new(func: F) -> Self {
        Self {
            func,
            _phantom: PhantomData,
        }
    }

    pub fn eval(&self, i1: &'a ArrayImpl, i2: &'a ArrayImpl) -> Result<ArrayImpl> {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check_array_eq<'a, A: Array>(array: &'a A, vec: &[Option<A::RefItem<'a>>])
    where
        A::RefItem<'a>: PartialEq,
    {
        for (a, b) in array.iter().zip(vec.iter()) {
            assert_eq!(&a, b);
        }
    }
}
