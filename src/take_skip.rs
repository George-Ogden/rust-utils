pub trait TakeSkip: Iterator + Sized {
    #[inline]
    /// Split an iterator at a given index.
    ///
    /// This eagerly collects the first `n` elements as a vector
    /// and returns the original iterator with the remaining elements.
    ///
    /// If the iterator contains fewer than `n` elements, fewer than `n` are collected.
    fn take_skip(mut self, n: usize) -> (Vec<Self::Item>, Self) {
        let take = Vec::from_iter(self.by_ref().take(n));
        (take, self)
    }
}

impl<T: Iterator + Sized> TakeSkip for T {}

#[cfg(test)]
#[path = "take_skip_test.rs"]
mod test;
