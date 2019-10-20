use std::net::TcpStream;

use crate::error::Result;
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

    pub fn send_request(&mut self, req: &[u8]) -> Result<()> {
        Stream::send_msg(&mut self.stream, req)
    }

    pub fn get_response(&mut self) -> Result<Response> {
        Stream::read_head(&mut self.stream)
    }

    pub fn get_body(&mut self, content_len: usize) -> Result<Vec<u8>> {
        Stream::get_body(&mut self.stream, content_len)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn http_stream_http() {
        let mut client =
            HttpStream::connect(&"http://api.ipify.org".parse::<Uri>().unwrap()).unwrap();
        client
            .send_request(b"GET / HTTP/1.0\r\nHost: api.ipify.org\r\n\r\n")
            .unwrap();
        let response = client.get_response().unwrap();
        let body = client.get_body(response.content_len().unwrap()).unwrap();
        let body = String::from_utf8(body).unwrap();
        assert!(&body.contains(crate::tests::IP.as_str()));
    }

    #[test]
    fn http_stream_https() {
        let mut client =
            HttpStream::connect(&"https://api.ipify.org".parse::<Uri>().unwrap()).unwrap();
        client
            .send_request(b"GET / HTTP/1.0\r\nHost: api.ipify.org\r\n\r\n")
            .unwrap();
        let response = client.get_response().unwrap();
        let body = client.get_body(response.content_len().unwrap()).unwrap();
        let body = String::from_utf8(body).unwrap();
        assert!(&body.contains(crate::tests::IP.as_str()));
    }

    #[test]
    fn http_stream_http_proxy() {
        let mut client =
            HttpStream::connect_proxy(&"http://127.0.0.1:5858".parse::<Uri>().unwrap()).unwrap();
        client
            .send_request(b"GET / HTTP/1.0\r\nHost: api.ipify.org\r\n\r\n")
            .unwrap();
        let response = client.get_response().unwrap();
        let body = client.get_body(response.content_len().unwrap()).unwrap();
        let body = String::from_utf8(body).unwrap();
        assert!(&body.contains(crate::tests::IP.as_str()));
    }

    #[test]
    fn http_stream_http_proxy_auth() {
        let mut client =
            HttpStream::connect_proxy(&"http://test:tset@127.0.0.1:5656".parse::<Uri>().unwrap())
                .unwrap();
        client
            .send_request(b"GET / HTTP/1.0\r\nHost: api.ipify.org\r\nProxy-Authorization: Basic dGVzdDp0c2V0\r\n\r\n")
            .unwrap();
        let response = client.get_response().unwrap();
        let body = client.get_body(response.content_len().unwrap()).unwrap();
        let body = String::from_utf8(body).unwrap();
        assert!(&body.contains(crate::tests::IP.as_str()));
    }
}
