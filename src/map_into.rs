use std::{
    collections::{BTreeSet, HashSet},
    hash::Hash,
};

use fmap::Functor;

pub trait MapInto<T> {
    type Container<X>;

    #[must_use]
    fn map_into<U>(self) -> Self::Container<U>
    where
        T: Into<U>;
}

impl<T> MapInto<T> for Option<T> {
    type Container<X> = Option<X>;

    #[inline]
    fn map_into<U>(self) -> Self::Container<U>
    where
        T: Into<U>,
    {
        self.map(Into::into)
    }
}

impl<T> MapInto<T> for Vec<T> {
    type Container<X> = Vec<X>;

    #[inline]
    fn map_into<U>(self) -> Self::Container<U>
    where
        T: Into<U>,
    {
        self.fmap(Into::into)
    }
}

pub trait HashMapInto<T: Hash + Eq> {
    type Container<X>;

    #[must_use]
    fn map_into<U: Hash + Eq>(self) -> Self::Container<U>
    where
        T: Into<U>;
}

#[expect(clippy::implicit_hasher)]
impl<T: Eq + Hash> HashMapInto<T> for HashSet<T> {
    type Container<X> = HashSet<X>;

    #[inline]
    fn map_into<U: Hash + Eq>(self) -> Self::Container<U>
    where
        T: Into<U>,
    {
        self.fmap(Into::into)
    }
}

pub trait BTreeMapInto<T: Ord> {
    type Container<X>;

    #[must_use]
    fn map_into<U: Ord>(self) -> Self::Container<U>
    where
        T: Into<U>;
}

impl<T: Ord> BTreeMapInto<T> for BTreeSet<T> {
    type Container<X> = BTreeSet<X>;

    #[inline]
    fn map_into<U: Ord>(self) -> Self::Container<U>
    where
        T: Into<U>,
    {
        self.fmap(Into::into)
    }
}

#[cfg(test)]
#[path = "map_into_test.rs"]
mod test;
