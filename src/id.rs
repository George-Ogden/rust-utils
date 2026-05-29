use derive_more::Display;
use std::sync::atomic::{AtomicUsize, Ordering};

#[cfg(not(any(test, feature = "test-utils")))]
static GLOBAL_ID: AtomicUsize = AtomicUsize::new(0);

#[cfg(any(test, feature = "test-utils"))]
thread_local! {
    static GLOBAL_ID: AtomicUsize = const { AtomicUsize::new(1) };
}

#[cfg(any(test, feature = "test-utils"))]
/// Set the global id to a known value.
#[inline]
pub fn set_global_id(id: usize) {
    GLOBAL_ID.with(|global_id| global_id.store(id, Ordering::SeqCst));
}

#[cfg(any(test, feature = "test-utils"))]
/// Set the global id back to zero.
#[inline]
pub fn reset_global_id() {
    set_global_id(0);
}

#[cfg(not(any(test, feature = "test-utils")))]
fn next_id() -> usize {
    GLOBAL_ID.fetch_add(1, Ordering::Relaxed)
}

#[cfg(any(test, feature = "test-utils"))]
#[must_use]
fn next_id() -> usize {
    GLOBAL_ID.with(|id| id.fetch_add(1, Ordering::Relaxed))
}

#[derive(Copy, Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash, Display)]
/// Unique auto-incrementing ids.
/// This is incremented with a global relaxed atomic.
/// During testing, this is thread-local so the [`set_global_id`] and [`reset_global_id`] methods can control the next id.
pub struct Id(usize);

impl Id {
    #[inline]
    #[must_use]
    /// Get the next unique id.
    pub fn next() -> Self {
        Self(next_id())
    }
}

#[cfg(test)]
#[path = "id_test.rs"]
mod test;
