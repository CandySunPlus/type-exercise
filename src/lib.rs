mod array;
pub mod macros;
mod scalar;

pub use array::*;
pub use scalar::*;

use thiserror::Error;

#[derive(Error, Debug)]
#[error("Type mispatch on conversion: expected {0}, get {1}")]
pub struct TypeMismatch(&'static str, &'static str);
