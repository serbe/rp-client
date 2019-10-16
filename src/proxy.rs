use crate::error::{Error, Result};
use crate::http::HttpStream;
use crate::uri::Uri;
use crate::socks::SocksStream;

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
        Ok(Proxy::Http(HttpProxy{stream: HttpStream::connect_proxy(proxy)?}))
    }

    pub fn https(proxy: &Uri) -> Result<Proxy> {
        Ok(Proxy::Https(HttpProxy{stream: HttpStream::connect_proxy(proxy)?}))
    }

    pub fn socks5(proxy: &Uri, target: &Uri) -> Result<Proxy> {
        Ok(Proxy::Socks(SocksProxy{stream: SocksStream::connect(proxy, target)?}))
    }

    pub fn socks5h(proxy: &Uri, target: &Uri) -> Result<Proxy> {
        Ok(Proxy::Socks(SocksProxy{stream: SocksStream::connect(proxy, target)?}))
    }
}

//     // fn with_basic_auth<T: Into<String>, U: Into<String>>(
//     //     mut self,
//     //     username: T,
//     //     password: U,
//     // ) -> Self {
//     //     self.set_basic_auth(username, password);
//     //     self
//     // }

//     // fn set_basic_auth<T: Into<String>, U: Into<String>>(&mut self, username: T, password: U) {
//     //     match *self {
//     //         ProxyScheme::Http { ref mut auth, .. } => {
//     //             let header = encode_basic_auth(&username.into(), &password.into());
//     //             *auth = Some(header);
//     //         }
//     //         #[cfg(feature = "socks")]
//     //         ProxyScheme::Socks5 { ref mut auth, .. } => {
//     //             *auth = Some((username.into(), password.into()));
//     //         }
//     //     }
//     // }
// }


// impl Intercept {
// fn set_basic_auth(&mut self, username: &str, password: &str) {
//     match self {
//         Intercept::All(ref mut s)
//         | Intercept::Http(ref mut s)
//         | Intercept::Https(ref mut s) => s.set_basic_auth(username, password),
//         Intercept::Socks(ref mut s) => {
//             let header = encode_basic_auth(username, password);
//             custom.auth = Some(header);
//         }
//     }
// }
// }

// pub(crate) fn encode_basic_auth(username: &str, password: &str) -> HeaderValue {
//     let val = format!("{}:{}", username, password);
//     let mut header = format!("Basic {}", base64::encode(&val))
//         .parse::<HeaderValue>()
//         .expect("base64 is always valid HeaderValue");
//     header.set_sensitive(true);
//     header
// }
