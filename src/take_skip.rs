pub trait TakeSkip: Iterator + Sized {
    #[inline]
    fn take_skip(mut self, n: usize) -> (Vec<Self::Item>, Self) {
        let mut take = Vec::with_capacity(n);
        for _ in 0..n {
            if let Some(item) = self.next() {
                take.push(item);
            } else {
                break;
            }
        }
        (take, self)
    }
}

impl<T: Iterator + Sized> TakeSkip for T {}

#[cfg(test)]
#[path = "take_skip_test.rs"]
mod test;
