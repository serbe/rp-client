use std::net::{SocketAddr, ToSocketAddrs};
use std::str::FromStr;

// use crate::addr::Addr;
use crate::error::{Error, Result};
use crate::scheme::Scheme;
use crate::userinfo::UserInfo;


// https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=e19f3d313ad22db7363658b96fd2b390

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

#[derive(Clone, Debug, Default, PartialEq)]
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

fn from_split_rev(split: Vec<&str>) -> (&str, Option<String>) {
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
            },
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
            None => self.host_port(),
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
        let (raw, scheme) = from_split_rev(raw.splitn(2, "://").collect());
        let scheme = match scheme {
            Some(scheme) => Some(Scheme::from_str(&scheme)?),
            None => None,
        };
        let (raw, userinfo) = from_split_rev(raw.splitn(2, '@').collect());
        let userinfo = if let Some(user_info) = userinfo {
            Some(user_info.into())
        } else {
            None
        };
        let (raw, path) = from_split(raw.splitn(2, '/').collect());

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_path() {
        let s = Url::from_str("http://www.example.org").unwrap();
        let mut u = Url::default();
        u.scheme = Some(Scheme::HTTP);
        u.host = "www.example.org".to_owned();
        assert_eq!(s, u);
    }

    #[test]
    fn with_path() {
        let s = Url::from_str("http://www.example.org/").unwrap();
        let mut u = Url::default();
        u.scheme = Some(Scheme::HTTP);
        u.host = "www.example.org".to_owned();
        u.path = Some("/".to_owned());
        assert_eq!(s, u);
    }

    // #[test]
    // fn path_with_hex_escaping() {
    // 	let mut u = Url::default();
    // 	let s = Url::from_str("http://www.example.org/file%20one%26two").unwrap();
    // 	u.scheme = Some(Scheme::HTTP);
    // 	u.host = "www.example.org";
    // 	// u.path = Some("/file one&two");
    // 	u.path = Some("/file%20one%26two");
    // 	assert_eq!(s, u);
    // }

    #[test]
    fn user() {
        let mut u = Url::default();
        let s = Url::from_str("ftp://webmaster@www.example.org/").unwrap();
        u.scheme = Some(Scheme::Other("ftp".to_owned()));
        u.userinfo = Some(UserInfo {
            username: "webmaster".to_owned(),
            password: String::new(),
        });
        u.host = "www.example.org".to_owned();
        u.path = Some("/".to_owned());
        assert_eq!(s, u);
    }

    // #[test]
    // fn escape_sequence_in_username() {
    // 	let mut u = Url::default();
    // 	let s = Url::from_str("ftp://john%20doe@www.example.org/").unwrap();
    // 	u.scheme = Some("ftp");
    // 	// u.user = Some("john doe");
    // 	u.user = Some("john%20doe");
    // u.userinfo = Some(UserInfo {
    //         username: "webmaster".to_owned(),
    //         password: String::new(),
    //     });
    // 	u.host = "www.example.org";
    // 	u.path = Some("/".to_owned());
    // 	assert_eq!(s, u);
    // }

    #[test]
    fn empty_query() {
        let mut u = Url::default();
        let s = Url::from_str("http://www.example.org/?").unwrap();
        u.scheme = Some(Scheme::HTTP);
        u.host = "www.example.org".to_owned();
        u.path = Some("/".to_owned());
        u.query = Some("".to_owned());
        assert_eq!(s, u);
    }

    #[test]
    fn query_ending_in_question_mark() {
        let mut u = Url::default();
        let s = Url::from_str("http://www.example.org/?foo=bar?").unwrap();
        u.scheme = Some(Scheme::HTTP);
        u.host = "www.example.org".to_owned();
        u.path = Some("/".to_owned());
        u.query = Some("foo=bar?".to_owned());
        assert_eq!(s, u);
    }

    #[test]
    fn query() {
        let mut u = Url::default();
        let s = Url::from_str("http://www.example.org/?q=rust+language").unwrap();
        u.scheme = Some(Scheme::HTTP);
        u.host = "www.example.org".to_owned();
        u.path = Some("/".to_owned());
        u.query = Some("q=rust+language".to_owned());
        assert_eq!(s, u);
    }

    // #[test]
    // fn query_with_hex_escaping() {
    //     let mut u = Url::default();
    //     let s = Url::from_str("http://www.example.org/?q=go%20language").unwrap();
    //     u.scheme = Some(Scheme::HTTP);
    //     u.host = "www.example.org".to_owned();
    //     u.path = Some("/".to_owned());
    //     u.query = Some("q=go%20language");
    //     assert_eq!(s, u);
    // }

    // #[test]
    // fn outside_query() {
    //     let mut u = Url::default();
    //     let s = Url::from_str("http://www.example.org/a%20b?q=c+d").unwrap();
    //     u.scheme = Some(Scheme::HTTP);
    //     u.host = "www.example.org".to_owned();
    //     u.path = Some("/a b");
    //     u.query = Some("q=c+d");
    //     assert_eq!(s, u);
    // }

    #[test]
    fn path_without_leading2() {
        let mut u = Url::default();
        let s = Url::from_str("http://www.example.org/?q=rust+language").unwrap();
        u.scheme = Some(Scheme::HTTP);
        u.host = "www.example.org".to_owned();
        u.path = Some("/".to_owned());
        u.query = Some("q=rust+language".to_owned());
        assert_eq!(s, u);
    }

    // #[test]
    // fn path_without_leading() {
    //     let mut u = Url::default();
    //     let s = Url::from_str("http:%2f%2fwww.example.org/?q=rust+language").unwrap();
    //     u.scheme = Some(Scheme::HTTP);
    //     // Opaque:   "%2f%2fwww.example.org/",
    //     u.query = Some("q=rust+language");
    //     assert_eq!(s, u);
    // }

    #[test]
    fn non() {
        let mut u = Url::default();
        let s = Url::from_str("mailto://webmaster@example.org").unwrap();
        u.scheme = Some(Scheme::Other("mailto".to_owned()));
        u.userinfo = Some(UserInfo {
            username: "webmaster".to_owned(),
            password: String::new(),
        });
        u.host = "example.org".to_owned();
        assert_eq!(s, u);
    }

    #[test]
    fn unescaped() {
        let mut u = Url::default();
        let s = Url::from_str("/foo?query=http://bad").unwrap();
        u.path = Some("/foo".to_owned());
        u.query = Some("query=http://bad".to_owned());
        assert_eq!(s, u);
    }

    #[test]
    fn leading() {
        let mut u = Url::default();
        let s = Url::from_str("//foo").unwrap();
        u.host = "foo".to_owned();
        assert_eq!(s, u);
    }

    #[test]
    fn leading2() {
        let mut u = Url::default();
        let s = Url::from_str("user@foo/path?a=b").unwrap();
        u.userinfo = Some(UserInfo {
            username: "user".to_owned(),
            password: String::new(),
        });
        u.host = "foo".to_owned();
        u.path = Some("/path".to_owned());
        u.query = Some("a=b".to_owned());
        assert_eq!(s, u);
    }

    #[test]
    fn same_codepath() {
        let mut u = Url::default();
        let s = Url::from_str("/threeslashes").unwrap();
        u.path = Some("/threeslashes".to_owned());
        assert_eq!(s, u);
    }

    // #[test]
    // fn relative_path() {
    // 	let mut u = Url::default();
    // 	let s = Url::from_str("a/b/c").unwrap();
    // 	u.path = Some("a/b/c");
    // 	assert_eq!(s, u);
    // }

    // #[test]
    // fn escaped() {
    //     let mut u = Url::default();
    //     let s = Url::from_str("http://%3Fam:pa%3Fsword@google.com").unwrap();
    //     u.scheme = Some(Scheme::HTTP);
    // u.userinfo = Some(UserInfo {
    //     username: "webmaster".to_owned(),
    //     password: String::new(),
    // });
    //     u.user = Some("?am");
    //     u.password = Some("pa?sword");
    //     u.host = "google.com";
    //     assert_eq!(s, u);
    // }

    #[test]
    fn host_subcomponent() {
        let mut u = Url::default();
        let s = Url::from_str("http://192.168.0.1/").unwrap();
        u.scheme = Some(Scheme::HTTP);
        u.host = "192.168.0.1".to_owned();
        u.path = Some("/".to_owned());
        assert_eq!(s, u);
    }

    #[test]
    fn host_and_port_subcomponents() {
        let mut u = Url::default();
        let s = Url::from_str("http://192.168.0.1:8080/").unwrap();
        u.scheme = Some(Scheme::HTTP);
        u.host = "192.168.0.1".to_owned();
        u.port = Some(8080);
        u.path = Some("/".to_owned());
        assert_eq!(s, u);
    }

    #[test]
    fn host_subcomponent2() {
        let mut u = Url::default();
        let s = Url::from_str("http://[fe80::1]/").unwrap();
        u.scheme = Some(Scheme::HTTP);
        u.host = "[fe80::1]".to_owned();
        u.path = Some("/".to_owned());
        assert_eq!(s, u);
    }

    #[test]
    fn host_and_port_subcomponents2() {
        let mut u = Url::default();
        let s = Url::from_str("http://[fe80::1]:8080/").unwrap();
        u.scheme = Some(Scheme::HTTP);
        u.host = "[fe80::1]".to_owned();
        u.port = Some(8080);
        u.path = Some("/".to_owned());
        assert_eq!(s, u);
    }

    // #[test]
    // fn host_subcomponent3() {
    //     let mut u = Url::default();
    //     let s = Url::from_str("http://[fe80::1%25en0]/").unwrap();
    //     u.scheme = Some(Scheme::HTTP);
    //     u.host = "[fe80::1%en0]";
    //     u.path = Some("/".to_owned());
    //     assert_eq!(s, u);
    // }

    // #[test]
    // fn host_and_port_subcomponents3() {
    //     let mut u = Url::default();
    //     let s = Url::from_str("http://[fe80::1%25en0]:8080/").unwrap();
    //     u.scheme = Some(Scheme::HTTP);
    //     u.host = "[fe80::1%en0]";
    //     u.port = Some("8080");
    //     u.path = Some("/".to_owned());
    //     assert_eq!(s, u);
    // }

    // #[test]
    // fn host_subcomponent4() {
    //     let mut u = Url::default();
    //     let s = Url::from_str("http:[fe80::1%25%65%6e%301-._~]/").unwrap();
    //     u.scheme = Some(Scheme::HTTP);
    //     u.host = "[fe80::1%en01-._~]";
    //     u.path = Some("/".to_owned());
    //     assert_eq!(s, u);
    // }

    // #[test]
    // fn host_and_port_subcomponents4() {
    //     let mut u = Url::default();
    //     let s = Url::from_str("http:[fe80::1%25%65%6e%301-._~]:8080/").unwrap();
    //     u.scheme = Some(Scheme::HTTP);
    //     u.host = "[fe80::1%en01-._~]";
    //     u.port = Some("8080");
    //     u.path = Some("/".to_owned());
    //     assert_eq!(s, u);
    // }

    // #[test]
    // fn alternate_escapings_of_path_survive_round_trip() {
    //     let mut u = Url::default();
    //     let s = Url::from_str("http://rest.rsc.io/foo%2fbar/baz%2Fquux?alt=media").unwrap();
    //     u.scheme = Some(Scheme::HTTP);
    //     u.host = "rest.rsc.io";
    //     u.path = Some("/foo/bar/baz/quux");
    //     // Rawu.path = Some("/foo%2fbar/baz%2Fquux");
    //     u.query = Some("alt=media");
    //     assert_eq!(s, u);
    // }

    #[test]
    fn issue_12036() {
        let mut u = Url::default();
        let s = Url::from_str("mysql://a,b,c/bar").unwrap();
        u.scheme = Some(Scheme::Other("mysql".to_owned()));
        u.host = "a,b,c".to_owned();
        u.path = Some("/bar".to_owned());
        assert_eq!(s, u);
    }

    // #[test]
    // fn worst_case_host() {
    //     let mut u = Url::default();
    //     let s = Url::from_str("scheme://!$&'()*+,;=hello!:port/path").unwrap();
    //     u.scheme = Some("scheme");
    //     u.host = "!$&'()*+,;=hello!";
    //     u.port = Some(":port");
    //     u.path = Some("/path");
    //     assert_eq!(s, u);
    // }

    // #[test]
    // fn worst_case_path() {
    //     let mut u = Url::default();
    //     let s = Url::from_str("http://host/!$&'()*+,;=:@[hello]").unwrap();
    //     u.scheme = Some(Scheme::HTTP);
    //     u.host = "host";
    //     u.path = Some("/!$&'()*+,;=:@[hello]");
    //     // Rawu.path = Some("/!$&'()*+,;=:@[hello]");
    //     assert_eq!(s, u);
    // }

    #[test]
    fn example() {
        let mut u = Url::default();
        let s = Url::from_str("http://example.com/oid/[order_id]").unwrap();
        u.scheme = Some(Scheme::HTTP);
        u.host = "example.com".to_owned();
        u.path = Some("/oid/[order_id]".to_owned());
        // Rawu.path = Some("/oid/[order_id]");
        assert_eq!(s, u);
    }

    #[test]
    fn example2() {
        let mut u = Url::default();
        let s = Url::from_str("http://192.168.0.2:8080/foo").unwrap();
        u.scheme = Some(Scheme::HTTP);
        u.host = "192.168.0.2".to_owned();
        u.port = Some(8080);
        u.path = Some("/foo".to_owned());
        assert_eq!(s, u);
    }

    //      let mut u = Url::default();
    //      let s = Url::from_str("http://192.168.0.2:/foo").unwrap();
    //      		u.scheme = Some(Scheme::HTTP);
    //      		u.host = "192.168.0.2:";
    //      		u.path = Some("/foo");
    //      assert_eq!(s, u);
    // }

    //      let mut u = Url::default();
    //      	 Malformed IPv6 but still accepted.
    //      let s = Url::from_str("http://2b01:e34:ef40:7730:8e70:5aff:fefe:edac:8080/foo").unwrap();
    //      		u.scheme = Some(Scheme::HTTP);
    //      		u.host = "2b01:e34:ef40:7730:8e70:5aff:fefe:edac:8080";
    //      		u.path = Some("/foo");
    //      assert_eq!(s, u);
    // }

    //      let mut u = Url::default();
    //      	 Malformed IPv6 but still accepted.
    //      let s = Url::from_str("http://2b01:e34:ef40:7730:8e70:5aff:fefe:edac:/foo").unwrap();
    //      		u.scheme = Some(Scheme::HTTP);
    //      		u.host = "2b01:e34:ef40:7730:8e70:5aff:fefe:edac:";
    //      		u.path = Some("/foo");
    //      assert_eq!(s, u);
    // }

    //      let mut u = Url::default();
    //      let s = Url::from_str("http:[2b01:e34:ef40:7730:8e70:5aff:fefe:edac]:8080/foo").unwrap();
    //      		u.scheme = Some(Scheme::HTTP);
    //      		u.host = "[2b01:e34:ef40:7730:8e70:5aff:fefe:edac]:8080";
    //      		u.path = Some("/foo");
    //      assert_eq!(s, u);
    // }

    //      let mut u = Url::default();
    //      let s = Url::from_str("http:[2b01:e34:ef40:7730:8e70:5aff:fefe:edac]:/foo").unwrap();
    //      		u.scheme = Some(Scheme::HTTP);
    //      		u.host = "[2b01:e34:ef40:7730:8e70:5aff:fefe:edac]:";
    //      		u.path = Some("/foo");
    //      assert_eq!(s, u);
    // }

    #[test]
    fn example3() {
        let mut u = Url::default();
        let s = Url::from_str("http://hello.世界.com/foo").unwrap();
        u.scheme = Some(Scheme::HTTP);
        u.host = "hello.世界.com".to_owned();
        u.path = Some("/foo".to_owned());
        assert_eq!(s, u);
    }

    //      let mut u = Url::default();
    //      let s = Url::from_str("http://hello.%e4%b8%96%e7%95%8c.com/foo").unwrap();
    //      		u.scheme = Some(Scheme::HTTP);
    //      		u.host = "hello.世界.com";
    //      		u.path = Some("/foo");
    //      assert_eq!(s, u);
    //      let s = Url::from_str("http://hello.%E4%B8%96%E7%95%8C.com/foo").unwrap();
    //      }

    //      let mut u = Url::default();
    //      let s = Url::from_str("http://hello.%E4%B8%96%E7%95%8C.com/foo").unwrap();
    //      		u.scheme = Some(Scheme::HTTP);
    //      		u.host = "hello.世界.com";
    //      		u.path = Some("/foo");
    //      assert_eq!(s, u);
    // }

    #[test]
    fn example4() {
        let mut u = Url::default();
        let s = Url::from_str("http://example.com//foo").unwrap();
        u.scheme = Some(Scheme::HTTP);
        u.host = "example.com".to_owned();
        u.path = Some("//foo".to_owned());
        assert_eq!(s, u);
    }

    #[test]
    fn test_that_we_can_reparse_the_host_names_we_accept() {
        let mut u = Url::default();
        let s = Url::from_str("myscheme://authority<\"hi\">/foo").unwrap();
        u.scheme = Some(Scheme::Other("myscheme".to_owned()));
        u.host = "authority<\"hi\">".to_owned();
        u.path = Some("/foo".to_owned());
        assert_eq!(s, u);
    }

    // #[test]
    // fn example5() {
    //     let mut u = Url::default();
    //     let s = Url::from_str("tcp:[2020::2020:20:2020:2020%25Windows%20Loves%20Spaces]:2020").unwrap();
    //     u.scheme = Some("tcp");
    //     u.host = "[2020::2020:20:2020:2020%Windows Loves Spaces]:2020";
    //     assert_eq!(s, u);
    // }
}
