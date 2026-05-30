use std::iter;

use itertools::Itertools;

/// Transpose an iterator of arbitrary iterators into an iterator of vectors.
/// The zip stops when the first item is empty.
#[inline]
pub fn multi_zip<T>(
    iter: impl IntoIterator<Item = impl IntoIterator<Item = T>>,
) -> impl Iterator<Item = Vec<T>> {
    let mut all_iters = iter.into_iter().map(IntoIterator::into_iter).collect_vec();
    iter::from_fn(move || {
        all_iters
            .iter_mut()
            .map(Iterator::next)
            .collect::<Option<Vec<_>>>()
            .filter(|_| !all_iters.is_empty())
    })
}

#[cfg(test)]
#[path = "multi_zip_test.rs"]
mod test;
