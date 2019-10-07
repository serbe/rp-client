use std::net::{SocketAddr, ToSocketAddrs};
use std::str::FromStr;

// use crate::addr::Addr;
use crate::error::{Error, Result};
use crate::scheme::Scheme;
use crate::userinfo::UserInfo;

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

#[derive(Clone, Debug, Default)]
pub struct Url {
    pub scheme: Option<Scheme>,
    pub userinfo: Option<UserInfo>,
    pub host: String,
    pub port: Option<u16>,
    pub path: Option<String>,
    pub query: Option<String>,
    pub fragment: Option<String>,
    // pub addr: Addr,
}

fn from_split(split: Vec<&str>) -> (&str, Option<String>) {
    if split.len() == 2 {
        (split[0], Some(split[1].to_owned()))
    } else {
        (split[0], None)
    }
}

fn from_rsplit(split: Vec<&str>) -> (&str, Option<String>) {
    if split.len() == 2 {
        (split[1], Some(split[0].to_owned()))
    } else {
        (split[0], None)
    }
}

impl Url {
    // pub fn new(uri: Uri) -> Self {
    //     Url { uri }
    // }

    //     fn into_uri(uri: Uri) -> Result<Self> {
    //         if uri.host().is_some() {
    //             Ok(Url::new(uri))
    //         } else {
    //             Err(Error::NoHost(uri))
    //         }
    //     }

    pub fn default_port(&self) -> Option<u16> {
        match self.port {
            Some(port) => Some(port),
            None => match &self.scheme {
                Some(scheme) => scheme.default_port(),
                None => None,
            }
        }
    }

    pub fn host_port(&self) -> String {
        match self.default_port() {
            Some(port) => format!("{}:{}", self.host, port),
            None => self.host.to_owned(),
        }
    }

    // pub fn scheme(&self) -> String {
    //     self.scheme.to_string()
    // }

    pub fn origin(&self) -> String {
        match &self.scheme {
            Some(scheme) => format!("{}://{}", scheme, self.host_port()),
            None => self.host_port()
        }
    }

    fn socket_addrs(&self) -> Result<Vec<SocketAddr>> {
        Ok(self.host_port().to_socket_addrs()?.collect())
    }

    pub fn socket_addr(&self) -> Result<SocketAddr> {
        Ok(self.socket_addrs()?[0])
    }

    //     pub fn uri(&self) -> Uri {
    //         self.uri.clone()
    //     }

    // pub fn addr(&self) -> Addr {
    //     Addr::from_str(&self.host_port())
    // }
}

impl FromStr for Url {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let raw = s;

        let (raw, fragment) = from_split(raw.rsplitn(2, '#').collect());
        let (raw, query) = from_split(raw.rsplitn(2, '?').collect());
        let (raw, scheme) = from_rsplit(raw.rsplitn(2, "://").collect());
        let scheme = match scheme {
            Some(scheme) => Some(Scheme::from_str(&scheme)?),
            None => None,
        };
        let (raw, userinfo) = from_rsplit(raw.rsplitn(2, '@').collect());
        let userinfo = if let Some(user_info) = userinfo {
            Some(user_info.into())
        } else {
            None
        };
        let (raw, path) = from_split(raw.rsplitn(2, '/').collect());

        let (host, port) = if let Some(pos) = raw.rfind(':') {
            if let Some(start) = raw.find('[') {
                if let Some(end) = raw.find(']') {
                    if start == 0 && pos == end + 1 {
                        (
                            raw.get(..pos)
                                .ok_or_else(|| Error::ParseHost(raw.to_owned()))?,
                            raw.get(pos + 1..),
                        )
                    } else if start == 0 && end == raw.len() - 1 {
                        (raw, None)
                    } else {
                        return Err(Error::ParseIPv6(raw.to_owned()));
                    }
                } else {
                    return Err(Error::ParseIPv6(raw.to_owned()));
                }
            } else {
                (
                    raw.get(..pos)
                        .ok_or_else(|| Error::ParseHost(raw.to_owned()))?,
                    raw.get(pos + 1..),
                )
            }
        } else {
            (raw, None)
        };

        let host = host.to_owned();
        let port = if let Some(p) = port {
            Some(
                p.parse::<u16>()
                    .map_err(|_| Error::ParsePort(p.to_owned()))?,
            )
        } else {
            None
        };

        Ok(Url {
            scheme,
            userinfo,
            host,
            port,
            path,
            query,
            fragment,
        })
    }
}

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
