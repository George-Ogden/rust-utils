use std::fmt::Debug;

/// Equivalent of the `ToString` trait, but using `Debug::fmt` instead of `Display::fmt`.
pub trait ToDebugString {
    fn to_debug_string(&self) -> String;
    fn to_debug_string_pretty(&self) -> String;
}

impl<T: Debug> ToDebugString for T {
    #[inline]
    fn to_debug_string(&self) -> String {
        format!("{self:?}")
    }

    #[inline]
    fn to_debug_string_pretty(&self) -> String {
        format!("{self:#?}")
    }
}

#[cfg(test)]
#[path = "to_debug_string_test.rs"]
mod test;
