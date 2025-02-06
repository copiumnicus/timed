use crate::time::now;
use std::time::Duration;

/// A timer that expires after a specified duration
pub struct Timer {
    end: Duration,
}

/// Creates a new Timer that expires after the specified duration
/// 
/// # Arguments
/// * `expire_offset` - Duration until the timer expires
pub fn new(expire_offset: Duration) -> Timer {
    Timer::new(expire_offset)
}

impl Timer {
    /// Creates a new Timer that expires after the specified duration
    /// 
    /// # Arguments
    /// * `expire_offset` - Duration until the timer expires
    pub fn new(expire_offset: Duration) -> Self {
        Self {
            end: now() + expire_offset,
        }
    }

    /// Checks if the timer has expired
    /// 
    /// # Returns
    /// * `true` if the current time is past the timer's end time, `false` otherwise
    pub fn is_finished(&self) -> bool {
        now() > self.end
    }
}
