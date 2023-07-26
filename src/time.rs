use std::time::{Duration, SystemTime, UNIX_EPOCH};

/// now nanosecs
pub fn now() -> Duration {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap()
}

pub fn how_old_against(last_sample: Duration, current_sample: Duration) -> Duration {
    if current_sample > last_sample {
        return current_sample - last_sample;
    }
    Duration::from_secs(0)
}

pub fn over_max_age(timestamp: Duration, max_age: Duration) -> bool {
    how_old(timestamp) > max_age
}

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
