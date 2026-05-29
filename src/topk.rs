use std::collections::BTreeSet;

pub trait Topk: Iterator + Sized
where
    Self::Item: Ord,
{
    #[inline]
    #[must_use]
    fn topk(self, k: usize) -> impl IntoIterator<Item = Self::Item> {
        let mut best = BTreeSet::new();
        for (id, item) in self.enumerate() {
            #[expect(clippy::else_if_without_else)]
            if best.len() < k {
                let result = best.insert((item, id));
                debug_assert!(result, "unique item already in map");
            } else if best.first().is_some_and(|worst| item >= worst.0) {
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

#[cfg(test)]
#[path = "topk_test.rs"]
mod test;
