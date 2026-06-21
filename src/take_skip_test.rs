#![expect(clippy::needless_pass_by_value)]
use std::fmt::Debug;

use super::*;
use itertools::Itertools;
use pretty_assertions::assert_eq;
use test_case::test_case;

#[test_case([1,2,4,8], 2, (vec![1,2], vec![4,8]))]
#[test_case([1,2,4,8], 1, (vec![1], vec![2,4,8]))]
#[test_case([1,2,4,8], 0, (vec![], vec![1,2,4,8]))]
#[test_case([1,2,4,8], 4, (vec![1,2,4,8], vec![]))]
#[test_case([1,2,4,8], 5, (vec![1,2,4,8], vec![]))]
#[test_case([] as [(); 0], 1, (vec![], vec![]))]
#[test_case([] as [(); 0], 0, (vec![], vec![]))]
#[test_case([Some(3)], 0, (vec![], vec![Some(3)]))]
#[test_case([1.0], 1, (vec![1.0], vec![]))]
fn test_iterator_take_skip<T: Debug + PartialEq>(
    original: impl IntoIterator<Item = T>,
    n: usize,
    expected: (Vec<T>, Vec<T>),
) {
    let iter = original.into_iter();
    let (take, skip) = iter.take_skip(n);
    let skip = skip.collect_vec();
    assert_eq!((take, skip), expected);
}

#[test_case(5.., 0, (vec![], vec![5,6,7,8]))]
#[test_case(6.., 1, (vec![6], vec![7,8,9]))]
#[test_case(7.., 3, (vec![7,8,9], vec![10,11,12]))]
fn test_iterator_take_skip_infinite<T: Debug + PartialEq>(
    original: impl IntoIterator<Item = T>,
    n: usize,
    expected: (Vec<T>, Vec<T>),
) {
    let iter = original.into_iter();
    let (take, skip) = iter.take_skip(n);
    let skip = skip.take(expected.1.len()).collect_vec();
    assert_eq!((take, skip), expected);
}
