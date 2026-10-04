#[macro_export]
macro_rules! assert_is_close {
    ($left:expr, $right:expr) => {
        use is_close::is_close;
        let left = $left;
        let right = $right;

        assert!(
            is_close!(left, right),
            r"assertion `is_close!({left:?}, {right:?})` failed",
        );
    };
}

#[cfg(test)]
#[path = "assert_is_close_test.rs"]
mod test;
