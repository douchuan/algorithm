//! Used by unit test to verify no memory leak.

use std::sync::atomic::{AtomicUsize, Ordering};

// record how many Drop::drop called
// tests run in concurrent, DROPS should be thread_local
thread_local! {
    static DROPS: AtomicUsize = AtomicUsize::new(0);
}

pub struct Elem;
// wrapper for DROPS
pub struct Ctx;

// drops counter +1
impl Drop for Elem {
    fn drop(&mut self) {
        DROPS.with(|drops| drops.fetch_add(1, Ordering::SeqCst));
    }
}

impl Ctx {
    /// count of Drop::drop called
    pub fn get(&self) -> usize {
        DROPS.with(|drops| drops.load(Ordering::SeqCst))
    }
}

pub fn with<F>(f: F)
where
    F: FnOnce(Ctx),
{
    DROPS.with(|drops| {
        // reset DROPS to 0
        drops.store(0, Ordering::SeqCst);
        f(Ctx)
    });
}
