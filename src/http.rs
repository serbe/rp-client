use std::io::{Read, Write};
use std::net::TcpStream;

use crate::error::{Error, Result};
use crate::request::Request;
use crate::stream::Stream;
use crate::uri::Uri;

#[derive(Debug)]
pub struct HttpStream {
    stream: Stream,
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
        Ok(HttpStream { stream })
    }

    pub fn connect_proxy(uri: &Uri) -> Result<Self> {
        let target = uri.socket_addr()?;
        let stream = TcpStream::connect(target)?;
        let stream = Stream::new_tcp(stream);
        Ok(HttpStream { stream })
    }

    pub fn get_body(&mut self, req: &Request) -> Result<String> {
        self.stream.write_all(&req.msg())?;
        self.stream.flush()?;
        let mut response = vec![];
        self.stream.read_to_end(&mut response)?;
        String::from_utf8(response).map_err(Error::FromUtf8)
    }
}

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
