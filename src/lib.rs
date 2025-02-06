pub mod measure;
mod time;
pub mod timer;
pub use measure::{measure, measure_log_over};
pub use time::{how_old, how_old_against, now, over_max_age};

/// Returns the current system time in milliseconds since UNIX_EPOCH
pub fn now_ms() -> u64 {
    now().as_millis() as u64
}

/// Returns the current system time in seconds since UNIX_EPOCH
pub fn now_secs() -> u64 {
    now().as_secs() as u64
}
