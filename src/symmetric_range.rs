use std::ops::{Neg, RangeInclusive};

#[inline]
pub fn symmetric_range_unchecked<T: Neg<Output = T> + Copy>(upper_bound: T) -> RangeInclusive<T> {
    -upper_bound..=upper_bound
}

#[inline]
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
