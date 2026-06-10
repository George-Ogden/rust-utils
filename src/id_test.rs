use super::*;
use pretty_assertions::assert_eq;

#[test]
fn test_id_reset() {
    reset_global_id();
    assert_eq!(Id::next(), Id(0));
    assert_eq!(Id::next(), Id(1));
}

#[test]
fn test_id_set() {
    set_global_id(6);
    assert_eq!(Id::next(), Id(6));
    assert_eq!(Id::next(), Id(7));
}

#[test]
fn test_id_to_from() {
    assert_eq!(Id::from(8), Id(8));
    assert_eq!(Into::<u32>::into(Id(9)), 9);
    assert_eq!(Into::<u64>::into(Id(10)), 10);
    assert_eq!(Into::<usize>::into(Id(11)), 11);
}
