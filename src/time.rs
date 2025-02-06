use std::time::{Duration, SystemTime, UNIX_EPOCH};

/// Returns the current system time as a Duration since UNIX_EPOCH
pub fn now() -> Duration {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap()
}

/// Calculates the duration between two time samples
/// 
/// # Arguments
/// * `last_sample` - The earlier time sample
/// * `current_sample` - The later time sample
/// 
/// # Returns
/// * Returns the duration between samples, or zero if current_sample is earlier than last_sample
pub fn how_old_against(last_sample: Duration, current_sample: Duration) -> Duration {
    if current_sample > last_sample {
        return current_sample - last_sample;
    }
    Duration::from_secs(0)
}

/// Checks if a timestamp is older than a maximum age
/// 
/// # Arguments
/// * `timestamp` - The timestamp to check
/// * `max_age` - The maximum allowed age
/// 
/// # Returns
/// * `true` if the timestamp is older than max_age, `false` otherwise
pub fn over_max_age(timestamp: Duration, max_age: Duration) -> bool {
    how_old(timestamp) > max_age
}

/// Calculates how old a timestamp is relative to now
/// 
/// # Arguments
/// * `start` - The timestamp to measure age from
/// 
/// # Returns
/// * The duration since the start timestamp, or zero if start is in the future
pub fn how_old(start: Duration) -> Duration {
    // NOW 1661076248230489000 THEN 1661076248234000000
    // thread 'tokio-runtime-worker' panicked at 'attempt to subtract with overflow'
    // -0.003511 -> 3 milis out of sync?
    // lets say that if this is out of sync like this we pat it to 0 world is crooked
    let now = now();
    if now < start {
        return Duration::from_nanos(0);
    }
    let diff = now - start;
    diff
}
