#![expect(clippy::float_cmp)]
use super::*;
use pretty_assertions::assert_eq;
use test_case::test_case;

#[test_case(0.0, 0.0)]
#[test_case(-0.0f64, 0.0)]
#[test_case(2.0, 1.0)]
#[test_case(0.0f64.next_up(), 1.0)]
#[test_case(-3.0, -1.0)]
#[test_case(0.0f64.next_down(), -1.0)]
#[test_case(f64::INFINITY, 1.0)]
#[test_case(f64::NEG_INFINITY, -1.0)]
#[test_case(f64::NAN, f64::NAN)]
fn sign_test(value: f64, expected: f64) {
    let result = sign(value);
    if expected.is_nan() {
        assert!(result.is_nan());
    } else {
        assert_eq!(result, expected);
    }
}
