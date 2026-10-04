#[test]
fn assert_is_close_no_args_close() {
    let x = 0.1 * 0.1;
    let y = 0.01;
    assert_ne!(x, y);
    assert_is_close!(x, y);
}

#[test]
#[should_panic = "assertion `is_close!(0.010000000000000002, 0.02)` failed"]
fn assert_is_close_no_args_far() {
    let x = 0.1 * 0.1;
    let y = 0.02;
    assert_ne!(x, y);
    assert_is_close!(x, y);
}

#[test]
fn assert_is_close_call_count_close() {
    let mut count = 0;
    let mut call_once_only = move |x| {
        assert_eq!(count, 0, "called twice!");
        count += 1;
        x
    };
    assert_is_close!(call_once_only(3.0), 3.0);
}

#[test]
#[should_panic = "assertion `is_close!(3.0, 4.0)` failed"]
fn assert_is_close_call_count_far() {
    let mut count = 0;
    let mut call_once_only = move |x| {
        assert_eq!(count, 0, "called twice!");
        count += 1;
        x
    };
    assert_is_close!(call_once_only(3.0), 4.0);
}

#[test]
fn assert_is_close_abs_tol_close() {
    let x = 0.1;
    let y = 0.2;
    assert_is_close!(x, y, abs_tol = 0.1);
}

#[test]
#[should_panic = "assertion `is_close!(0.1, 0.3, abs_tol = 0.1)` failed"]
fn assert_is_close_abs_tol_far() {
    let x = 0.1;
    let y = 0.3;
    assert_is_close!(x, y, abs_tol = 0.1);
}

#[test]
fn assert_is_close_multiple_arguments_close() {
    assert_is_close!(10.0, 9.0, rel_tol = 1e-1, method = is_close::WEAK);
}

#[test]
#[should_panic = "assertion `is_close!(9.0, 10.0, rel_tol = 1e-1, method = is_close::STRONG)` failed"]
fn assert_is_close_multiple_arguments_far() {
    assert_is_close!(9.0, 10.0, rel_tol = 1e-1, method = is_close::STRONG);
}
