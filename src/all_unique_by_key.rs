use itertools::Itertools;
use std::hash::Hash;

pub trait AllUniqueByKey: Iterator + Sized {
    #[inline]
    /// Check whether all elements are unique (non equal) when the key is applied.
    /// Empty iterators are considered to have unique elements.
    /// The iterator is not consumed if early elements are found to be equal.
    fn all_unique_by_key<U: Eq + Hash, F: FnMut(Self::Item) -> U>(self, key: F) -> bool {
        // nosemgrep: map-all-unique
        self.map(key).all_unique()
    }
}

impl<I: Iterator + Sized> AllUniqueByKey for I {}

#[cfg(test)]
#[path = "all_unique_by_key_test.rs"]
mod test;
