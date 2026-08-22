#![expect(clippy::needless_pass_by_value)]
use std::hash::{DefaultHasher, Hash, Hasher};

use super::*;
use is_close::is_close;
use pretty_assertions::assert_eq;
use test_case::test_case;

fn prob(value: f64) -> Prob {
    Prob::new(value).unwrap()
}

fn quick_hash<H: Hash>(value: &H) -> u64 {
    let mut hasher = DefaultHasher::new();
    value.hash(&mut hasher);
    hasher.finish()
}

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

#[test_case(0.0f64, "0.0")]
#[test_case(-0.0, "0.0")]
#[test_case(1.0, "1.0")]
#[test_case(0.5, "0.5")]
#[test_case(0.25, "0.25")]
fn test_prob_display(value: f64, expected: &'static str) {
    assert_eq!(prob(value).to_string(), expected);
}

#[test]
fn test_prob_from_bool() {
    assert_eq!(Prob::from(true), Prob(1.0));
    assert_eq!(Prob::from(false), Prob(0.0));
    assert_eq!(Prob::ZERO, Prob(0.0));
    assert_eq!(Prob::ONE, Prob(1.0));
}

#[test]
fn test_prob_comparison() {
    assert!(prob(0.0) < prob(1.0));
    assert!(prob(1.0) > prob(0.0));
    assert!(prob(0.5f64.next_down()) < prob(0.5));
    assert_ne!(prob(0.5f64.next_down()), prob(0.5));
    assert!(prob(0.5f64.next_down()) <= prob(0.5));
    assert!(prob(0.5f64.next_up()) > prob(0.5));
    assert_ne!(prob(0.5f64.next_up()), prob(0.5));
    assert!(prob(0.5f64.next_up()) >= prob(0.5));
    assert_ne!(prob(0.0), prob(1.0));
    assert_ne!(prob(1.0), prob(0.0));
    assert_eq!(prob(0.0), prob(0.0));
    assert_eq!(prob(1.0), prob(1.0));
    assert_eq!(prob(0.5), prob(0.5));
    assert_eq!(quick_hash(&prob(0.5)), quick_hash(&prob(0.5)));
    assert_eq!(quick_hash(&prob(-0.0)), quick_hash(&prob(0.0)));
    assert_ne!(quick_hash(&prob(0.2)), quick_hash(&prob(0.3)));
}

#[test_case(0.0, 1.0)]
#[test_case(0.1, 0.9)]
#[test_case(0.0f64, 1.0f64)]
fn test_prob_negate(left: f64, right: f64) {
    let left = prob(left);
    let right = prob(right);
    assert!(is_close!((!left).get(), right.get(), abs_tol = 1e-18));
    assert!(is_close!(left.get(), (!right).get(), abs_tol = 1e-18));
}

#[test_case(0.0, 1.0, 0.0)]
#[test_case(0.1, 0.9, 0.09)]
#[test_case(1.0, 1.0, 1.0)]
#[test_case(0.0, 0.0, 0.0)]
#[test_case(0.5, 0.75, 0.375)]
fn test_prob_multiply(left: f64, right: f64, expected: f64) {
    let left = prob(left);
    let right = prob(right);
    assert!(is_close!((left * right).get(), expected, abs_tol = 1e-18));
    assert!(is_close!((right * left).get(), expected, abs_tol = 1e-18));
    let mut value = left;
    value *= right;
    assert!(is_close!(value.get(), expected, abs_tol = 1e-18));
    assert!(is_close!((left & right).get(), expected, abs_tol = 1e-18));
    let mut value = left;
    value &= right;
    assert!(is_close!(value.get(), expected, abs_tol = 1e-18));
}

#[test_case(0.0, 1.0, 1.0)]
#[test_case(0.1, 0.9, 1.0)]
#[test_case(1.0, 1.0, 1.0)]
#[test_case(0.0, 0.0, 0.0)]
#[test_case(0.2, 0.1, 0.3)]
#[test_case(0.45, 0.1, 0.55)]
#[test_case(0.5, 0.75, 1.0)]
#[test_case(0.5, 0.499, 0.999)]
#[test_case(0.5, 0.501, 1.0)]
fn test_prob_saturating_add(left: f64, right: f64, expected: f64) {
    let left = prob(left);
    let right = prob(right);
    assert!(is_close!(
        left.saturating_add(right).get(),
        expected,
        abs_tol = 1e-18
    ));
    assert!(is_close!(
        right.saturating_add(left).get(),
        expected,
        abs_tol = 1e-18
    ));
    assert!(is_close!(
        Prob::saturating_add(left, right).get(),
        expected,
        abs_tol = 1e-18
    ));
    assert!(is_close!((left | right).get(), expected, abs_tol = 1e-18));
    let mut value = left;
    value |= right;
    assert!(is_close!(value.get(), expected, abs_tol = 1e-18));
}
