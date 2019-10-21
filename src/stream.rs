use std::io::{self, Read, Write};
use std::net::TcpStream;

use native_tls::{TlsConnector, TlsStream};

use crate::error::Result;
use crate::response::Response;

#[derive(Debug)]
pub enum Stream {
    Tcp(TcpStream),
    Tls(Box<TlsStream<TcpStream>>),
}

impl Stream {
    pub fn new_tcp(stream: TcpStream) -> Self {
        Stream::Tcp(stream)
    }

    pub fn new_tls(domain: &str, stream: TcpStream) -> Result<Self> {
        let builder = TlsConnector::new()?;
        Ok(Stream::Tls(Box::new(builder.connect(domain, stream)?)))
    }

    pub fn send_msg(stream: &mut Stream, msg: &[u8]) -> Result<()> {
        stream.write_all(msg)?;
        stream.flush()?;
        Ok(())
    }

    pub fn read_head(stream: &mut Stream) -> Result<Response> {
        let mut head = Vec::with_capacity(200);
        copy_until(stream, &mut head, &[13, 10, 13, 10])?;
        Response::from_head(&head)
    }

    pub fn get_body(stream: &mut Stream, content_len: usize) -> Result<Vec<u8>> {
        let mut body = Vec::with_capacity(200);
        copy_until_len(stream, &mut body, content_len)?;
        Ok(body)
    }
}

impl Read for Stream {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        match self {
            Stream::Tcp(stream) => stream.read(buf),
            Stream::Tls(stream) => (*stream).read(buf),
        }
    }
}

impl Write for Stream {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match self {
            Stream::Tcp(stream) => stream.write(buf),
            Stream::Tls(stream) => (*stream).write(buf),
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        match self {
            Stream::Tcp(stream) => stream.flush(),
            Stream::Tls(stream) => (*stream).flush(),
        }
    }
}

pub fn copy_until<R, W>(reader: &mut R, writer: &mut W, val: &[u8]) -> Result<usize>
where
    R: Read + ?Sized,
    W: Write + ?Sized,
{
    let mut buf = Vec::with_capacity(200);

    let mut pre_buf = [0; 10];
    let mut read = reader.read(&mut pre_buf)?;
    buf.extend(&pre_buf[..read]);

    for byte in reader.bytes() {
        buf.push(byte?);
        read += 1;

        if &buf[(buf.len() - val.len())..] == val {
            break;
        }
    }

    writer.write_all(&buf)?;
    writer.flush()?;

    Ok(read)
}

pub fn copy_until_len<R, W>(reader: &mut R, writer: &mut W, len: usize) -> Result<usize>
where
    R: Read + ?Sized,
    W: Write + ?Sized,
{
    let mut buf = Vec::with_capacity(len);

    let mut pre_buf = [0; 10];
    let mut read = reader.read(&mut pre_buf)?;
    buf.extend(&pre_buf[..read]);

    for (i, byte) in reader.bytes().enumerate() {
        buf.push(byte?);
        read += 1;

        if i == len + 1 {
            break;
        }
    }

    writer.write_all(&buf)?;
    writer.flush()?;

    Ok(read)
}
