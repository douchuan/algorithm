//! Used by unit test to verify no memory leak.

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

// record how many Drop::drop called
// tests run in concurrent, DROPS should be thread_local
thread_local! {
    static DROPS: Arc<AtomicUsize> = Arc::new(AtomicUsize::new(0));
}

pub struct Elem {
    drops: Arc<AtomicUsize>,
}

// wrapper for DROPS
pub struct Ctx {
    drops: Arc<AtomicUsize>,
}

impl Default for Elem {
    fn default() -> Self {
        let drops = DROPS.with(|drops| drops.clone());
        Self { drops }
    }
}

impl Drop for Elem {
    fn drop(&mut self) {
        self.drops.fetch_add(1, Ordering::SeqCst);
    }
}

impl Ctx {
    /// count of Drop::drop called
    pub fn get(&self) -> usize {
        self.drops.load(Ordering::SeqCst)
    }
}

pub fn with<F>(f: F)
where
    F: FnOnce(Ctx),
{
    DROPS.with(|drops| {
        // reset DROPS to 0
        drops.store(0, Ordering::SeqCst);
        let ctx = Ctx {
            drops: drops.clone(),
        };
        f(ctx)
    });
}
