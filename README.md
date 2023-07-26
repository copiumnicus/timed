# timed

Highest value primitive I made:

```rust
fn production_cpu_or_io_bound_task() {
    let _g = timed::measure("IS THIS SLOW?");
    // do stuff
    // ...
    // _g gets dropped and logs execution time using `tracing` crate
}
```