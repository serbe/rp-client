use std::str::FromStr;
use std::net::SocketAddr;

// use http::{HeaderValue, Uri};

use crate::error::{Error, Result};
use crate::url::{Url};
use crate::userinfo::UserInfo;
use crate::scheme::Scheme;

pub trait IntoProxyScheme {
    fn into_proxy_scheme(self) -> Result<ProxyScheme>;
}

// impl<T: IntoUrl> IntoProxyScheme for T {
//     fn into_proxy_scheme(self) -> Result<ProxyScheme> {
//         ProxyScheme::parse(self.into_url()?)
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
        Ok(Proxy::Http(
            proxy_scheme.into_proxy_scheme()?,
        ))
    }

    pub fn https<U: IntoProxyScheme>(proxy_scheme: U) -> Result<Proxy> {
        Ok(Proxy::Https(
            proxy_scheme.into_proxy_scheme()?,
        ))
    }

    pub fn all<U: IntoProxyScheme>(proxy_scheme: U) -> Result<Proxy> {
        Ok(Proxy::All(
            proxy_scheme.into_proxy_scheme()?,
        ))
    }

    pub fn socks<U: IntoProxyScheme>(proxy_scheme: U) -> Result<Proxy> {
        Ok(Proxy::Socks(
            proxy_scheme.into_proxy_scheme()?,
        ))
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
        url: Url,
    },
    Socks5 {
        addr: SocketAddr,
        auth: Option<UserInfo>,
        remote_dns: bool,
    },
}

impl ProxyScheme {
    fn http(url: Url) -> Result<Self> {
        Ok(ProxyScheme::Http {
            url,
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
}

impl FromStr for ProxyScheme {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let url = s.parse::<Url>()?;
        match url.scheme {
            Some(Scheme::HTTP) | Some(Scheme::HTTPS) => Self::http(url),
            Some(Scheme::SOCKS5) => Self::socks5(url.socket_addr()?),
            Some(Scheme::SOCKS5H) => Self::socks5h(url.socket_addr()?),
            Some(Scheme::Other(scheme)) => Err(Error::UnsupportedScheme(scheme)),
            None => Err(Error::EmptyScheme)
        }

        // if let Some(pwd) = url.password() {
        //     let decoded_username = percent_decode(url.username().as_bytes()).decode_utf8_lossy();
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

// pub(crate) trait Addr {
//     fn scheme(&self) -> &str;
//     fn host(&self) -> &str;
//     fn port(&self) -> Option<u16>;
// }

// impl Addr for Url {
//     fn scheme(&self) -> &str {
//         &Url::scheme(self).unwrap_or("".to_string())
//     }

//     fn host(&self) -> &str {
//         Url::host(self)
//     }

//     fn port(&self) -> Option<u16> {
//         Url::port(self)
//     }
// }

// impl Addr for Uri {
//     fn scheme(&self) -> &str {
//         self.scheme_part()
//             .expect("Uri should have a scheme")
//             .as_str()
//     }

//     fn host(&self) -> &str {
//         Uri::host(self)
//             .expect("<Uri as Dst>::host should have a str")
//     }

//     fn port(&self) -> Option<u16> {
//         self.port_part().map(|p| p.as_u16())
//     }
// }
