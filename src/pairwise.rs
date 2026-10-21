use itertools::Itertools;

pub trait Pairwise: Iterator<Item: Clone> + Sized {
    #[inline]
    /// Iterate over pairs of item.
    /// This is similar to the Python [`itertools.pairwise`](https://docs.python.org/3/library/itertools.html#itertools.pairwise) method
    /// and is equivalent to
    /// [`Itertools::tuple_windows::<2>`](https://docs.rs/itertools/0.15.0/itertools/trait.Itertools.html#method.tuple_windows).
    fn pairwise(self) -> impl Iterator<Item = (Self::Item, Self::Item)> {
        self.array_windows().map_into()
    }
}

impl<I: Iterator<Item: Clone> + Sized> Pairwise for I {}

#[inline]
/// Helper method to call `Pairwise::pairwise`.
pub fn pairwise<I: IntoIterator<Item: Clone>>(iter: I) -> impl Iterator<Item = (I::Item, I::Item)> {
    iter.into_iter().pairwise()
}

#[cfg(test)]
#[path = "pairwise_test.rs"]
mod test;
