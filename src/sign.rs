use num_traits::{Signed, Zero, signum};

#[inline]
#[must_use]
pub fn sign<T: Signed + Zero>(x: T) -> T {
    if x.is_zero() { T::zero() } else { signum(x) }
}

#[cfg(test)]
#[path = "sign_test.rs"]
mod test;
