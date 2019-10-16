use crate::error::Result;
use crate::proxy::Proxy;
use crate::uri::Uri;
use crate::http::HttpStream;

#[derive(Debug)]
pub enum Transport {
    Proxy(Proxy),
    Stream(HttpStream),
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

    pub fn proxy(proxy: &Uri, target: &Uri) -> Result<Self> {
        Ok(Transport::Proxy(Proxy::proxy(proxy, target)?))
    }

    pub fn stream(uri: &Uri) -> Result<Self> {
        Ok(Transport::Stream(HttpStream::connect(uri)?))
    }
}
