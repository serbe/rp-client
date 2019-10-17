use std::net::TcpStream;

use crate::error::Result;
use crate::request::Request;
use crate::response::Response;
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

    pub fn send_request(&mut self, req: &Request) -> Result<()> {
        Stream::send_msg(&mut self.stream, &req.msg())
    }

    pub fn get_response(&mut self) -> Result<Response> {
        Stream::read_head(&mut self.stream)
    }

    pub fn get_body(&mut self, content_len: usize) -> Result<Vec<u8>> {
        Stream::get_body(&mut self.stream, content_len)
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
