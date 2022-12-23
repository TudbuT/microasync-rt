# microasync-util

[MicroAsync](https://crates.io/crates/microasync)
([GitHub](https://github.com/tudbut/microasync)) does not have many features, no IO
support, no proper runtime. MicroAsync-Util provides such things:
- A small runtime with the ability to add tasks (`no_std` supported)
- A small timer
- AsyncIO for Files, TCP, and UDP

## QueuedRuntime

QueuedRuntime is a very small async runtime with support for adding more tasks while it is
running. New tasks MUST only be added from within tasks already running on it or before it
is awaited!

```rs
use microasync::sync;
use microasync_util::{QueuedRuntime, wait_ms};

fn main() {
    let mut runtime = QueuedRuntime::new();
    for _ in 0..50 {
        runtime.push(print_something_after_ms(2000));
    }
    sync(runtime);
}

async fn print_something_after_ms(ms: u64) {
    wait_ms(ms).await;
    println!("something! :D");
}
```

```rs
use microasync::sync;
use microasync_util::{QueuedRuntime, wait_ms, get_current_runtime};

fn main() {
    let mut runtime = QueuedRuntime::new();
    runtime.push(print_something_after_ms(0));
    sync(runtime);
}

async fn print_something_after_ms(ms: u64) {
    wait_ms(ms).await;
    println!("something after {ms}ms! :D");
    get_current_runtime().await.push(print_something_after_ms(ms + 1));
}
```

## Examples

There are a bunch of examples in examples/ - feel free to check those out!
