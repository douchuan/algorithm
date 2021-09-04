use std::sync::{Arc, Mutex};

// tests run in concurrent, DROPS should be thread_local
thread_local! {
    static DROPS: Arc<Mutex<usize>> = Arc::new(Mutex::new(0));
}

pub struct Elem {
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

pub fn start() {
    DROPS.with(|drops| *drops.lock().unwrap() = 0)
}

pub fn peek() -> usize {
    DROPS.with(|drops| *drops.lock().unwrap())
}

pub fn end() -> usize {
    DROPS.with(|drops| *drops.lock().unwrap())
}
