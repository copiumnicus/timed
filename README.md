# timed

Precise timing and performance measurement using tracing.

## Features

- 🚀 High-precision timing measurements using system time
- 📊 RAII-based measurement utilities that automatically log on drop
- 🔍 Conditional logging based on duration thresholds
- ⏰ Timer utilities for expiration and age tracking
- 🔗 Seamless integration with the `tracing` crate
- 🪶 Minimal runtime overhead with disabled tracing

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
timed = "0.1.0"
```

## Usage Examples

### Basic Performance Measurement

```rust
use timed::measure;

fn process_data() {
    let _timer = measure("data_processing");
    // Your code here
    // Duration will be automatically logged when _timer is dropped
}
```

### Threshold-based Logging

Only log when execution time exceeds a specified threshold:

```rust
use timed::measure_log_over;
use std::time::Duration;

fn performance_sensitive_operation() {
    let _timer = measure_log_over(
        "slow_operation",
        Duration::from_millis(100)
    );
    // Only logs if execution takes longer than 100ms
}
```

### Timer Utilities

Create expiring timers for timeout functionality:

```rust
use timed::timer;
use std::time::Duration;

fn check_timeout() {
    let timer = timer::new(Duration::from_secs(5));

    // Check if timer has expired
    if timer.is_finished() {
        println!("Operation timed out!");
    }
}
```

### Age Tracking

Track how old a timestamp is:

```rust
use timed::{now, how_old};
use std::time::Duration;

fn check_staleness() {
    let timestamp = now();
    // ... some time passes ...
    let age = how_old(timestamp);

    if age > Duration::from_secs(60) {
        println!("Data is stale!");
    }
}
```
