use super::*;
use pretty_assertions::assert_eq;

#[test]
#[expect(clippy::float_cmp)]
fn test_tuple_pair() {
    let pair = (3, 5.0);
    assert_eq!(pair.first(), 3);
    assert_eq!(pair.second(), 5.0);
}

#[test]
fn test_tuple_pair_new() {
    assert_eq!(<((), (i32,))>::new((), (0,)), ((), (0,)));
}

#[test]
fn test_tuple_ref_pair() {
    let tuple = (true, "right");
    let pair = &tuple;
    assert_eq!(pair.first(), &true);
    assert_eq!(pair.second(), &"right");
}
