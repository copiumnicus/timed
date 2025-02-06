use super::time::now;
use std::time::Duration;
use tracing::debug;

/// A struct for measuring and logging execution time
/// 
/// Implements Drop to automatically log the elapsed time when the measure goes out of scope
pub struct Measure {
    /// The start time of the measurement
    pub start: Duration,
    /// The message to log with the timing information
    pub message: String,
    /// Optional threshold duration - only log if elapsed time exceeds this value
    pub log_over: Option<Duration>,
}

impl Measure {
    /// Calculates the elapsed time since measurement started
    /// 
    /// # Returns
    /// * The duration since measurement started, or zero if clock went backwards
    pub fn elapsed(&self) -> Duration {
        let end = now();
        let diff = if end > self.start {
            end - self.start
        } else {
            Duration::default()
        };
        diff
    }
}

impl Drop for Measure {
    fn drop(&mut self) {
        let diff = self.elapsed();
        if let Some(over) = self.log_over {
            // don't log if fast
            if diff < over {
                return;
            }
        }
        debug!("T: {:?} = {}", diff, self.message)
    }
}

/// Creates a new measurement that only logs if execution time exceeds a threshold
/// 
/// # Arguments
/// * `msg` - The message to log with the timing information
/// * `log_over` - Only log if execution time exceeds this duration
/// 
/// # Returns
/// * A new Measure instance that will log timing when dropped
pub fn measure_log_over(msg: impl ToString, log_over: Duration) -> Measure {
    let message: String = msg.to_string();
    Measure {
        start: now(),
        message,
        log_over: Some(log_over),
    }
}

/// Creates a new measurement that always logs execution time
/// 
/// # Arguments
/// * `msg` - The message to log with the timing information
/// 
/// # Returns
/// * A new Measure instance that will log timing when dropped
pub fn measure(msg: impl ToString) -> Measure {
    let message: String = msg.to_string();
    Measure {
        start: now(),
        message,
        log_over: None,
    }
}
