use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::io::{BufReader, BufWriter, Seek, SeekFrom};
fn main() {
    println!("Hello, world!");
    write().unwrap();

    read().unwrap();
}

fn write() -> std::io::Result<()> {
    let f = OpenOptions::new().append(true).open("log.txt")?;
    let mut writer = BufWriter::new(f);
    writer.write_all(String::from("hello").as_bytes())?;
    writer.flush()?;
    Ok(())
}

fn read() -> std::io::Result<()> {
    let mut f = File::open("log.txt")?;
    f.seek(SeekFrom::Start(10))?;
    let mut reader = BufReader::new(f);

    let mut buffer = String::new();
    reader.read_line(&mut buffer)?;
    println!("{buffer}");

    let mut readbuf = [0; 5];
    reader.read_exact(&mut readbuf)?;
    // println!("{:?}", readbuf);
    println!("{:?}", String::from_utf8(readbuf.to_vec()).unwrap());

    let mut buf: Vec<u8> = Vec::new();
    reader.read_to_end(&mut buf)?;
    println!("{:?}", String::from_utf8(buf.to_vec()).unwrap());

    Ok(())
}
