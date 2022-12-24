use std::{net::TcpStream, time::SystemTime};

use microasync::{join, sync};
use microasync_util::{
    io::{ReadAsync, WriteAsync},
    wait_ms,
};

fn main() {
    // Connecting cannot be done async (for now), so we won't include that in the timing.
    let tcp = TcpStream::connect(("google.com", 80)).expect("connection refused");
    let start = SystemTime::now();
    println!(
        "{}",
        sync(join!(request("google.com", tcp, "GET /"), async {
            wait_ms(1000).await;
            "".to_owned()
        }))[0]
    );
    println!("Took {}ms.", start.elapsed().unwrap().as_millis());
}

async fn request(host: &str, mut tcp: TcpStream, method: &str) -> String {
    tcp.write(
        (method.to_owned() + " HTTP/1.1\r\nHost: " + host + "\r\nConnection: close\r\n\r\n")
            .as_bytes(),
    )
    .await
    .expect("connection broke");
    let mut buf = [0_u8; 64];
    let mut r = String::new();
    loop {
        let n = tcp.read(&mut buf).await.expect("connection broke");
        if n == 0 {
            break;
        }
        r += buf[0..n]
            .iter()
            .map(|x| *x as char)
            .collect::<String>()
            .as_str();
    }
    r
}
