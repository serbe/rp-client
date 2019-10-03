use std::net::SocketAddr;

use http::{HeaderValue, Uri};

use crate::error::{Error, Result};
use crate::url::{Url, IntoUrl};

pub trait IntoProxyScheme {
    fn into_proxy_scheme(self) -> Result<ProxyScheme>;
}

impl<T: IntoUrl> IntoProxyScheme for T {
    fn into_proxy_scheme(self) -> Result<ProxyScheme> {
        ProxyScheme::parse(self.into_url()?)
    }
}

impl IntoProxyScheme for ProxyScheme {
    fn into_proxy_scheme(self) -> Result<ProxyScheme> {
        Ok(self)
    }
}

#[derive(Clone, Debug)]
pub struct Proxy {
    intercept: Intercept,
}

impl Proxy {
    pub fn http<U: IntoProxyScheme>(proxy_scheme: U) -> Result<Proxy> {
        Ok(Proxy::new(Intercept::Http(
            proxy_scheme.into_proxy_scheme()?,
        )))
    }

    pub fn https<U: IntoProxyScheme>(proxy_scheme: U) -> Result<Proxy> {
        Ok(Proxy::new(Intercept::Https(
            proxy_scheme.into_proxy_scheme()?,
        )))
    }

    pub fn all<U: IntoProxyScheme>(proxy_scheme: U) -> Result<Proxy> {
        Ok(Proxy::new(Intercept::All(
            proxy_scheme.into_proxy_scheme()?,
        )))
    }

    pub fn socks<U: IntoProxyScheme>(proxy_scheme: U) -> Result<Proxy> {
        Ok(Proxy::new(Intercept::Socks(
            proxy_scheme.into_proxy_scheme()?,
        )))
    }

    fn new(intercept: Intercept) -> Proxy {
        Proxy { intercept }
    }

    // pub fn basic_auth(mut self, username: &str, password: &str) -> Proxy {
    //     self.intercept.set_basic_auth(username, password);
    //     self
    // }

    // pub(crate) fn maybe_has_http_auth(&self) -> bool {
    //     match self.intercept {
    //         Intercept::All(ProxyScheme::Http { auth: Some(..), .. }) |
    //         Intercept::Http(ProxyScheme::Http { auth: Some(..), .. }) |
    //         // Custom *may* match 'http', so assume so.
    //         Intercept::Custom(_) => true,
    //         _ => false,
    //     }
    // }

    // pub(crate) fn http_basic_auth<D: Dst>(&self, uri: &D) -> Option<HeaderValue> {
    //     match self.intercept {
    //         Intercept::All(ProxyScheme::Http { ref auth, .. })
    //         | Intercept::Http(ProxyScheme::Http { ref auth, .. }) => auth.clone(),
    //         Intercept::Custom(ref custom) => custom.call(uri).and_then(|scheme| match scheme {
    //             ProxyScheme::Http { auth, .. } => auth,
    //             #[cfg(feature = "socks")]
    //             _ => None,
    //         }),
    //         _ => None,
    //     }
    // }

    // pub(crate) fn intercept<D: Dst>(&self, uri: &D) -> Option<ProxyScheme> {
    //     match self.intercept {
    //         Intercept::All(ref u) => Some(u.clone()),
    //         Intercept::Http(ref u) => {
    //             if uri.scheme() == "http" {
    //                 Some(u.clone())
    //             } else {
    //                 None
    //             }
    //         }
    //         Intercept::Https(ref u) => {
    //             if uri.scheme() == "https" {
    //                 Some(u.clone())
    //             } else {
    //                 None
    //             }
    //         }
    //         Intercept::Custom(ref custom) => custom.call(uri),
    //     }
    // }

    // pub(crate) fn is_match<D: Dst>(&self, uri: &D) -> bool {
    //     match self.intercept {
    //         Intercept::All(_) => true,
    //         Intercept::Http(_) => uri.scheme() == "http",
    //         Intercept::Https(_) => uri.scheme() == "https",
    //         Intercept::Custom(ref custom) => custom.call(uri).is_some(),
    //     }
    // }
}

#[derive(Clone, Debug)]
pub enum ProxyScheme {
    Http {
        auth: Option<HeaderValue>,
        uri: Url,
    },
    Socks5 {
        addr: SocketAddr,
        auth: Option<(String, String)>,
        remote_dns: bool,
    },
}

impl ProxyScheme {
    fn http<T: IntoUrl>(url: T) -> Result<Self> {
        Ok(ProxyScheme::Http {
            auth: None,
            uri: url.into_url()?,
        })
    }

    fn socks5(addr: SocketAddr) -> Result<Self> {
        Ok(ProxyScheme::Socks5 {
            addr,
            auth: None,
            remote_dns: false,
        })
    }

    fn socks5h(addr: SocketAddr) -> Result<Self> {
        Ok(ProxyScheme::Socks5 {
            addr,
            auth: None,
            remote_dns: true,
        })
    }

    // fn with_basic_auth<T: Into<String>, U: Into<String>>(
    //     mut self,
    //     username: T,
    //     password: U,
    // ) -> Self {
    //     self.set_basic_auth(username, password);
    //     self
    // }

    // fn set_basic_auth<T: Into<String>, U: Into<String>>(&mut self, username: T, password: U) {
    //     match *self {
    //         ProxyScheme::Http { ref mut auth, .. } => {
    //             let header = encode_basic_auth(&username.into(), &password.into());
    //             *auth = Some(header);
    //         }
    //         #[cfg(feature = "socks")]
    //         ProxyScheme::Socks5 { ref mut auth, .. } => {
    //             *auth = Some((username.into(), password.into()));
    //         }
    //     }
    // }

    fn parse(url: Url) -> Result<Self> {
        let to_addr = || {
            let host_and_port = url.with_default_port(|url| match url.scheme() {
                "socks5" | "socks5h" => Ok(1080),
                _ => Err(()),
            })?;
            let mut addr = try_!(host_and_port.to_socket_addrs());
            addr.next().ok_or_else(::error::unknown_proxy_scheme)
        };

        let mut scheme = match url.scheme()?.as_str() {
            "http" | "https" => Self::http(url.clone())?,
            "socks5" => Self::socks5(to_addr()?)?,
            "socks5h" => Self::socks5h(to_addr()?)?,
            scheme => return Err(Error::UnknownProxyScheme(scheme)),
        };

        if let Some(pwd) = url.password() {
            let decoded_username = percent_decode(url.username().as_bytes()).decode_utf8_lossy();
            let decoded_password = percent_decode(pwd.as_bytes()).decode_utf8_lossy();
            scheme = scheme.with_basic_auth(decoded_username, decoded_password);
        }

        Ok(scheme)
    }
}

#[derive(Clone, Debug)]
enum Intercept {
    All(ProxyScheme),
    Http(ProxyScheme),
    Https(ProxyScheme),
    Socks(ProxyScheme),
}

impl Intercept {
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
}

// pub(crate) fn encode_basic_auth(username: &str, password: &str) -> HeaderValue {
//     let val = format!("{}:{}", username, password);
//     let mut header = format!("Basic {}", base64::encode(&val))
//         .parse::<HeaderValue>()
//         .expect("base64 is always valid HeaderValue");
//     header.set_sensitive(true);
//     header
// }
