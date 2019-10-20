use crate::error::{Error, Result};
use crate::http::HttpStream;
use crate::response::Response;
use crate::socks::SocksStream;
use crate::uri::Uri;

#[derive(Debug)]
pub struct HttpProxy {
    stream: HttpStream,
}

#[derive(Debug)]
pub struct SocksProxy {
    stream: SocksStream,
}

#[derive(Debug)]
pub enum Proxy {
    Http(HttpProxy),
    Https(HttpProxy),
    Socks(SocksProxy),
}

impl Proxy {
    pub fn proxy(proxy: &Uri, target: &Uri) -> Result<Proxy> {
        match proxy.scheme() {
            "http" => Proxy::http(proxy),
            "https" => Proxy::https(proxy),
            "socks5" => Proxy::socks5(proxy, target),
            "socks5h" => Proxy::socks5h(proxy, target),
            s => Err(Error::UnsupportedScheme(s.to_owned())),
        }
    }

    pub fn http(proxy: &Uri) -> Result<Proxy> {
        Ok(Proxy::Http(HttpProxy {
            stream: HttpStream::connect_proxy(proxy)?,
        }))
    }

    pub fn https(proxy: &Uri) -> Result<Proxy> {
        Ok(Proxy::Https(HttpProxy {
            stream: HttpStream::connect_proxy(proxy)?,
        }))
    }

    pub fn socks5(proxy: &Uri, target: &Uri) -> Result<Proxy> {
        Ok(Proxy::Socks(SocksProxy {
            stream: SocksStream::connect(proxy, target)?,
        }))
    }

    pub fn socks5h(proxy: &Uri, target: &Uri) -> Result<Proxy> {
        Ok(Proxy::Socks(SocksProxy {
            stream: SocksStream::connect(proxy, target)?,
        }))
    }

    pub fn send_request(&mut self, req: &[u8]) -> Result<()> {
        match self {
            Proxy::Http(http_proxy) => http_proxy.stream.send_request(req),
            Proxy::Https(http_proxy) => http_proxy.stream.send_request(req),
            Proxy::Socks(socks_proxy) => socks_proxy.stream.send_request(req),
        }
    }

    pub fn get_response(&mut self) -> Result<Response> {
        match self {
            Proxy::Http(http_proxy) => http_proxy.stream.get_response(),
            Proxy::Https(http_proxy) => http_proxy.stream.get_response(),
            Proxy::Socks(socks_proxy) => socks_proxy.stream.get_response(),
        }
    }

    pub fn get_body(&mut self, content_len: usize) -> Result<Vec<u8>> {
        match self {
            Proxy::Http(http_proxy) => http_proxy.stream.get_body(content_len),
            Proxy::Https(http_proxy) => http_proxy.stream.get_body(content_len),
            Proxy::Socks(socks_proxy) => socks_proxy.stream.get_body(content_len),
        }
    }
}
