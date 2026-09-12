use derive_more::{Display, From, Into};
use std::sync::atomic::{AtomicU32, Ordering};

#[cfg(not(feature = "test-utils"))]
static GLOBAL_ID: AtomicU32 = AtomicU32::new(0);

#[cfg(feature = "test-utils")]
thread_local! {
    static GLOBAL_ID: AtomicU32 = const { AtomicU32::new(1) };
}

#[cfg(feature = "test-utils")]
/// Set the global id to a known value.
#[inline]
pub fn set_global_id(id: u32) {
    GLOBAL_ID.with(|global_id| global_id.store(id, Ordering::SeqCst));
}

#[cfg(feature = "test-utils")]
/// Set the global id back to zero.
#[inline]
pub fn reset_global_id() {
    set_global_id(0);
}

#[cfg(not(feature = "test-utils"))]
fn next_id() -> u32 {
    GLOBAL_ID.fetch_add(1, Ordering::Relaxed)
}

#[cfg(feature = "test-utils")]
#[must_use]
fn next_id() -> u32 {
    GLOBAL_ID.with(|id| id.fetch_add(1, Ordering::Relaxed))
}

#[derive(Copy, Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash, Display, Into, From)]
/// Unique auto-incrementing ids.
/// This is incremented with a global relaxed atomic.
/// During testing, this is thread-local so the [`set_global_id`] and [`reset_global_id`] methods can control the next id.
pub struct Id(u32);

impl Id {
    #[inline]
    #[must_use]
    /// Get the next unique id.
    pub fn next() -> Self {
        Self(next_id())
    }
}

impl From<Id> for i64 {
    #[inline]
    fn from(id: Id) -> Self {
        Self::from(id.0)
    }
}

impl From<Id> for u64 {
    #[inline]
    fn from(id: Id) -> Self {
        Self::from(id.0)
    }
}

impl From<Id> for usize {
    #[inline]
    fn from(id: Id) -> Self {
        id.0 as Self
    }
}

#[cfg(test)]
#[path = "id_test.rs"]
mod test;
