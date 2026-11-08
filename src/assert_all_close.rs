use is_close::ComparatorBuilder;
use itertools::{EitherOrBoth, Itertools, all};
use num_traits::Float;

#[inline]
#[doc(hidden)]
pub fn all_close<
    'a,
    'b,
    T: 'a + Float,
    I: IntoIterator<Item = &'a T>,
    J: IntoIterator<Item = &'a T>,
>(
    left: I,
    right: J,
    comparator: &ComparatorBuilder<T>,
) -> bool {
    all(left.into_iter().zip_longest(right), |either| match either {
        EitherOrBoth::Both(left, right) => comparator.is_close(*left, *right),
        EitherOrBoth::Left(_) | EitherOrBoth::Right(_) => false,
    })
}

#[macro_export]
/// Assert that two collections are close together.
///
/// Pass in any settings supported by `is_close::is_close!`.
/// This fails if any items are pairwise different or the collections are different lengths.
/// It uses `pretty_assertions` to generate an error message if they are not different.
macro_rules! assert_all_close {
    ($left:expr, $right:expr $(, $set:ident = $val:expr)*) => {
        let left = $left;
        let right = $right;

        let mut comparator = $crate::assert_is_close::is_close::default();
        // The set methods return a mutable reference to allow chaining.
        let comparator = (&mut comparator)$(.$set($val))*;

        if !$crate::assert_all_close::all_close(&left, &right, comparator) {
            pretty_assertions::assert_eq!(
                left,
                right,
                r"`all_close!(left, right{})`",
                stringify!($(, $set = $val)*),
            );
        }
    };
}

#[cfg(test)]
#[path = "assert_all_close_test.rs"]
mod test;
