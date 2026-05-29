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
