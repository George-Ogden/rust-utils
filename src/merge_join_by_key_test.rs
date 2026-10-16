#![expect(clippy::needless_pass_by_value)]

use super::*;
use fmap::Functor;
use itertools::{
    EitherOrBoth::{self, Both, Left, Right},
    Itertools,
};
use ordered_float::OrderedFloat;
use pretty_assertions::assert_eq;
use std::fmt::Debug;
use test_case::test_case;

#[test_case([] as [(); 0], [], Clone::clone, [])]
#[test_case([], [()], Clone::clone, [Right(())])]
#[test_case([()], [(), ()], Clone::clone, [Both((), ()), Right(())])]
#[test_case([0, 3], [1, 2, 4], Clone::clone, [Left(0), Right(1), Right(2), Left(3), Right(4)])]
#[test_case([1.5], [0.5, 1.5, 2.5], |x| OrderedFloat(*x), [Right(0.5), Both(1.5, 1.5), Right(2.5)])]
#[test_case([0, 5, 0], [1, 1], Clone::clone, [Left(0), Right(1), Right(1), Left(5), Left(0)])]
fn test_merge_join_by_key<
    T: Clone + Debug + PartialEq,
    U: Ord,
    F: Clone + FnMut(&T) -> U,
    const K: usize,
    const M: usize,
    const N: usize,
>(
    left: [T; M],
    right: [T; N],
    key: F,
    expected: [EitherOrBoth<T, T>; K],
) {
    let expected = expected.to_vec();
    {
        let merged = left
            .clone()
            .into_iter()
            .merge_join_by_key(right.clone(), key.clone())
            .collect_vec();
        assert_eq!(merged, expected);
    }

    {
        let merged = merge_join_by_key(left.clone(), right.clone(), key.clone()).collect_vec();
        assert_eq!(merged, expected);
    }

    {
        let (right, left) = (left, right);
        let flipped_merged = left.into_iter().merge_join_by_key(right, key).collect_vec();
        let flipped_expected = expected.fmap(|either| match either {
            Both(left, right) => Both(right, left),
            Left(left) => Right(left),
            Right(right) => Left(right),
        });
        assert_eq!(flipped_merged, flipped_expected);
    }
}
