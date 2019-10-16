use std::net::TcpStream;

use crate::error::{Result};
use crate::stream::Stream;
use crate::uri::Uri;

#[derive(Debug)]
pub struct HttpStream {
    stream: Stream,
    // target: Addr,
    // is_proxy: bool,
    // bind_addr: Host,
    // bind_port: [u8; 2],
}

impl HttpStream {
    pub fn connect(uri: &Uri) -> Result<Self> {
        let target = uri.socket_addr()?;
        let stream = TcpStream::connect(target)?;
        let stream = if uri.is_ssl() {
            Stream::new_tls(uri.host(), stream)?
        } else {
            Stream::new_tcp(stream)
        };
        Ok(HttpStream {
            stream,
            // target,
            // is_proxy: true,
        })
    }

    pub fn connect_proxy(uri: &Uri) -> Result<Self> {
        let target = uri.socket_addr()?;
        // let proxy_addr: Addr = proxy.parse()?;
        let stream = TcpStream::connect(target)?;
        // let stream = if proxy_addr.is_ssl() {
        //     Stream::new_tls(&proxy_addr.host()?, stream)?
        // } else {
        //     Stream::new_tcp(stream)
        // };
        let stream = Stream::new_tcp(stream);
        Ok(HttpStream {
            stream,
            // target,
            // is_proxy: true,
        })
    }

    // pub fn get(&mut self) -> io::Result<Vec<u8>> {
    //     let request = format!(
    //         "GET {} HTTP/1.0\r\nHost: {}\r\n\r\n",
    //         self.target.path(),
    //         self.target.host()?
    //     )
    //     .into_bytes();
    //     self.stream.write_all(&request)?;
    //     self.stream.flush()?;
    //     let mut response = vec![];
    //     self.stream.read_to_end(&mut response)?;
    //     let pos = response
    //         .windows(4)
    //         .position(|x| x == b"\r\n\r\n")
    //         .ok_or_else(|| Error::WrongHttp)?;
    //     let body = &response[pos + 4..response.len()];
    //     Ok(body.to_vec())
    // }

    // pub fn post_json(&mut self, body: &str) -> io::Result<Vec<u8>> {
    //     let body = if !body.is_empty() {
    //         format!("Content-Length: {}\r\n\r\n{}", body.len(), body)
    //     } else {
    //         String::new()
    //     };
    //     let request = format!(
    //         "POST {} HTTP/1.0\r\nHost: {}\r\nContent-Type: application/json\r\n{}\r\n",
    //         self.target.path(),
    //         self.target.host()?,
    //         body
    //     )
    //     .into_bytes();
    //     self.stream.write_all(&request)?;
    //     self.stream.flush()?;
    //     let mut response = vec![];
    //     self.stream.read_to_end(&mut response)?;
    //     let pos = response
    //         .windows(4)
    //         .position(|x| x == b"\r\n\r\n")
    //         .ok_or_else(|| Error::WrongHttp)?;
    //     let body = &response[pos + 4..response.len()];
    //     Ok(body.to_vec())
    // }
}

// impl Read for HttpStream {
//     fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
//         self.stream.read(buf)
//     }
// }

// impl Write for HttpStream {
//     fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
//         self.stream.write(buf)
//     }

//     fn flush(&mut self) -> io::Result<()> {
//         self.stream.flush()
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn http() {
//         let mut client = HttpStream::connect("http://api.ipify.org").unwrap();
//         let body = client.get().unwrap();
//         let txt = String::from_utf8_lossy(&body);
//         assert!(txt.contains(crate::tests::IP.as_str()));
//     }

//     #[test]
//     fn https() {
//         let mut client = HttpStream::connect("https://api.ipify.org").unwrap();
//         let body = client.get().unwrap();
//         let txt = String::from_utf8_lossy(&body);
//         assert!(txt.contains(crate::tests::IP.as_str()));
//     }

//     #[test]
//     fn http_proxy() {
//         let mut client =
//             HttpStream::connect_proxy("127.0.0.1:5858", "https://api.ipify.org").unwrap();
//         let body = client.get().unwrap();
//         let txt = String::from_utf8_lossy(&body);
//         assert!(txt.contains(crate::tests::IP.as_str()));
//     }
// }
