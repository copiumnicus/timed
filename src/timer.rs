use crate::time::now;
use std::time::Duration;

pub struct Timer {
    end: Duration,
}

pub fn new(expire_offset: Duration) -> Timer {
    Timer::new(expire_offset)
}

impl Timer {
    pub fn new(expire_offset: Duration) -> Self {
        Self {
            end: now() + expire_offset,
        }
    }

    pub fn is_finished(&self) -> bool {
        now() > self.end
    }
}
