use std::net::TcpStream;

use crate::error::Result;
use crate::proxy::Proxy;
use crate::uri::Uri;

#[derive(Debug)]
pub struct Transport {
    proxy: Option<Proxy>,
    stream: Option<TcpStream>,
}

impl Default for Transport {
    fn default() -> Self {
        Transport{proxy: None, stream: None}
    }
}

impl Transport {
    pub fn new() -> Self {
        Transport::default()
    }

    pub fn proxy(uri: Uri) -> Result<Self> {
        Ok(Transport {
            proxy: Some(Proxy::parse(uri)?),
            stream: None,
        })
    }

    pub fn proxy_http(uri: Uri) -> Result<Self> {
        Ok(Transport {
            proxy: Some(Proxy::http(uri)?),
            stream: None,
        })
    }

    pub fn proxy_https(uri: Uri) -> Result<Self> {
        Ok(Transport {
            proxy: Some(Proxy::https(uri)?),
            stream: None,
        })
    }

    pub fn proxy_socks(uri: Uri) -> Result<Self> {
        Ok(Transport {
            proxy: Some(Proxy::socks(uri)?),
            stream: None,
        })
    }
}
