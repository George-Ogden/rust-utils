#![expect(clippy::needless_pass_by_value)]

use super::*;
use pretty_assertions::assert_eq;
use std::fmt::Debug;
use test_case::test_case;

#[test_case([] as [(); 0], [])]
#[test_case([()], [])]
#[test_case([(), ()], [((), ())])]
#[test_case([1, 2, 3], [(1, 2), (2, 3)])]
#[test_case([5, 4, 3, 2, 1], [(5, 4), (4, 3), (3, 2), (2, 1)])]
fn test_pairwise<T: Debug + Clone + PartialEq, const M: usize, const N: usize>(
    original: [T; M],
    expected: [(T, T); N],
) {
    let expected = expected.to_vec();
    assert_eq!(
        original.clone().into_iter().pairwise().collect_vec(),
        expected
    );

    assert_eq!(pairwise(original).collect_vec(), expected);
}

#[test]
fn test_infinite_pairwise() {
    assert_eq!(
        (0..).pairwise().take(5).collect_vec(),
        [(0, 1), (1, 2), (2, 3), (3, 4), (4, 5)]
    );
}
