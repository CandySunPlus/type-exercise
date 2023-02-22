use bitvec::vec::BitVec;

use super::Array;

pub struct StringArray {
    /// The flattened data of string
    data: Vec<u8>,
    /// Offsets of each string in the data flat array
    offsets: Vec<usize>,
    /// The null bitmap of this array
    bitmap: BitVec,
}

impl Array for StringArray {
    type RefItem<'a> = &'a str;

    fn get(&self, idx: usize) -> Option<Self::RefItem<'_>> {
        if self.bitmap[idx] {
            let range = self.offsets[idx]..self.offsets[idx + 1];
            Some(unsafe { std::str::from_utf8_unchecked(&self.data[range]) })
        } else {
            None
        }
    }

    fn len(&self) -> usize {
        self.bitmap.len()
    }
}
