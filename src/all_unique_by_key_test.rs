use std::cell::RefCell;
use std::rc::Rc;
use std::sync::atomic::{AtomicI32, Ordering};
use std::{convert, hash, iter, panic};

use super::*;
use derive_more::Constructor;
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

#[test]
fn test_all_unique_by_lazy() {
    let mut count = 0;
    let result = vec![0; 4].into_iter().all_unique_by_key(|x| {
        count += 1;
        x
    });
    assert!(!result);
    assert_eq!(count, 2);
}

#[derive(Constructor)]
pub struct Destroy(Rc<RefCell<i32>>);

impl Drop for Destroy {
    fn drop(&mut self) {
        *self.0.borrow_mut() += 1;
    }
}

impl PartialEq for Destroy {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}
impl Eq for Destroy {}
impl Hash for Destroy {
    fn hash<H: hash::Hasher>(&self, _state: &mut H) {}
}

#[test]
fn test_all_unique_by_destructor() {
    let count = Rc::new(RefCell::new(0));
    let items = iter::repeat_with(|| Destroy::new(Rc::clone(&count)))
        .take(4)
        .collect_vec();
    let result = items.into_iter().all_unique_by_key(convert::identity);
    assert!(!result);
    assert_eq!(*count.borrow(), 4);
}
