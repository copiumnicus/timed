use super::time::now;
use std::time::Duration;
use tracing::debug;

pub struct Measure {
    pub start: Duration,
    pub message: String,
    pub log_over: Option<Duration>,
}

impl Measure {
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

pub fn measure_log_over(msg: impl ToString, log_over: Duration) -> Measure {
    let message: String = msg.to_string();
    Measure {
        start: now(),
        message,
        log_over: Some(log_over),
    }
}

pub fn measure(msg: impl ToString) -> Measure {
    let message: String = msg.to_string();
    Measure {
        start: now(),
        message,
        log_over: None,
    }
}
