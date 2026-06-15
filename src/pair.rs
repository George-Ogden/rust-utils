pub trait Pair<T, U> {
    #[must_use]
    fn first(self) -> T;

    #[must_use]
    fn second(self) -> U;
}

pub trait PairNew<T, U>: Pair<T, U> {
    #[must_use]
    fn new(first: T, second: U) -> Self;
}

impl<T, U> Pair<T, U> for (T, U) {
    #[inline]
    fn first(self) -> T {
        self.0
    }

    #[inline]
    fn second(self) -> U {
        self.1
    }
}

impl<T, U> PairNew<T, U> for (T, U) {
    #[inline]
    fn new(first: T, second: U) -> Self {
        (first, second)
    }
}

impl<'a, T, U> Pair<&'a T, &'a U> for &'a (T, U) {
    #[inline]
    fn first(self) -> &'a T {
        &self.0
    }

    #[inline]
    fn second(self) -> &'a U {
        &self.1
    }
}

#[cfg(test)]
#[path = "pair_test.rs"]
mod test;
