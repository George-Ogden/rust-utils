use super::*;
use itertools::Itertools;
use pretty_assertions::assert_eq;

#[test]
fn test_itertools_map_into() {
    assert_eq!((0..=2).map_into::<f64>().collect_vec(), vec![0.0, 1.0, 2.0]);
}

#[test]
fn test_functor_map_into() {
    let v = vec![3, 4, 5];
    assert_eq!(v.map_into::<f64>(), vec![3.0, 4.0, 5.0]);
}

#[test]
fn test_option_none_map_into() {
    let x: Option<i8> = None;
    assert_eq!(x.map_into::<f64>(), None);
}

#[test]
fn test_option_some_map_into() {
    let x: Option<i8> = Some(3);
    assert_eq!(x.map_into::<f64>(), Some(3.0));
}

#[test]
fn test_hash_set_map_into() {
    let x = HashSet::from_iter([0, 1, 2]);
    assert_eq!(
        x.map_into::<Option<usize>>(),
        HashSet::from_iter([Some(0), Some(1), Some(2)])
    );
}

#[test]
fn test_tree_set_map_into() {
    let x = BTreeSet::from_iter([0, 1, 2]);
    assert_eq!(
        x.map_into::<Option<usize>>(),
        BTreeSet::from_iter([Some(0), Some(1), Some(2)])
    );
}
