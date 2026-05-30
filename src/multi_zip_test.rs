use super::*;
use pretty_assertions::assert_eq;

#[test]
fn test_multi_zip_empty() {
    assert_eq!(
        multi_zip([] as [[(); 0]; 0]).collect_vec(),
        Vec::<Vec<()>>::new()
    );
}

#[test]
fn test_multi_zip_no_items() {
    assert_eq!(
        multi_zip([[] as [(); 0]]).collect_vec(),
        Vec::<Vec<()>>::new()
    );
}

#[test]
fn test_multi_zip_one_item() {
    assert_eq!(multi_zip([[()]]).collect_vec(), vec![vec![()]]);
}

#[test]
fn test_multi_zip_one_element() {
    assert_eq!(
        multi_zip([vec![1], vec![2], vec![3]]).collect_vec(),
        vec![vec![1, 2, 3]]
    );
}

#[test]
fn test_multi_zip_multiple_elements() {
    assert_eq!(
        multi_zip(vec![vec![1, 11, 21], vec![2, 12, 22], vec![3, 13, 23]]).collect_vec(),
        vec![vec![1, 2, 3], vec![11, 12, 13], vec![21, 22, 23]]
    );
}

#[test]
fn test_multi_zip_multiple_elements_one_shortest() {
    assert_eq!(
        multi_zip([vec![1, 11], vec![2, 12, 22], vec![3, 13, 23]]).collect_vec(),
        vec![vec![1, 2, 3], vec![11, 12, 13]]
    );
}

#[test]
fn test_multi_zip_multiple_elements_one_longest() {
    assert_eq!(
        multi_zip(vec![vec![1, 11, 21], vec![2, 12, 22], vec![3, 13, 23, 33]]).collect_vec(),
        vec![vec![1, 2, 3], vec![11, 12, 13], vec![21, 22, 23]]
    );
}

#[test]
fn test_multi_zip_infinite_iterators() {
    let mut zip = multi_zip(vec![0.., 1..]);
    assert_eq!(zip.next(), Some(vec![0, 1]));
    assert_eq!(zip.next(), Some(vec![1, 2]));
    assert_eq!(zip.next(), Some(vec![2, 3]));
    assert_eq!(zip.next(), Some(vec![3, 4]));
    assert_eq!(zip.next(), Some(vec![4, 5]));
}
