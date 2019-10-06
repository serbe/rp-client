use std::net::{SocketAddr, ToSocketAddrs};
use std::str::FromStr;

use crate::error::{Error, Result};
use crate::scheme::Scheme;
use crate::addr::Addr;

// pub trait IntoUrl {
//     fn into_url(self) -> Result<Url>;
// }

// impl IntoUrl for Url {
//     fn into_url(self) -> Result<Url> {
//         if self.uri.host().is_some() {
//             Ok(self)
//         } else {
//             Err(Error::NoHost(self.uri))
//         }
//     }
// }

// impl<'a> IntoUrl for &'a str {
//     fn into_url(self) -> Result<Url> {
//         self.parse::<Url>()?.into_url()
//     }
// }

// impl<'a> IntoUrl for &'a String {
//     fn into_url(self) -> Result<Url> {
//         self.parse::<Url>()?.into_url()
//     }
// }

// impl<'a> IntoUrl for Uri {
//     fn into_url(self) -> Result<Url> {
//         Url::new(self).into_url()
//     }
// }

#[derive(Clone, Debug)]
pub struct Url {
    scheme: Scheme,
    addr: Addr,
    host: String,
    port: u16,
    path: Option<String>,
    query: Option<String>,
    fragment: Option<String>,
}

impl Url {
    // pub fn new(uri: Uri) -> Self {
    //     Url { uri }
    // }

    pub fn from_str(s: &str) -> Result<Self> {
        let raw = s;

        let (raw, fragment) = if let Some(pos) = raw.find('#') {
                let split = raw.splitn(2, '#').collect();
                (split[0], Some(split[1].to_owned()))
        } else {
            (raw, None)
        };

        let (raw, query) = if let Some(pos) = raw.find('?') {
            (
                raw.get(..pos).ok_or_else(|| Error::ParseQuery(raw))?,
                raw.get(pos + 1..),
            )
        } else {
            (raw, None)
        };

        let (raw, scheme) = if let Some(pos) = raw.find("://") {
            (
                raw.get(pos + 3..).ok_or_else(|| Error::ParseScheme(raw))?,
                raw.get(..pos),
            )
        } else {
            (raw, None)
        };

        let (raw, user, password) = if let Some(pos) = raw.find('@') {
            let new_raw = raw
                .get(pos + 1..)
                .ok_or_else(|| Error::ParseUserInfo(raw))?;
            let userinfo = raw.get(..pos);
            match userinfo {
                Some(user) => {
                    if let Some(pos) = user.find(':') {
                        (new_raw, user.get(..pos), user.get(pos + 1..))
                    } else {
                        (new_raw, Some(user), None)
                    }
                }
                None => (new_raw, None, None),
            }
        } else {
            (raw, None, None)
        };

        let (raw, path) = if let Some(pos) = raw.find('/') {
            (
                raw.get(..pos).ok_or_else(|| Error::ParsePath(raw))?,
                raw.get(pos..),
            )
        } else {
            (raw, None)
        };

        let (host, port) = if let Some(pos) = raw.rfind(':') {
            if let Some(start) = raw.find('[') {
                if let Some(end) = raw.find(']') {
                    if start == 0 && pos == end + 1 {
                        (
                            raw.get(..pos).ok_or_else(|| Error::ParseHost(raw))?,
                            raw.get(pos + 1..),
                        )
                    } else if start == 0 && end == raw.len() - 1 {
                        (raw, None)
                    } else {
                        Err(Error::ParseIPv6(raw))?
                    }
                } else {
                    Err(Error::ParseIPv6(raw))?
                }
            } else {
                (
                    raw.get(..pos).ok_or_else(|| Error::ParseHost(raw))?,
                    raw.get(pos + 1..),
                )
            }
        } else {
            (raw, None)
        };

        if let Some(port) = port {
            let _ = port.parse::<u32>().map_err(|_| Error::ParsePort(raw))?;
        }

        Ok(Url {
            scheme,
            user,
            password,
            host,
            port,
            path,
            query,
            fragment,
        })
    }
}

//     fn into_uri(uri: Uri) -> Result<Self> {
//         if uri.host().is_some() {
//             Ok(Url::new(uri))
//         } else {
//             Err(Error::NoHost(uri))
//         }
//     }

//     fn default_port(&self) -> Option<u16> {
//         match (self.uri.scheme_str(), self.uri.port_u16()) {
//             (_, Some(port)) => Some(port),
//             (Some(scheme), None) => match (scheme.to_lowercase()).as_str() {
//                 "http" => Some(80),
//                 "https" => Some(443),
//                 "socks5" => Some(1080),
//                 "socks5h" => Some(1080),
//                 _ => None,
//             },
//             (_, _) => Some(80),
//         }
//     }

//     fn host_port(&self) -> Result<String> {
//         match (self.uri.host(), self.default_port()) {
//             (Some(host), Some(port)) => Ok(format!("{}:{}", host, port)),
//             (None, Some(_)) => Err(Error::NoHost(self.uri.clone())),
//             (Some(_), None) => Err(Error::NoPort(self.uri.clone())),
//             (None, None) => Err(Error::NoHostPort(self.uri.clone())),
//         }
//     }

//     pub fn scheme(&self) -> Result<String> {
//         match self.uri.scheme_str() {
//             Some(scheme) => Ok(scheme.to_lowercase()),
//             None => Err(Error::NoScheme(self.uri.clone())),
//         }
//     }

//     fn origin(&self) -> Result<String> {
//         Ok(format!("{}://{}", self.scheme()?, self.host_port()?))
//     }

//     fn socket_addrs(&self) -> Result<Vec<SocketAddr>> {
//         Ok(self.host_port()?.to_socket_addrs()?.collect())
//     }

//     pub fn socket_addr(&self) -> Result<SocketAddr> {
//         Ok(self.socket_addrs()?[0])
//     }

//     pub fn uri(&self) -> Uri {
//         self.uri.clone()
//     }
// }

// impl<'a> TryFrom<&'a str> for Url {
//     type Error = Error;

//     fn try_from(uri: &'a str) -> Result<Url> {
//         Url::into_uri(uri.parse::<Uri>()?)
//     }
// }

// impl TryFrom<String> for Url {
//     type Error = Error;

//     fn try_from(uri: String) -> Result<Url> {
//         Url::into_uri(uri.parse::<Uri>()?)
//     }
// }

// impl FromStr for Url {
//     type Err = Error;

//     fn from_str(s: &str) -> Result<Self> {
//         Url::into_uri(s.parse::<Uri>()?)
//     }
// }

// pub(crate) fn expect_uri(uri: &Uri) -> Uri {
//     uri.as_str()
//         .parse()
//         .expect("a parsed Url should always be a valid Uri")
// }

// pub(crate) fn try_uri(url: &Url) -> Option<::hyper::Uri> {
//     url.as_str().parse().ok()
// }
