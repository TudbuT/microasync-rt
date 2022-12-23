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
