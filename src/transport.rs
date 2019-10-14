use std::net::TcpStream;

use crate::proxy::Proxy;
use crate::uri::Uri;
use crate::error::{Result};

// #[derive(Clone)]
pub struct Transport {
    proxy: Option<Proxy>,
    stream: Option<TcpStream>,
}

impl Transport {
    pub fn new() -> Self {
        Transport{
            proxy: None,
            stream: None,
        }
    }

    pub fn proxy(uri: Uri) -> Result<Self> {
        Ok(Transport{
            proxy: Some(Proxy::parse(uri)?),
            stream: None,
        })
    }

    pub fn proxy_http(uri: Uri) -> Result<Self> {
        Ok(Transport{
            proxy: Some(Proxy::http(uri)?),
            stream: None,
        })
    }

    pub fn proxy_https(uri: Uri) -> Result<Self> {
        Ok(Transport{
            proxy: Some(Proxy::https(uri)?),
            stream: None,
        })
    }

    pub fn proxy_socks(uri: Uri) -> Result<Self> {
        Ok(Transport{
            proxy: Some(Proxy::socks(uri)?),
            stream: None,
        })
    }
}