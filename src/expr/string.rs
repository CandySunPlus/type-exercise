use crate::{BoolArray, StringArray};

use super::vectorize::BinaryExprFunc;

/// Checks if `i1.contains(i2)` for two string inputs.
pub struct ExprStrContains;

impl BinaryExprFunc<StringArray, StringArray, BoolArray> for ExprStrContains {
    fn eval(&self, i1: &str, i2: &str) -> <BoolArray as crate::Array>::OwnedItem {
        i1.contains(i2)
    }
}
