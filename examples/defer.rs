use std::{thread, time::Duration};

use microasync::{sync, join};
use microasync_util::defer;


fn main() {
    println!("{}", sync(join!(test(), async { is_alive().await; "".to_owned() }))[0]);
}

async fn is_alive() {
    println!("The runtime is NOT blocked by the test() function: This future runs *after* the poll\
        to the test() function, so if this runs before test() is done, that means test() returned\
        Poll::Pending and is not blocking.");
}

async fn test() -> String {
    defer(|(s,)| {
        thread::sleep(Duration::from_millis(2000));
        s + "world"
    }, ("Hello, ".to_owned(),)).await
}
