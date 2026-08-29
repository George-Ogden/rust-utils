use num_traits::{Signed, Zero, signum};

#[inline]
#[must_use]
/// Calculate the sign of a number.
///
/// For floats:
/// - `0.0` if the number is zero, `-0.0` or `+0.0`.
/// - `1.0` if the number is positive, `+1.0` or `INFINITY`.
/// - `-1.0` if the number is negative, `-1.0` or `NEG_INFINITY`.
/// - `NaN` if the number is `NaN`.
///
/// For signed integers:
/// - `0` if the number is zero.
/// - `1` if the number is positive.
/// - `-1` if the number is negative.
pub fn sign<T: Signed + Zero>(x: T) -> T {
    if x.is_zero() { T::zero() } else { signum(x) }
}

#[cfg(test)]
#[path = "sign_test.rs"]
mod test;
