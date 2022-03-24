use std::io::{BufWriter, Write};
use std::net::TcpStream;

#[derive(Debug)]
struct MyWriter<W: Write> {
    writer: W,
}

impl<W: Write> MyWriter<W> {
    pub fn new(writer: W) -> MyWriter<W> {
        Self {
            writer,
        }
    }
    pub fn write(&mut self, buf: &str) -> std::io::Result<()> {
        self.writer.write_all(buf.as_bytes())
    }
}

fn main() {
    let s = 2;

    match s {
        1 => {
            let stream = TcpStream::connect("127.0.0.1:8080").unwrap();
            let mut my_writer = MyWriter::new(stream);
            my_writer.write("hello world!");
        }
        2 => {
            let stream = TcpStream::connect("127.0.0.1:8080").unwrap();
            let writer = BufWriter::new(stream);
            let mut my_writer = MyWriter::new(writer);
            my_writer.write("hello world!");
        }
        _ => {}
    }
}