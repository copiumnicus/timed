# timed

Highest value primitive I made:

```rust
fn production_cpu_or_io_bound_task() {
    let _g = timed::measure("IS THIS SLOW?");
    // do stuff
    // ...
    // _g gets dropped and logs execution time using `tracing` crate
    // you still need your own tracing_subscriber setup ofc
}
// other option you are interested in cases where execution time spiked
use std::time::Duration;

fn production_cpu_or_io_bound_task() {
    let _g = timed::measure_log_over("found you", Duration::from_millis(2));
    // ...
    // _g gets dropped logs if over
}
```