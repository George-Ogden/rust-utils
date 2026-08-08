use std::ops::{Neg, RangeInclusive};

#[inline]
/// Generate the closed interval `[-x, x]`.
///
/// This does not check that `-x <= x`.
/// Use `symmetric_range` as an alternative to ensure that the range is nonempty.
pub fn symmetric_range_unchecked<T: Neg<Output = T> + Copy>(upper_bound: T) -> RangeInclusive<T> {
    -upper_bound..=upper_bound
}

#[inline]
/// Generate the closed interval `[-x, x]`.
///
/// # Panics
/// Will panic if `!(-x <= x)`.
///
/// When `x = 0`, the range contains one element (`0`).
pub fn symmetric_range<T: Neg<Output = T> + Copy + PartialOrd>(
    upper_bound: T,
) -> RangeInclusive<T> {
    let range = symmetric_range_unchecked(upper_bound);
    assert!(
        range.start() <= range.end(),
        "Lower bound is less than upper bound."
    );
    range
}

#[cfg(test)]
#[path = "symmetric_range_test.rs"]
mod test;
