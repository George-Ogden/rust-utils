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

#[inline]
#[expect(clippy::type_complexity)]
pub fn merge_join_by_key<
    I: IntoIterator,
    J: IntoIterator<Item = I::Item>,
    F: FnMut(&I::Item) -> U,
    U: Ord,
>(
    left: I,
    right: J,
    key: F,
) -> MergeJoinBy<I::IntoIter, J::IntoIter, impl FnMut(&I::Item, &J::Item) -> Ordering> {
    left.into_iter().merge_join_by_key(right, key)
}

#[cfg(test)]
#[path = "merge_join_by_key_test.rs"]
mod test;
