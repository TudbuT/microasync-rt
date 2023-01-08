use microasync::sync;
use microasync_rt::io::ReadAsync;
use std::{
    fs::File,
    io::{self, Seek, SeekFrom},
};

fn main() {
    let f = File::open("Cargo.toml").unwrap();
    println!("{}", sync(read(f)).unwrap());
}

async fn read(mut f: File) -> Result<String, io::Error> {
    let cur = f.seek(SeekFrom::Current(0))?;
    let len = f.seek(SeekFrom::End(0))?;
    f.seek(SeekFrom::Start(cur))?;

    let mut v = String::new();
    let mut buf = [0_u8; 1024];
    let mut n = 0_usize;
    while n != len as usize {
        n += f.read(&mut buf).await?;
        v.push_str(buf.iter().map(|x| *x as char).collect::<String>().as_str());
    }
    Ok(v)
}
