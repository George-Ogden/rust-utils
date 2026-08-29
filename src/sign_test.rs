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
fn test_sign_f64(value: f64, expected: f64) {
    let result = sign(value);
    if expected.is_nan() {
        assert!(result.is_nan());
    } else {
        assert_eq!(result, expected);
    }
}

#[test_case(0.0, 0.0)]
#[test_case(-0.0f32, 0.0)]
#[test_case(2.0, 1.0)]
#[test_case(0.0f32.next_up(), 1.0)]
#[test_case(-3.0, -1.0)]
#[test_case(0.0f32.next_down(), -1.0)]
#[test_case(f32::INFINITY, 1.0)]
#[test_case(f32::NEG_INFINITY, -1.0)]
#[test_case(f32::NAN, f32::NAN)]
fn test_sign_f32(value: f32, expected: f32) {
    let result = sign(value);
    if expected.is_nan() {
        assert!(result.is_nan());
    } else {
        assert_eq!(result, expected);
    }
}

#[test_case(0, 0)]
#[test_case(2, 1)]
#[test_case(1, 1)]
#[test_case(-1i32, -1)]
#[test_case(-3, -1)]
#[test_case(i32::MIN, -1)]
#[test_case(i32::MAX, 1)]
fn test_sign_i32(value: i32, expected: i32) {
    let result = sign(value);
    assert_eq!(result, expected);
}
