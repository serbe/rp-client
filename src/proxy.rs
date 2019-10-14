use std::net::SocketAddr;
use std::str::FromStr;

// use http::{HeaderValue, Uri};

use crate::error::{Error, Result};
// use crate::scheme::Scheme;
use crate::uri::Uri;
use crate::userinfo::UserInfo;

pub trait IntoProxyScheme {
    fn into_proxy_scheme(self) -> Result<ProxyScheme>;
}

// impl<T: IntoUri> IntoProxyScheme for T {
//     fn into_proxy_scheme(self) -> Result<ProxyScheme> {
//         ProxyScheme::parse(self.into_uri()?)
//     }
// }

impl IntoProxyScheme for ProxyScheme {
    fn into_proxy_scheme(self) -> Result<ProxyScheme> {
        Ok(self)
    }
}

#[derive(Clone, Debug)]
pub enum Proxy {
    All(ProxyScheme),
    Http(ProxyScheme),
    Https(ProxyScheme),
    Socks(ProxyScheme),
}

impl Proxy {
    pub fn http<U: IntoProxyScheme>(proxy_scheme: U) -> Result<Proxy> {
        Ok(Proxy::Http(proxy_scheme.into_proxy_scheme()?))
    }

    pub fn https<U: IntoProxyScheme>(proxy_scheme: U) -> Result<Proxy> {
        Ok(Proxy::Https(proxy_scheme.into_proxy_scheme()?))
    }

    pub fn all<U: IntoProxyScheme>(proxy_scheme: U) -> Result<Proxy> {
        Ok(Proxy::All(proxy_scheme.into_proxy_scheme()?))
    }

    pub fn socks<U: IntoProxyScheme>(proxy_scheme: U) -> Result<Proxy> {
        Ok(Proxy::Socks(proxy_scheme.into_proxy_scheme()?))
    }

    // fn new(intercept: Intercept) -> Proxy {
    //     Proxy { intercept }
    // }

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
        uri: Uri,
    },
    Socks5 {
        addr: SocketAddr,
        auth: Option<UserInfo>,
        remote_dns: bool,
    },
}

impl ProxyScheme {
    fn http(uri: Uri) -> Result<Self> {
        Ok(ProxyScheme::Http { uri })
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
}

impl FromStr for ProxyScheme {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let uri = s.parse::<Uri>()?;
        match uri.scheme() {
            "http" | "https" => Self::http(uri.clone()),
            "socks5" => Self::socks5(uri.socket_addr()?),
            "socks5h" => Self::socks5h(uri.socket_addr()?),
            _ => Err(Error::UnsupportedScheme(s.to_string())),
        }

        // if let Some(pwd) = uri.password() {
        //     let decoded_username = percent_decode(uri.username().as_bytes()).decode_utf8_lossy();
        //     let decoded_password = percent_decode(pwd.as_bytes()).decode_utf8_lossy();
        //     scheme = scheme.with_basic_auth(decoded_username, decoded_password);
        // }
    }
}

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
