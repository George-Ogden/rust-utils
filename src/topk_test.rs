use std::{cmp::Reverse, sync::Arc};

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

#[test]
fn test_topk_pointers() {
    let chosen = Arc::new(0);
    let x = vec![Arc::clone(&chosen), Arc::new(0)];
    assert_eq!(
        Arc::as_ptr(&x.into_iter().topk(1).into_iter().next().unwrap()),
        Arc::as_ptr(&chosen)
    );
}

#[test]
fn test_topk_by_empty() {
    let x: Vec<usize> = Vec::new();
    assert_eq!(
        x.into_iter()
            .topk_by(3, |x| x + 1)
            .into_iter()
            .collect_vec(),
        Vec::new()
    );
}

#[test]
fn test_topk_by_take_zero() {
    let x: Vec<usize> = vec![3];
    assert_eq!(
        x.into_iter()
            .topk_by(0, |x| x * 2)
            .into_iter()
            .collect_vec(),
        Vec::new()
    );
}

#[test]
fn test_topk_by_take_more() {
    let x: Vec<usize> = vec![0, 1, 2];
    assert_eq!(
        x.into_iter().topk_by(5, |_| 0).into_iter().collect_vec(),
        vec![0, 1, 2]
    );
}

#[test_case(0)]
#[test_case(1)]
#[test_case(2)]
fn test_topk_by_reverse_shuffled(seed: u64) {
    let mut x: Vec<usize> = vec![0, 1, 1, 2, 3, 4, 8, 9, 9, 10];
    x.shuffle(&mut StdRng::seed_from_u64(seed));
    assert_eq!(
        x.into_iter()
            .topk_by(3, |x| Reverse(*x))
            .into_iter()
            .collect_vec(),
        vec![1, 1, 0]
    );
}

#[test]
fn test_topk_by_parity() {
    let x: Vec<usize> = vec![0, 1, 1, 2, 3, 4, 8, 9, 9, 10];
    assert_eq!(
        x.into_iter()
            .topk_by(4, |x| x % 2)
            .into_iter()
            .collect_vec(),
        vec![1, 1, 3, 9]
    );
}

#[test]
fn test_topk_by_parity_smaller() {
    let x: Vec<usize> = vec![0, 1, 1, 2, 3, 4, 8, 9, 9, 10];
    let length = x.len();
    let mut count = 0;
    assert_eq!(
        x.into_iter()
            .topk_by(3, |x| {
                count += 1;
                x % 2
            })
            .into_iter()
            .collect_vec(),
        vec![1, 1, 3]
    );
    assert_eq!(count, length);
}

#[test]
fn test_topk_by_fn_pointers() {
    let chosen = Arc::new(0);
    let x = vec![Arc::clone(&chosen), Arc::new(0)];
    assert_eq!(
        Arc::as_ptr(
            &x.into_iter()
                .topk_by(1, |x| **x)
                .into_iter()
                .next()
                .unwrap()
        ),
        Arc::as_ptr(&chosen)
    );
}

#[test]
fn test_topk_by_fn_pointers_multiple() {
    let first = Arc::new(0);
    let second = Arc::new(0);
    let x = vec![Arc::clone(&first), Arc::clone(&second)];
    let topk = &mut x.into_iter().topk_by(2, |x| **x).into_iter();
    assert_eq!(Arc::as_ptr(&topk.next().unwrap()), Arc::as_ptr(&first));
    assert_eq!(Arc::as_ptr(&topk.next().unwrap()), Arc::as_ptr(&second));
}
