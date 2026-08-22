use std::fmt::{self, Display};
use std::ops::{BitAnd, BitAndAssign, Mul, MulAssign, Not};
use std::{
    cmp,
    hash::{self, Hash},
};

use derive_more::{Deref, Into};

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Error {
    NanValue,
    Negative,
    GreaterThanOne,
}

impl Error {
    fn from_invalid(value: f64) -> Self {
        if value.is_nan() {
            Self::NanValue
        } else if value < 0.0 {
            Self::Negative
        } else if value > 1.0 {
            Self::GreaterThanOne
        } else {
            unreachable!("Expected an invalid probability {value}.")
        }
    }
}

type ProbResult = Result<Prob, Error>;

#[derive(Debug, Clone, Copy, PartialEq, Into, Deref)]
pub struct Prob(f64);

impl Prob {
    #[inline]
    /// Convert an f64 to a probability.
    /// # Errors
    /// The fails if the value is NaN, negative or greater than one.
    /// The error describes this problem.
    pub fn new(value: f64) -> ProbResult {
        if (0.0..=1.0).contains(&value) {
            // Convert -0.0 to +0.0.
            Ok(Self(value + 0.0))
        } else {
            Err(Error::from_invalid(value))
        }
    }

    #[inline]
    #[must_use]
    pub const fn get(self) -> f64 {
        self.0
    }
}

impl TryFrom<f64> for Prob {
    type Error = Error;

    #[inline]
    fn try_from(value: f64) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl TryFrom<f32> for Prob {
    type Error = Error;

    #[inline]
    fn try_from(value: f32) -> Result<Self, Self::Error> {
        Self::try_from(f64::from(value))
    }
}

impl Display for Prob {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.get() {
            0.0 => write!(f, "0.0"),
            1.0 => write!(f, "1.0"),
            p => write!(f, "{p}"),
        }
    }
}

impl PartialOrd for Prob {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Prob {
    #[inline]
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.get().partial_cmp(&other.get()).unwrap()
    }
}

impl Eq for Prob {}

impl Hash for Prob {
    #[inline]
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        // The value is canonicalized during `Prob::new`.
        self.get().to_bits().hash(state);
    }
}

impl Not for Prob {
    type Output = Self;

    #[inline]
    /// Return the negation (1.0 - p).
    fn not(self) -> Self::Output {
        Self(1.0 - self.get())
    }
}

impl Mul for Prob {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.get() * rhs.get())
    }
}
impl MulAssign for Prob {
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl BitAnd for Prob {
    type Output = Self;

    #[inline]
    #[expect(clippy::suspicious_arithmetic_impl)]
    /// Returns the product of the two probabilities.
    fn bitand(self, rhs: Self) -> Self::Output {
        self * rhs
    }
}
impl BitAndAssign for Prob {
    #[inline]
    fn bitand_assign(&mut self, rhs: Self) {
        Self::mul_assign(self, rhs);
    }
}

#[cfg(test)]
#[path = "prob_test.rs"]
mod test;
