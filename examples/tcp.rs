use std::{
    io::{self, Write},
    net::{SocketAddr, TcpListener, TcpStream},
};

use microasync::sync;
use microasync_rt::{
    get_current_runtime,
    io::{read::tcpstream::accept, ReadAsync},
    wait_ms, QueuedRuntime, Runtime,
};

fn main() {
    let mut runtime = QueuedRuntime::new();
    runtime.push(go(("0.0.0.0", 5000)));
    sync(runtime);
}

async fn go(addr: (&str, u16)) {
    let mut listener = TcpListener::bind(addr).unwrap();
    loop {
        get_current_runtime()
            .await
            .push(handle(accept(&mut listener).await.unwrap()));
    }
}

async fn handle((mut stream, _): (TcpStream, SocketAddr)) {
    let mut buf = [0_u8; 10];
    loop {
        let n = stream.read(&mut buf).await.unwrap();
        if n == 0 {
            break;
        }
        print!(
            "{}",
            buf[0..n].iter().map(|x| *x as char).collect::<String>()
        );
        io::stdout().flush().unwrap();
        wait_ms(100).await; // Delay so multiple connections can accumulate, for demonstration
                            // purposes.
    }
}
