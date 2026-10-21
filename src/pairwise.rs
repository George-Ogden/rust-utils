use itertools::Itertools;

pub trait Pairwise: Iterator<Item: Clone> + Sized {
    #[inline]
    fn pairwise(self) -> impl Iterator<Item = (Self::Item, Self::Item)> {
        self.array_windows().map_into()
    }
}

impl<I: Iterator<Item: Clone> + Sized> Pairwise for I {}

#[inline]
pub fn pairwise<I: IntoIterator<Item: Clone>>(iter: I) -> impl Iterator<Item = (I::Item, I::Item)> {
    iter.into_iter().pairwise()
}

#[cfg(test)]
#[path = "pairwise_test.rs"]
mod test;
