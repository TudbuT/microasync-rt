extern crate std;

use core::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};
use std::time::{Duration, SystemTime};

pub struct Timer {
    length: Duration,
    start: SystemTime,
}

impl Future for Timer {
    type Output = ();

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.start.elapsed().unwrap_or(self.length) >= self.length {
            Poll::Ready(())
        } else {
            Poll::Pending
        }
    }
}

pub async fn wait(duration: Duration) {
    Timer {
        length: duration,
        start: SystemTime::now(),
    }
    .await
}

pub async fn wait_ms(ms: u64) {
    Timer {
        length: Duration::from_millis(ms),
        start: SystemTime::now(),
    }
    .await
}
