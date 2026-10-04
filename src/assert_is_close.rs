#[doc(hidden)]
pub use is_close;

#[macro_export]
macro_rules! assert_is_close {
    ($left:expr, $right:expr $(, $set:ident = $val:expr)*) => {
        use $crate::assert_is_close::is_close::is_close;
        let left = $left;
        let right = $right;

        assert!(
            is_close!(left, right $(, $set = $val )*),
            r"assertion `is_close!({left:?}, {right:?}{})` failed",
            stringify!($(, $set = $val)*)
        );
    };
}

#[cfg(test)]
#[path = "assert_is_close_test.rs"]
mod test;
