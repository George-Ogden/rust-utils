use std::cmp::Ordering;

use itertools::{Itertools, MergeJoinBy};

pub trait MergeJoinByKey: Iterator + Sized {
    #[inline]
    #[expect(clippy::type_complexity)]
    fn merge_join_by_key<J: IntoIterator<Item = Self::Item>, F: FnMut(&Self::Item) -> U, U: Ord>(
        self,
        other: J,
        mut key: F,
    ) -> MergeJoinBy<Self, J::IntoIter, impl FnMut(&Self::Item, &J::Item) -> Ordering> {
        self.merge_join_by(other, move |left, right| key(left).cmp(&key(right)))
    }
}

impl<I: Iterator + Sized> MergeJoinByKey for I {}

#[cfg(test)]
#[path = "merge_join_by_key_test.rs"]
mod test;
