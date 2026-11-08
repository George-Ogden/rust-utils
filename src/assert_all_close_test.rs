use pretty_assertions::assert_eq;

#[test]
fn assert_all_close_no_args_close() {
    let left = [0.1 * 0.1];
    let right = [0.01];
    assert_ne!(left, right);
    assert_all_close!(left, right);
}

#[test]
#[should_panic]
fn assert_all_close_no_args_all_far() {
    let left = [0.2, 0.3];
    let right = [0.1, 0.2];
    assert_all_close!(left, right);
}

#[test]
#[should_panic]
fn assert_all_close_no_args_one_far() {
    let left = [0.2, 0.2];
    let right = [0.1, 0.2];
    assert_all_close!(left, right);
}

#[test]
#[should_panic]
fn assert_all_close_no_args_wrong_length() {
    let left = vec![0.1, 0.1];
    let right = vec![0.1];
    assert_all_close!(left, right);
}

#[test]
fn assert_all_close_call_count_close() {
    let mut count = 0;
    let mut call_once_only = move |x| {
        assert_eq!(count, 0, "called twice!");
        count += 1;
        x
    };
    assert_all_close!([call_once_only(3.0)], [3.0]);
}

#[test]
fn assert_all_close_abs_tol_close() {
    let x = [0.1, 0.2];
    let y = [0.2, 0.1];
    assert_all_close!(x, y, abs_tol = 0.1);
}

#[test]
#[should_panic]
fn assert_all_close_abs_tol_far() {
    let x = [0.1, 0.2];
    let y = [0.2, 0.0];
    assert_all_close!(x, y, abs_tol = 0.1);
}

#[test]
fn assert_all_close_multiple_arguments_close() {
    assert_all_close!(
        vec![10.0, 20.0, 30.0],
        vec![9.0, 18.0, 30.0],
        rel_tol = 1e-1,
        method = is_close::WEAK
    );
}

#[test]
#[should_panic]
fn assert_all_close_multiple_arguments_far() {
    assert_all_close!(
        vec![0.0, 9.0],
        vec![0.0, 10.0],
        rel_tol = 1e-1,
        method = is_close::STRONG
    );
}

#[test]
#[should_panic]
fn assert_all_close_multiple_arguments_far_wrong_length() {
    assert_all_close!(
        vec![0.0; 3],
        vec![0.0; 2],
        rel_tol = 1e-1,
        method = is_close::STRONG
    );
}
