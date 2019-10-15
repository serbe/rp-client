use std::net::TcpStream;

use crate::error::Result;
use crate::proxy::Proxy;
use crate::uri::Uri;
use crate::stream::Stream;

#[derive(Debug)]
pub enum Transport {
    Proxy(Proxy),
    Stream(Stream),
    None,
}

impl Default for Transport {
    fn default() -> Self {
        Transport::None
    }
}

impl Transport {
    pub fn new() -> Self {
        Transport::default()
    }

    pub fn proxy(uri: &Uri) -> Result<Self> {
        Ok(Transport::Proxy(Proxy::parse(uri)?))
    }

    pub fn proxy_http(uri: &Uri) -> Result<Self> {
        Ok(Transport::Proxy(Proxy::http(uri)?))
    }

    pub fn proxy_https(uri: &Uri) -> Result<Self> {
        Ok(Transport::Proxy(Proxy::https(uri)?))
    }

    pub fn proxy_socks(uri: &Uri) -> Result<Self> {
        Ok(Transport::Proxy(Proxy::socks(uri)?))
    }
}
