use std::cell::RefCell;
use std::rc::Rc;
use std::sync::atomic::{AtomicI32, Ordering};
use std::{convert, panic};

use super::*;
use pretty_assertions::assert_eq;
use test_case::test_case;

static COUNTER: AtomicI32 = AtomicI32::new(0);

fn count<T>(_ignored: T) -> i32 {
    COUNTER.fetch_add(1, Ordering::Relaxed)
}

fn always_panics<T>(_ignored: T) -> ! {
    panic!("always panics!")
}

#[test_case([1, 2, 3, 4], convert::identity, true)]
#[test_case([1, 2, 3, 3], convert::identity, false)]
#[test_case(["a", "b", "c"], str::len, false)]
#[test_case(["", "ab", "c"], str::len, true)]
#[test_case([Rc::new(RefCell::new(2)), Rc::new(RefCell::new(2))], |r| r.as_ptr(), true)]
#[test_case([1, 1, 1], count, true)]
#[test_case([] as [(); 0], always_panics, true)]
#[test_case([] as [(); 0], convert::identity, true)]
fn test_all_unique_by_identity<T, U: Eq + Hash + Clone, F: FnMut(T) -> U>(
    items: impl IntoIterator<Item = T>,
    f: F,
    all_unique: bool,
) {
    let items = Vec::from_iter(items);
    assert_eq!(items.into_iter().all_unique_by_key(f), all_unique);
}
