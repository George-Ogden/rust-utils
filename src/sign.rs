use std::cmp::Ordering;

use num_traits::Float;

#[inline]
#[must_use]
pub fn sign<T: Float>(x: T) -> T {
    match x.partial_cmp(&T::zero()) {
        Some(Ordering::Less) => -T::one(),
        Some(Ordering::Greater) => T::one(),
        Some(Ordering::Equal) => T::zero(),
        None => T::nan(),
    }
}

#[cfg(test)]
#[path = "sign_test.rs"]
mod test;
