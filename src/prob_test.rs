#![expect(clippy::needless_pass_by_value)]
use std::hash::{DefaultHasher, Hash, Hasher};

use super::*;
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
