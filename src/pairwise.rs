use itertools::Itertools;

pub trait Pairwise: Iterator<Item: Clone> + Sized {
    #[inline]
    fn pairwise(self) -> impl Iterator<Item = (Self::Item, Self::Item)> {
        self.array_windows().map_into()
    }
}

impl<I: Iterator<Item: Clone> + Sized> Pairwise for I {}

#[cfg(test)]
#[path = "pairwise_test.rs"]
mod test;
