pub trait Pair<T, U> {
    #[must_use]
    fn first(self) -> T;

    #[must_use]
    fn second(self) -> U;
}

pub trait PairGeneralization<T, U>: Pair<T, U> {
    type Pair<A, B>: Pair<A, B>;
}

pub trait PairNew<T, U>: Pair<T, U> {
    #[must_use]
    fn new(first: T, second: U) -> Self;
}

pub trait PairClone<T, U>: Pair<T, U> + PairGeneralization<T, U> {
    fn clone<'a>(pair: Self::Pair<&'a T, &'a U>) -> Self;
}

pub trait PairRev<T, U>: Pair<T, U> + PairGeneralization<T, U> {
    #[must_use]
    fn rev(self) -> Self::Pair<U, T>;
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

impl<T, U> PairGeneralization<T, U> for (T, U) {
    type Pair<A, B> = (A, B);
}

impl<'a, T: 'a, U: 'a> Pair<&'a T, &'a U> for &'a (T, U) {
    #[inline]
    fn first(self) -> &'a T {
        &self.0
    }

    #[inline]
    fn second(self) -> &'a U {
        &self.1
    }
}

impl<T, U> PairRev<T, U> for (T, U) {
    #[inline]
    fn rev(self) -> Self::Pair<U, T> {
        let (first, second) = self;
        (second, first)
    }
}

impl<T: Clone, U: Clone, P: Pair<T, U> + PairNew<T, U> + PairGeneralization<T, U>> PairClone<T, U>
    for P
where
    for<'a> Self::Pair<&'a T, &'a U>: Copy,
{
    #[inline]
    fn clone<'a>(pair: Self::Pair<&'a T, &'a U>) -> Self {
        Self::new(pair.first().clone(), pair.second().clone())
    }
}

#[cfg(test)]
#[path = "pair_test.rs"]
mod test;
