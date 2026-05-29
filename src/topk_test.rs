use super::*;
use itertools::Itertools;
use pretty_assertions::assert_eq;
use rand::{SeedableRng, rngs::StdRng, seq::SliceRandom};
use test_case::test_case;

#[test]
fn test_topk_empty() {
    let x: Vec<usize> = Vec::new();
    assert_eq!(x.into_iter().topk(3).into_iter().collect_vec(), Vec::new());
}

#[test]
fn test_topk_take_zero() {
    let x: Vec<usize> = vec![3];
    assert_eq!(x.into_iter().topk(0).into_iter().collect_vec(), Vec::new());
}

#[test]
fn test_topk_take_more() {
    let x: Vec<usize> = vec![0, 1, 2];
    assert_eq!(
        x.into_iter().topk(5).into_iter().collect_vec(),
        vec![0, 1, 2]
    );
}

#[test]
fn test_topk_take_all_non_unique() {
    let x: Vec<usize> = vec![2, 0, 2, 1, 2, 1];
    assert_eq!(
        x.into_iter().topk(6).into_iter().collect_vec(),
        vec![0, 1, 1, 2, 2, 2]
    );
}

#[test]
fn test_topk_take_some_non_unique() {
    let x: Vec<usize> = vec![0, 1, 1, 2, 2, 2];
    assert_eq!(
        x.into_iter().topk(4).into_iter().collect_vec(),
        vec![1, 2, 2, 2]
    );
}

#[test]
fn test_topk_take_some_non_unique_reversed() {
    let x: Vec<usize> = vec![0, 1, 1, 2, 2, 2];
    assert_eq!(
        x.into_iter().topk(4).into_iter().collect_vec(),
        vec![1, 2, 2, 2]
    );
}

#[test_case(0)]
#[test_case(1)]
#[test_case(2)]
fn test_topk_take_some_shuffled(seed: u64) {
    let mut x: Vec<usize> = vec![0, 1, 1, 2, 3, 4, 8, 9, 9, 10];
    x.shuffle(&mut StdRng::seed_from_u64(seed));
    assert_eq!(
        x.into_iter().topk(3).into_iter().collect_vec(),
        vec![9, 9, 10]
    );
}
