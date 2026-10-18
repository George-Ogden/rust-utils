use std::iter::Map;

pub trait IntoMap: IntoIterator + Sized {
    #[inline]
    fn into_map<U, F: FnMut(Self::Item) -> U>(self, f: F) -> Map<Self::IntoIter, F> {
        self.into_iter().map(f)
    }
}

impl<I: IntoIterator + Sized> IntoMap for I {}

#[cfg(test)]
#[path = "into_map_test.rs"]
mod test;
