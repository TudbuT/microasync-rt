use std::time::Duration;

use microasync::sync;
use microasync_rt::{get_current_runtime, QueuedRuntime, Runtime};

fn main() {
    let mut runtime = QueuedRuntime::new();
    runtime.push(print_something_after_ms(0));
    sync(runtime);
}

async fn print_something_after_ms(ms: u64) {
    get_current_runtime()
        .await
        .sleep(Duration::from_millis(ms))
        .await;
    println!("something after {ms}ms! :D");
    get_current_runtime()
        .await
        .push(print_something_after_ms(ms + 1));
}
