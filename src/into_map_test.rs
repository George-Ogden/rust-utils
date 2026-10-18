use super::*;
use fmap::Functor;
use itertools::Itertools;
use pretty_assertions::assert_eq;

#[test]
fn test_into_map() {
    let iter = vec![1, 1, 2, 3, 5, 8, 13];
    assert_eq!(
        iter.clone().fmap(|x| x + 1),
        iter.into_map(|x| x + 1).collect_vec()
    );
}
