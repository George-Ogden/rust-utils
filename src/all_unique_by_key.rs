use itertools::Itertools;
use std::hash::Hash;

pub trait AllUniqueByKey: Iterator + Sized {
    #[inline]
    fn all_unique_by_key<U: Eq + Hash, F: FnMut(Self::Item) -> U>(self, key: F) -> bool {
        self.map(key).all_unique()
    }
}

impl<I: Iterator + Sized> AllUniqueByKey for I {}

#[cfg(test)]
#[path = "all_unique_by_key_test.rs"]
mod test;
