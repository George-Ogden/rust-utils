use super::*;
use itertools::Itertools;
use ordered_float::NotNan;
use pretty_assertions::assert_eq;

#[test]
fn test_symmetric_range_valid() {
    let range = symmetric_range(2.0);
    assert_eq!(range, -2.0..=2.0);
    assert_eq!(range, symmetric_range_unchecked(2.0));
}

#[test]
fn test_symmetric_range_zero() {
    let range = symmetric_range(0);
    assert_eq!(range, 0..=0);
    assert_eq!(range, symmetric_range_unchecked(0));
    assert_eq!(range.into_iter().collect_vec(), [0]);
}

#[test]
#[should_panic]
fn test_symmetric_range_invalid() {
    let _range = symmetric_range(NotNan::from(-1));
}

#[test]
fn test_symmetric_range_invalid_unchecked() {
    let range = symmetric_range_unchecked(NotNan::from(-1));
    assert!(range.is_empty());
}
