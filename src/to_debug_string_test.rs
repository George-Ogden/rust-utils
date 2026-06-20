use std::fmt::{self, Display};

use super::*;
use pretty_assertions::assert_eq;

#[test]
fn test_to_string_debug_auto_impl() {
    #[derive(Debug)]
    struct DebugAutoImpl {
        num: i32,
        float: f64,
    }

    let instance = DebugAutoImpl {
        num: 10,
        float: 15.0,
    };

    assert_eq!(
        instance.to_debug_string(),
        "DebugAutoImpl { num: 10, float: 15.0 }"
    );
    assert_eq!(
        instance.to_debug_string_pretty(),
        textwrap::dedent(
            r"
            DebugAutoImpl {
                num: 10,
                float: 15.0,
            }
            "
        )
        .trim()
    );
}

#[test]
fn test_to_string_debug_manual_impl() {
    struct DebugManualImpl;

    impl Display for DebugManualImpl {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "display")
        }
    }

    impl Debug for DebugManualImpl {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "debug")
        }
    }

    assert_eq!(DebugManualImpl.to_debug_string(), "debug");
    assert_eq!(DebugManualImpl.to_debug_string_pretty(), "debug");
}

#[test]
fn test_to_string_manual_impl() {
    struct ToDebugStringManualImpl;

    impl ToDebugString for ToDebugStringManualImpl {
        fn to_debug_string(&self) -> String {
            String::from("debug string")
        }

        fn to_debug_string_pretty(&self) -> String {
            String::from("debug string pretty")
        }
    }

    assert_eq!(ToDebugStringManualImpl.to_debug_string(), "debug string");
    assert_eq!(
        ToDebugStringManualImpl.to_debug_string_pretty(),
        "debug string pretty"
    );
}
