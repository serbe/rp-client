use std::net::{SocketAddr, ToSocketAddrs};
// use std::str::FromStr;

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

#[derive(Clone, Debug)]
pub struct Url {
    pub scheme: Scheme,
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

    pub fn from_str(s: &'static str) -> Result<Self> {
        let raw = s;

        let (raw, fragment) = from_split(raw.rsplitn(2, '#').collect());
        let (raw, query) = from_split(raw.rsplitn(2, '?').collect());
        let (raw, scheme) = from_rsplit(raw.rsplitn(2, "://").collect());
        let scheme = Scheme::from_opt(scheme)?;
        let (raw, userinfo) = from_rsplit(raw.rsplitn(2, "@").collect());
        let userinfo = if let Some(user_info) = userinfo {
            Some(UserInfo::from_str(&user_info))
        } else {
            None
        };
        let (raw, path) = from_split(raw.rsplitn(2, '/').collect());

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

        let host = host.to_owned();
        let port = if let Some(p) = port {
            Some(p.parse::<u16>().map_err(|_| Error::ParsePort(p))?)
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

    //     fn into_uri(uri: Uri) -> Result<Self> {
    //         if uri.host().is_some() {
    //             Ok(Url::new(uri))
    //         } else {
    //             Err(Error::NoHost(uri))
    //         }
    //     }

    pub fn default_port(&self) -> u16 {
        match self.port {
            Some(port) => port,
            None => self.scheme.default_port(),
        }
    }

    pub fn host_port(&self) -> String {
        format!("{}:{}", self.host, self.default_port())
    }

    pub fn scheme(&self) -> String {
        self.scheme.to_string()
    }

    pub fn origin(&self) -> String {
        format!("{}://{}", self.scheme.to_string(), self.host_port())
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
