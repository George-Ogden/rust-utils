use derive_more::{Deref, Display, Into};

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

#[derive(Debug, Clone, Copy, PartialEq, Display, Into, Deref)]
pub struct Prob(f64);

impl Prob {
    #[inline]
    /// Convert an f64 to a probability.
    /// # Errors
    /// The fails if the value is NaN, negative or greater than one.
    /// The error describes this problem.
    pub fn new(value: f64) -> ProbResult {
        if (0.0..=1.0).contains(&value) {
            Ok(Self(value))
        } else {
            Err(Error::from_invalid(value))
        }
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

#[cfg(test)]
#[path = "prob_test.rs"]
mod test;
