#![expect(clippy::needless_pass_by_value)]
use super::*;
use pretty_assertions::assert_eq;
use test_case::test_case;

#[test_case(f64::NAN, Err(Error::NanValue))]
#[test_case(0.0, Ok(Prob(0.0)))]
#[test_case(-0.0f64, Ok(Prob(0.0)))]
#[test_case(0.1, Ok(Prob(0.1)))]
#[test_case(0.5, Ok(Prob(0.5)))]
#[test_case(0.99, Ok(Prob(0.99)))]
#[test_case(1.0, Ok(Prob(1.0)))]
#[test_case(0.0f64.next_down(), Err(Error::Negative))]
#[test_case(1.0f64.next_up(), Err(Error::GreaterThanOne))]
#[test_case(f64::INFINITY, Err(Error::GreaterThanOne))]
#[test_case(f64::NEG_INFINITY, Err(Error::Negative))]
fn test_prob_constructor(value: f64, expected: ProbResult) {
    assert_eq!(Prob::new(value), expected);
    assert_eq!(Prob::try_from(value), expected);
}

#[test_case(f32::NAN, Err(Error::NanValue))]
#[test_case(0.0, Ok(Prob(0.0)))]
#[test_case(-0.0f32, Ok(Prob(0.0)))]
#[test_case(0.5, Ok(Prob(0.5)))]
#[test_case(1.0, Ok(Prob(1.0)))]
#[test_case(0.0f32.next_down(), Err(Error::Negative))]
#[test_case(1.0f32.next_up(), Err(Error::GreaterThanOne))]
fn test_prob_from_f32(value: f32, expected: ProbResult) {
    assert_eq!(Prob::try_from(value), expected);
}
