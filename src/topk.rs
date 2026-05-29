use std::{
    collections::{BTreeMap, BTreeSet, btree_map, btree_set},
    iter::Map,
};

pub trait Topk: Iterator + Sized
where
    Self::Item: Ord,
{
    #[inline]
    /// Select the k largest items from an iterator.
    /// If there insufficient items, all are returned.
    /// The returned items are sorted from smallest to largest.
    #[expect(clippy::type_complexity)]
    fn topk(
        self,
        k: usize,
    ) -> Map<btree_set::IntoIter<(Self::Item, usize)>, fn((Self::Item, usize)) -> Self::Item> {
        let mut best = BTreeSet::new();
        for (id, item) in self.enumerate() {
            #[expect(clippy::else_if_without_else)]
            if best.len() < k {
                let result = best.insert((item, id));
                debug_assert!(result, "unique item already in map");
            } else if best.first().is_some_and(|worst| item > worst.0) {
                let result = best.pop_first();
                debug_assert!(result.is_some(), "item should be in map");
                let result = best.insert((item, id));
                debug_assert!(result, "unique item already in map");
            }
        }
        best.into_iter().map(|(item, _id)| item)
    }
}

impl<T> Topk for T
where
    T: Iterator,
    T::Item: Ord,
{
}

pub trait TopkBy: Iterator + Sized {
    #[inline]
    fn topk_by<K: Ord, F: FnMut(&Self::Item) -> K>(
        self,
        k: usize,
        mut f: F,
    ) -> btree_map::IntoValues<(K, usize), Self::Item> {
        let mut best = BTreeMap::new();
        for (id, value) in self.enumerate() {
            let key = f(&value);
            #[expect(clippy::else_if_without_else)]
            if best.len() < k {
                let result = best.insert((key, id), value);
                debug_assert!(result.is_none(), "unique item already in map");
            } else if best
                .first_key_value()
                .is_some_and(|(worst, _)| key > worst.0)
            {
                let result = best.pop_first();
                debug_assert!(result.is_some(), "item should be in map");
                let result = best.insert((key, id), value);
                debug_assert!(result.is_none(), "unique item already in map");
            }
        }
        best.into_values()
    }
}

impl<T> TopkBy for T where T: Iterator {}

#[cfg(test)]
#[path = "topk_test.rs"]
mod test;
