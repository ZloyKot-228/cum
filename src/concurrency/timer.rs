use std::{
    cell::RefCell,
    time::{Duration, Instant},
};

#[derive(Default)]
pub struct Timer {
    start: RefCell<Option<Instant>>,
    elapsed: RefCell<Option<Duration>>,
    running: RefCell<bool>,
}

impl Timer {
    /// If timer is running - do nothing.
    pub fn start(&self) {
        if !*self.running.borrow() {
            *self.start.borrow_mut() = Some(Instant::now());
            *self.running.borrow_mut() = true;
        }
    }

    /// If timer is not running - do nothing.
    pub fn stop(&self) {
        if *self.running.borrow() && self.start.borrow().is_some() {
            *self.elapsed.borrow_mut() = Some(self.start.borrow().unwrap().elapsed());
            *self.start.borrow_mut() = None;
            *self.running.borrow_mut() = false;
        }
    }

    pub fn elapsed_float(&self) -> Option<f64> {
        self.elapsed.borrow().as_ref().map(|d| d.as_secs_f64())
    }
}
