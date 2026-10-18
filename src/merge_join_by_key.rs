use std::cmp::Ordering;

use itertools::{Itertools, MergeJoinBy};

pub trait MergeJoinByKey: Iterator + Sized {
    #[inline]
    #[expect(clippy::type_complexity)]
    /// Compare two items by a common, ordered key to merge their iterators in ascending order.
    /// Refer to
    /// <https://docs.rs/itertools/0.15.0/itertools/trait.Itertools.html#method.merge_join_by> for
    /// more information.
    fn merge_join_by_key<J: IntoIterator<Item = Self::Item>, F: FnMut(&Self::Item) -> U, U: Ord>(
        self,
        other: J,
        mut key: F,
    ) -> MergeJoinBy<Self, J::IntoIter, impl FnMut(&Self::Item, &J::Item) -> Ordering> {
        #[expect(clippy::disallowed_methods)]
        self.merge_join_by(other, move |left, right| key(left).cmp(&key(right)))
    }
}

impl<I: Iterator + Sized> MergeJoinByKey for I {}

#[inline]
#[expect(clippy::type_complexity)]
/// Helper method to call `MergeJoinByKey::merge_join_by_key`.
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
