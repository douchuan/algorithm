//! Used by unit test to verify no memory leak.

use std::sync::{Arc, Mutex};

// record how many Drop::drop called
// tests run in concurrent, DROPS should be thread_local
thread_local! {
    static DROPS: Arc<Mutex<usize>> = Arc::new(Mutex::new(0));
}

pub struct Elem {
    drops: Arc<Mutex<usize>>,
}

// wrapper for DROPS
pub struct Ctx {
    drops: Arc<Mutex<usize>>,
}

impl Default for Elem {
    fn default() -> Self {
        let drops = DROPS.with(|drops| drops.clone());
        Self { drops }
    }
}

impl Drop for Elem {
    fn drop(&mut self) {
        let mut drops = self.drops.lock().unwrap();
        *drops += 1;
    }
}

impl Ctx {
    /// count of Drop::drop called
    pub fn get(&self) -> usize {
        *self.drops.lock().unwrap()
    }
}

pub fn with<F>(f: F)
where
    F: FnOnce(Ctx),
{
    DROPS.with(|drops| {
        // reset DROPS to 0
        *drops.lock().unwrap() = 0;
        let ctx = Ctx {
            drops: drops.clone(),
        };
        f(ctx)
    });
}
