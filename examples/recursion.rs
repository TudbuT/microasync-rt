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
