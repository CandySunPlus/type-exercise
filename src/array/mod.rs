use std::fmt::Debug;

mod iterator;
mod primitive_array;
mod string_array;

pub use iterator::*;
pub use primitive_array::*;
pub use string_array::*;

/// [`Array`] is a collection of data of the some type
pub trait Array: Send + Sync + Sized + 'static {
    /// Type of the item that can be retrieved from the [`Array`].
    /// For example, we can get a `i32` from [`Int32Array`], while [`StringArray`] produces a `&str`.
    /// As we need a lifetime that is the same as `self` for `&str`, we use GAT here.
    type RefItem<'a>: Clone + Copy + Debug;

    /// The corresponding [`ArrayBuilder`] of this [`Array`]
    type Builder: ArrayBuilder;

    /// Retrieve a reference to value
    fn get(&self, idx: usize) -> Option<Self::RefItem<'_>>;

    /// Number of items of array
    fn len(&self) -> usize;

    /// Indicates whether this array is empty
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Get iterator of this array
    fn iter(&self) -> ArrayIterator<Self>;
}

/// [`ArrayBuilder`] builds an [`Array`]
pub trait ArrayBuilder {
    /// The corresponding [`Array`] of this [`ArrayBuilder`]
    ///
    /// Here we use associated type to constraint the [`Array`] type of this builder,
    /// so that `Self::Array::Builder == Self`.
    /// This property is very useful when constructing generic functions, and may help
    /// a lot when implementing expressions.
    type Array: Array;

    /// Create a builder with `capacity`
    fn with_capacity(capacity: usize) -> Self;

    /// Append a value to builder
    fn push(&mut self, value: Option<<Self::Array as Array>::RefItem<'_>>);

    /// Finish build and return a new array
    fn finish(self) -> Self::Array;
}
