#![allow(dead_code)]

use std::marker::PhantomData;

use anyhow::Result;

use crate::{Array, ArrayBuilder, ArrayImpl, Scalar, TypeMismatch};

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

    /// Evaluate the expression with the given array.
    pub fn eval(&self, i1: &'a ArrayImpl, i2: &'a ArrayImpl) -> Result<ArrayImpl> {
        let i1a: &'a I1 = i1.try_into()?;
        let i2a: &'a I2 = i2.try_into()?;

        assert_eq!(i1.len(), i2.len(), "array lenghth mismatch");

        let mut builder = O::Builder::with_capacity(i1.len());

        for (i1, i2) in i1a.iter().zip(i2a.iter()) {
            match (i1, i2) {
                (Some(i1), Some(i2)) => builder.push(Some((self.func)(i1, i2).as_scalar_ref())),
                _ => builder.push(None),
            }
        }

        Ok(builder.finish().into())
    }
}

#[cfg(test)]
mod tests {
    use super::cmp::*;
    use super::string::*;
    use super::*;
    use crate::{BoolArray, I32Array, I64Array, StringArray};

    fn check_array_eq<'a, A: Array>(array: &'a A, vec: &[Option<A::RefItem<'a>>])
    where
        A::RefItem<'a>: PartialEq,
    {
        for (a, b) in array.iter().zip(vec.iter()) {
            assert_eq!(&a, b);
        }
    }

    #[test]
    fn test_cmp_le() {
        let expr = BinaryExpression::<I32Array, I32Array, BoolArray, _>::new(
            cmp_le::<I32Array, I32Array, I64Array>,
        );
        let result = expr
            .eval(
                &I32Array::from_slice(&[Some(0), Some(1), None]).into(),
                &I32Array::from_slice(&[Some(1), Some(0), None]).into(),
            )
            .unwrap();

        check_array_eq::<BoolArray>(
            &result.try_into().unwrap(),
            &[Some(true), Some(false), None],
        );
    }

    #[test]
    fn test_cmp_ge_str() {
        let expr = BinaryExpression::<StringArray, StringArray, BoolArray, _>::new(
            cmp_ge::<StringArray, StringArray, StringArray>,
        );

        let result = expr
            .eval(
                &StringArray::from_slice(&[Some("0"), Some("1"), None]).into(),
                &StringArray::from_slice(&[Some("1"), Some("0"), None]).into(),
            )
            .unwrap();

        check_array_eq::<BoolArray>(
            &result.try_into().unwrap(),
            &[Some(false), Some(true), None],
        );
    }

    #[test]
    fn test_str_contains() {
        let expr = BinaryExpression::<StringArray, StringArray, BoolArray, _>::new(str_contains);
        let result = expr
            .eval(
                &StringArray::from_slice(&[Some("000"), Some("111"), None]).into(),
                &StringArray::from_slice(&[Some("0"), Some("0"), None]).into(),
            )
            .unwrap();
        check_array_eq::<BoolArray>(
            &result.try_into().unwrap(),
            &[Some(true), Some(false), None],
        );
    }
}
