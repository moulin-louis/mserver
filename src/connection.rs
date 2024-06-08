use std::error::Error;
use std::io;
use std::io::{Read, Write};
use std::net::TcpStream;

use mserialize::MSerialize;

#[derive(Debug)]
pub struct ClientConnection {
    pub tcp_stream: TcpStream,
}

impl ClientConnection {
    pub fn set_nonblocking(&self, nonblocking: bool) -> io::Result<()> {
        self.tcp_stream.set_nonblocking(nonblocking)
    }

    pub fn new(tcp_stream: TcpStream) -> Self {
        ClientConnection {
            tcp_stream
        }
    }

    pub fn write_struct<T>(&mut self, payload: T) -> Result<usize, Box<dyn Error>>
        where T: MSerialize {
        Ok(0)
    }
}

impl Write for ClientConnection {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.tcp_stream.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.tcp_stream.flush()
    }

    fn write_all(&mut self, buf: &[u8]) -> io::Result<()> {
        self.write_all(buf)
    }
}

impl Read for ClientConnection {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.tcp_stream.read(buf)
    }
}