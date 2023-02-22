use std::fmt::Debug;

mod primitive_array;
mod string_array;

/// [`Array`] is a collection of data of the some type
pub trait Array: Send + Sync + Sized + 'static {
    /// Type of the item that can be retrieved from the [`Array`].
    /// For example, we can get a `i32` from [`Int32Array`], while [`StringArray`] produces a `&str`.
    /// As we need a lifetime that is the same as `self` for `&str`, we use GAT here.
    type RefItem<'a>: Clone + Copy + Debug;

    /// Retrieve a reference to value
    fn get(&self, idx: usize) -> Option<Self::RefItem<'_>>;

    /// Number of items of array
    fn len(&self) -> usize;

    /// Indicates whether this array is empty
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
