use std::fmt;
use std::net::{SocketAddr, ToSocketAddrs, Ipv4Addr, Ipv6Addr};
use std::ops::{Index, Range};
use std::str;
use std::string::ToString;
use std::str::FromStr;

use crate::error::{Error, Result};
// use crate::proxy::ProxyScheme;

#[derive(Clone, Debug, PartialEq)]
pub enum Addr {
    Ipv4(Ipv4Addr),
    Ipv6(Ipv6Addr),
    Domain(String),
}

impl FromStr for Addr {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        if s.starts_with('[') {
            if !s.ends_with(']') {
                return Err(Error::ParseIPv6);
            }
            return s[1..s.len() - 1].parse::<Ipv6Addr>().map(Addr::Ipv6).map_err(Error::StdParseAddr);
        }
        if let Ok(ipv6) = s.parse::<Ipv6Addr>() {
            Ok(Addr::Ipv6(ipv6))
        } else if let Ok(ipv4) = s.parse::<Ipv4Addr>() {
            Ok(Addr::Ipv4(ipv4))
        } else if valid_domain(s) {
            Ok(Addr::Domain(s.to_string()))
        } else {
            Err(Error::ParseAddr)
        }
    }
}

fn valid_domain(s: &str) -> bool {
    let i_chars = vec!['\0', '\t', '\n', '\r', ' ', '#', '%', '/', ':', '?', '@', '[', '\\', ']'];
    i_chars.iter().all(|&c| !s.contains(c))
                
}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct RangeC {
    pub start: usize,
    pub end: usize,
}

impl RangeC {
    pub const fn new(start: usize, end: usize) -> RangeC {
        RangeC { start, end }
    }
}

impl From<RangeC> for Range<usize> {
    fn from(range: RangeC) -> Range<usize> {
        Range {
            start: range.start,
            end: range.end,
        }
    }
}

impl Index<RangeC> for String {
    type Output = str;

    #[inline]
    fn index(&self, index: RangeC) -> &str {
        &self[..][Range::from(index)]
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Uri {
    inner: String,
    scheme: RangeC,
    authority: Authority,
    addr: Addr,
    path: Option<RangeC>,
    query: Option<RangeC>,
    fragment: Option<RangeC>,
}

impl Uri {
    pub fn as_str(&self) -> &str {
        &self.inner
    }

    pub fn scheme(&self) -> &str {
        &self.inner[self.scheme]
    }

    pub fn user_info(&self) -> Option<&str> {
        self.authority.user_info()
    }

    pub fn host(&self) -> &str {
        self.authority.host()
    }

    pub fn host_header(&self) -> String {
        match self.default_port() {
            p if p == 80 || p == 8080 => self.host().to_string(),
            p => format!("{}:{}", self.host(), p),
        }
    }

    pub fn port(&self) -> Option<u16> {
        self.authority.port()
    }

    pub fn default_port(&self) -> u16 {
        let default_port = match self.scheme() {
            "https" => 443,
            "http" => 80,
            "socks5" | "socks5h" => 1080,
            _ => 80,
        };

        match self.authority.port() {
            Some(port) => port,
            None => default_port,
        }
    }

    pub fn path(&self) -> Option<&str> {
        self.path.map(|r| &self.inner[r])
    }

    pub fn query(&self) -> Option<&str> {
        self.query.map(|r| &self.inner[r])
    }

    pub fn fragment(&self) -> Option<&str> {
        self.fragment.map(|r| &self.inner[r])
    }

    pub fn resource(&self) -> &str {
        let mut result = "/";

        for v in &[self.path, self.query, self.fragment] {
            if let Some(r) = v {
                result = &self.inner[r.start..];
                break;
            }
        }

        result
    }

    // pub fn domain(&self) -> Result<String> {
    //     match &self.scheme() {
    //         Some("http") | Some("https") | Some("socks5") | Some("socks5h") => Ok(self.origin()),
    //         _ => Err(Error::UnsupportedProxyScheme),
    //     }
    // }

    pub fn origin(&self) -> String {
        format!("{}://{}", self.scheme(), self.host_port())
    }

    // remove unwrap
    pub fn host_port(&self) -> String {
        format!("{}:{}", self.host(), self.default_port())
    }

    fn socket_addrs(&self) -> Result<Vec<SocketAddr>> {
        Ok(self.host_port().to_socket_addrs()?.collect())
    }

    pub fn socket_addr(&self) -> Result<SocketAddr> {
        Ok(self.socket_addrs()?[0])
    }
}

impl fmt::Display for Uri {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut uri = self.inner.to_string();
        let auth = self.authority.to_string();
        let start = self.scheme.end + 3;
        uri.replace_range(start..(start + auth.len()), &auth);
        write!(f, "{}", uri)
    }
}

impl FromStr for Uri {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut s = s.to_string();
        remove_spaces(&mut s);

        let (uri_part, fragment) = get_chunks(&s, Some(RangeC::new(0, s.len())), "#", true, true);
        let (uri_part, query) = get_chunks(&s, uri_part, "?", true, true);
        let (scheme, maybe_part) = get_chunks(&s, uri_part, ":", true, false);
        let (scheme, mut uri_part) = if let Some(scheme) = scheme {
            let range = Range::from(scheme);
            if s[range.clone()].chars().all(|c| c.is_alphanumeric()) {
                s.replace_range(range.clone(), &s[range].to_lowercase());
                (scheme, maybe_part)
            } else {
                return Err(Error::EmptyScheme)
            }
        } else {
            return Err(Error::EmptyScheme)
        };

        let authority = if let Some(u) = &uri_part {
            let (auth, part) = if s[*u].contains("//") {
                get_chunks(&s, Some(RangeC::new(u.start + 2, u.end)), "/", false, false)
            } else {
                get_chunks(&s, uri_part, "/", false, false)
            };
            if let Some(a) = auth {
                uri_part = part;
                s[a].parse::<Authority>()?
            } else {
                return Err(Error::EmptyAuthority)
            }
        } else {
            return Err(Error::EmptyAuthority)
        };

        let addr = authority.host().parse::<Addr>()?;

        let (path, _uri_part) = get_chunks(&s, uri_part, "?", false, false);

        // if authority.is_some() || &s[scheme] == "file" {
        //     path = path.map(|p| RangeC::new(p.start - 1, p.end));
        // }

        Ok(Uri {
            inner: s,
            scheme,
            authority,
            addr,
            path,
            query,
            fragment,
        })
    }
}

// impl FromStr for Uri {
//     type Err = Error;

//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         let mut s = s.to_string();
//         remove_spaces(&mut s);

//         let (scheme, mut uri_part) = get_chunks(&s, Some(RangeC::new(0, s.len())), ":");
//         let scheme = scheme.ok_or(ParseErr::UriErr)?;

//         let mut authority = None;

//         if let Some(u) = &uri_part {
//             if s[*u].contains("//") {
//                 let (auth, part) = get_chunks(&s, Some(RangeC::new(u.start + 2, u.end)), "/");

//                 authority = if let Some(a) = auth {
//                     Some(s[a].parse()?)
//                 } else {
//                     None
//                 };

//                 uri_part = part;
//             }
//         }

//         let (mut path, uri_part) = get_chunks(&s, uri_part, "?");

//         if authority.is_some() || &s[scheme] == "file" {
//             path = path.map(|p| RangeC::new(p.start - 1, p.end));
//         }

//         let (query, fragment) = get_chunks(&s, uri_part, "#");

//         Ok(Uri {
//             inner: s,
//             scheme,
//             authority,
//             path,
//             query,
//             fragment,
//         })
//     }
// }

#[derive(Clone, Debug, PartialEq)]
pub struct Authority {
    inner: String,
    username: Option<RangeC>,
    password: Option<RangeC>,
    host: RangeC,
    port: Option<RangeC>,
}

impl Authority {
    pub fn username(&self) -> Option<&str> {
        self.username.map(|r| &self.inner[r])
    }

    pub fn password(&self) -> Option<&str> {
        self.password.map(|r| &self.inner[r])
    }

    pub fn user_info(&self) -> Option<&str> {
        match (&self.username, &self.password) {
            (Some(u), Some(p)) => Some(&self.inner[u.start..p.end]),
            (Some(u), None) => Some(&self.inner[*u]),
            _ => None,
        }
    }

    pub fn host(&self) -> &str {
        &self.inner[self.host]
    }

    pub fn port(&self) -> Option<u16> {
        match &self.port {
            Some(p) => Some(self.inner[*p].parse().unwrap()),
            None => None,
        }
    }
}

impl FromStr for Authority {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let inner = s.to_string();

        let mut username = None;
        let mut password = None;

        let uri_part = if s.contains('@') {
            let (info, part) = get_chunks(&s, Some(RangeC::new(0, s.len())), "@", true, false);
            let (name, pass) = get_chunks(&s, info, ":", true, false);

            username = name;
            password = pass;

            part
        } else {
            Some(RangeC::new(0, s.len()))
        };

        let split_by = if s.contains(']') && s.contains('[') {
            "]:"
        } else {
            ":"
        };
        let (host, port) = get_chunks(&s, uri_part, split_by, true, false);
        let host = host.ok_or(Error::ParseHost)?;

        if let Some(p) = port {
            if inner[p].parse::<u16>().is_err() {
                return Err(Error::ParsePort);
            }
        }

        Ok(Authority {
            inner,
            username,
            password,
            host,
            port,
        })
    }
}

impl fmt::Display for Authority {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let auth = if let Some(pass) = self.password {
            let range = Range::from(pass);

            let hidden_pass = "*".repeat(range.len());
            let mut auth = self.inner.to_string();
            auth.replace_range(range, &hidden_pass);

            auth
        } else {
            self.inner.to_string()
        };

        write!(f, "{}", auth)
    }
}

//Removes whitespace from `text`
fn remove_spaces(text: &mut String) {
    text.retain(|c| !c.is_whitespace());
}

fn get_chunks<'a>(
    s: &'a str,
    range: Option<RangeC>,
    separator: &'a str,
    cut: bool,
    allow_empty: bool,
) -> (Option<RangeC>, Option<RangeC>) {
    if let Some(r) = range {
        let range = Range::from(r);
        let c = if cut { 0 } else { 1 };

        match s[range.clone()].find(separator) {
            Some(i) => {
                let mid = r.start + i + separator.len();
                let before = Some(RangeC::new(r.start, mid - 1)).filter(|r| r.start != r.end);
                let after = if allow_empty {
                    Some(RangeC::new(mid - c, r.end))
                } else {
                    Some(RangeC::new(mid - c, r.end)).filter(|r| r.start != r.end)
                };
                (before, after)
            }
            None => {
                if !s[range].is_empty() {
                    (Some(r), None)
                } else {
                    (None, None)
                }
            }
        }
    } else {
        (None, None)
    }
}



#[cfg(test)]
mod tests {
    use std::net::{Ipv4Addr, Ipv6Addr};

    use crate::uri::{Addr, Uri};

    #[test]
    fn addr_ipv4() {
        assert_eq!(
            "127.0.0.1".parse::<Addr>().unwrap(),
            Addr::Ipv4(Ipv4Addr::new(127, 0, 0, 1))
        );
    }

    #[test]
    fn addr_ipv6() {
        assert_eq!(
            "[2001:0db8:11a3:09d7:1f34:8a2e:07a0:765d]".parse::<Addr>().unwrap(),
            Addr::Ipv6(Ipv6Addr::new(0x2001, 0xdb8, 0x11a3, 0x9d7, 0x1f34, 0x8a2e, 0x7a0, 0x765d))
        );
    }

    #[test]
    fn addr_domain() {
        assert_eq!(
            "test.com".parse::<Addr>().unwrap(),
            Addr::Domain("test.com".to_string())
        );
    }
    #[test]
    fn case_scheme() {
        let uri = "hTtP://www.example.org".parse::<Uri>().unwrap();
        assert_eq!(uri.scheme(), "http");
    }

    #[test]
    fn case_uri() {
        let uri = "hTtP://www.example.org".parse::<Uri>().unwrap();
        assert_eq!(uri.as_str(), "http://www.example.org");
    }

    #[test]
    fn no_path() {
        let uri = "http://www.example.org".parse::<Uri>().unwrap();
        assert_eq!(uri.scheme(), "http");
        assert_eq!(uri.host(), "www.example.org");
    }

    #[test]
    fn with_path() {
        let uri = "http://www.example.org/".parse::<Uri>().unwrap();
        assert_eq!(uri.scheme(), "http");
        assert_eq!(uri.host(), "www.example.org");
        assert_eq!(uri.path(), Some("/"));
    }

    #[test]
    fn path_with_hex_escaping() {
        let uri = "http://www.example.org/file%20one%26two"
            .parse::<Uri>()
            .unwrap();
        assert_eq!(uri.scheme(), "http");
        assert_eq!(uri.host(), "www.example.org");
        assert_eq!(uri.path(), Some("/file%20one%26two"));
        // assert_eq!(u.decode_path(), Some("/file one&two".to_owned()));
    }

    #[test]
    fn user() {
        let uri = "ftp://webmaster@www.example.org/".parse::<Uri>().unwrap();
        assert_eq!(uri.scheme(), "ftp");
        assert_eq!(uri.user_info(), Some("webmaster"));
        assert_eq!(uri.host(), "www.example.org");
        assert_eq!(uri.path(), Some("/"));
    }

    #[test]
    fn escape_sequence_in_username() {
        let uri = "ftp://john%20doe@www.example.org/".parse::<Uri>().unwrap();
        assert_eq!(uri.scheme(), "ftp");
        assert_eq!(uri.user_info(), Some("john%20doe"));
        assert_eq!(uri.host(), "www.example.org");
        assert_eq!(uri.path(), Some("/"));
        // assert_eq!(u.decode_user(), Some("john doe".to_owned()));
    }

    #[test]
    fn empty_query() {
        let uri = "http://www.example.org/?".parse::<Uri>().unwrap();
        assert_eq!(uri.scheme(), "http");
        assert_eq!(uri.host(), "www.example.org");
        assert_eq!(uri.path(), Some("/"));
        assert_eq!(uri.query(), Some(""));
    }

    #[test]
    fn query_ending_in_question_mark() {
        let uri = "http://www.example.org/?foo=bar?".parse::<Uri>().unwrap();
        assert_eq!(uri.scheme(), "http");
        assert_eq!(uri.host(), "www.example.org");
        assert_eq!(uri.path(), Some("/"));
        assert_eq!(uri.query(), Some("foo=bar?"));
    }

    #[test]
    fn query() {
        let uri = "http://www.example.org/?q=rust+language"
            .parse::<Uri>()
            .unwrap();
        assert_eq!(uri.scheme(), "http");
        assert_eq!(uri.host(), "www.example.org");
        assert_eq!(uri.path(), Some("/"));
        assert_eq!(uri.query(), Some("q=rust+language"));
    }

    #[test]
    fn query_with_hex_escaping() {
        let uri = "http://www.example.org/?q=go%20language"
            .parse::<Uri>()
            .unwrap();
        assert_eq!(uri.scheme(), "http");
        assert_eq!(uri.host(), "www.example.org");
        assert_eq!(uri.path(), Some("/"));
        assert_eq!(uri.query(), Some("q=go%20language"));
    }

    #[test]
    fn outside_query() {
        let uri = "http://www.example.org/a%20b?q=c+d".parse::<Uri>().unwrap();
        assert_eq!(uri.scheme(), "http");
        assert_eq!(uri.host(), "www.example.org");
        assert_eq!(uri.path(), Some("/a%20b"));
        assert_eq!(uri.query(), Some("q=c+d"));
        // assert_eq!(s.decode_path(), Some("/a b".to_owned()));
    }

    #[test]
    fn path_without_leading2() {
        let uri = "http://www.example.org/?q=rust+language"
            .parse::<Uri>()
            .unwrap();
        assert_eq!(uri.scheme(), "http");
        assert_eq!(uri.host(), "www.example.org");
        assert_eq!(uri.path(), Some("/"));
        assert_eq!(uri.query(), Some("q=rust+language"));
    }

    // #[test]
    // fn path_without_leading() {
    //         //     let uri = "http:%2f%2fwww.example.org/?q=rust+language".parse::<Uri>().unwrap();
    //     assert_eq!(uri.scheme(), "http");
    //     // Opaque:   "%2f%2fwww.example.org/",
    //     u.query = Some("q=rust+language");
    //         // }

    #[test]
    fn non() {
        let uri = "mailto://webmaster@example.org".parse::<Uri>().unwrap();
        assert_eq!(uri.scheme(), "mailto");
        assert_eq!(uri.user_info(), Some("webmaster"));
        assert_eq!(uri.host(), "example.org");
    }
  
    #[test]
    fn escaped() {
        let uri = "http://%3Fam:pa%3Fsword@google.com".parse::<Uri>().unwrap();
        assert_eq!(uri.scheme(), "http");
        assert_eq!(uri.user_info(), Some("%3Fam:pa%3Fsword"));
        assert_eq!(uri.host(), "google.com");
        // assert_eq!(s.decode_user(), Some("?am".to_owned()));
        // assert_eq!(s.decode_password(), Some("pa?sword".to_owned()));
    }

    #[test]
    fn host_subcomponent() {
        let uri = "http://192.168.0.1/".parse::<Uri>().unwrap();
        assert_eq!(uri.scheme(), "http");
        assert_eq!(uri.host(), "192.168.0.1");
        assert_eq!(uri.path(), Some("/"));
    }

    #[test]
    fn host_and_port_subcomponents() {
        let uri = "http://192.168.0.1:8080/".parse::<Uri>().unwrap();
        assert_eq!(uri.scheme(), "http");
        assert_eq!(uri.host(), "192.168.0.1");
        assert_eq!(uri.port(), Some(8080));
        assert_eq!(uri.path(), Some("/"));
    }

    #[test]
    fn host_subcomponent2() {
        let uri = "http://[fe80::1]/".parse::<Uri>().unwrap();
        assert_eq!(uri.scheme(), "http");
        assert_eq!(uri.host(), "[fe80::1]");
        assert_eq!(uri.path(), Some("/"));
    }

    #[test]
    fn host_and_port_subcomponents2() {
        let uri = "http://[fe80::1]:8080/".parse::<Uri>().unwrap();
        assert_eq!(uri.scheme(), "http");
        assert_eq!(uri.host(), "[fe80::1]");
        assert_eq!(uri.port(), Some(8080));
        assert_eq!(uri.path(), Some("/"));
    }

    // #[test]
    // fn host_subcomponent3() {
    //         //     let uri = "http://[fe80::1%25en0]/".parse::<Uri>().unwrap();
    //     assert_eq!(uri.scheme(), "http");
    //     u.host = "[fe80::1%en0]";
    //     assert_eq!(uri.path(), Some("/"));
    //         // }

    // #[test]
    // fn host_and_port_subcomponents3() {
    //         //     let uri = "http://[fe80::1%25en0]:8080/".parse::<Uri>().unwrap();
    //     assert_eq!(uri.scheme(), "http");
    //     u.host = "[fe80::1%en0]";
    //     u.port = Some("8080");
    //     assert_eq!(uri.path(), Some("/"));
    //         // }

    // #[test]
    // fn host_subcomponent4() {
    //         //     let uri = "http:[fe80::1%25%65%6e%301-._~]/".parse::<Uri>().unwrap();
    //     assert_eq!(uri.scheme(), "http");
    //     u.host = "[fe80::1%en01-._~]";
    //     assert_eq!(uri.path(), Some("/"));
    //         // }

    // #[test]
    // fn host_and_port_subcomponents4() {
    //             let uri = "http:[fe80::1%25%65%6e%301-._~]:8080/".parse::<Uri>().unwrap();
    //     assert_eq!(uri.scheme(), "http");
    //     assert_eq!(uri.host(), "[fe80::1%25%65%6e%301-._~]");
    //     // assert_eq!(uri.host(), "[fe80::1%en01-._~]");
    //     assert_eq!(uri.port(), Some(8080));
    //     assert_eq!(uri.path(), Some("/"));
    //         }

    #[test]
    fn alternate_escapings_of_path_survive_round_trip() {
        let uri = "http://rest.rsc.io/foo%2fbar/baz%2Fquux?alt=media"
            .parse::<Uri>()
            .unwrap();
        assert_eq!(uri.scheme(), "http");
        assert_eq!(uri.host(), "rest.rsc.io");
        assert_eq!(uri.path(), Some("/foo%2fbar/baz%2Fquux"));
        assert_eq!(uri.query(), Some("alt=media"));
        // assert_eq!(s.decode_path(), Some("/foo/bar/baz/quux".to_owned()));
    }

    #[test]
    fn issue_12036() {
        let uri = "mysql://a,b,c/bar".parse::<Uri>().unwrap();
        assert_eq!(uri.scheme(), "mysql");
        assert_eq!(uri.host(), "a,b,c");
        assert_eq!(uri.path(), Some("/bar"));
    }

    #[test]
    fn worst_case_host() {
        let uri = "scheme://!$&'()*+,;=hello!:23/path".parse::<Uri>().unwrap();
        assert_eq!(uri.scheme(), "scheme");
        assert_eq!(uri.host(), "!$&'()*+,;=hello!");
        assert_eq!(uri.port(), Some(23));
        assert_eq!(uri.path(), Some("/path"));
    }

    #[test]
    fn worst_case_path() {
        let uri = "http://host/!$&'()*+,;=:@[hello]".parse::<Uri>().unwrap();
        assert_eq!(uri.scheme(), "http");
        assert_eq!(uri.host(), "host");
        assert_eq!(uri.path(), Some("/!$&'()*+,;=:@[hello]"));
        // Rawu.path = Some("/!$&'()*+,;=:@[hello]");
    }

    #[test]
    fn example() {
        let uri = "http://example.com/oid/[order_id]".parse::<Uri>().unwrap();
        assert_eq!(uri.scheme(), "http");
        assert_eq!(uri.host(), "example.com");
        assert_eq!(uri.path(), Some("/oid/[order_id]"));
        // Rawu.path = Some("/oid/[order_id]");
    }

    #[test]
    fn example2() {
        let uri = "http://192.168.0.2:8080/foo".parse::<Uri>().unwrap();
        assert_eq!(uri.scheme(), "http");
        assert_eq!(uri.host(), "192.168.0.2");
        assert_eq!(uri.port(), Some(8080));
        assert_eq!(uri.path(), Some("/foo"));
    }

    //          //      let uri = "http://192.168.0.2:/foo".parse::<Uri>().unwrap();
    //      		assert_eq!(uri.scheme(), "http");
    //      		u.host = "192.168.0.2:";
    //      		u.path = Some("/foo");
    //          // }

    //          //      	 Malformed IPv6 but still accepted.
    //      let uri = "http://2b01:e34:ef40:7730:8e70:5aff:fefe:edac:8080/foo".parse::<Uri>().unwrap();
    //      		assert_eq!(uri.scheme(), "http");
    //      		u.host = "2b01:e34:ef40:7730:8e70:5aff:fefe:edac:8080";
    //      		u.path = Some("/foo");
    //          // }

    //          //      	 Malformed IPv6 but still accepted.
    //      let uri = "http://2b01:e34:ef40:7730:8e70:5aff:fefe:edac:/foo".parse::<Uri>().unwrap();
    //      		assert_eq!(uri.scheme(), "http");
    //      		u.host = "2b01:e34:ef40:7730:8e70:5aff:fefe:edac:";
    //      		u.path = Some("/foo");
    //          // }

    // #[test]
    // fn ipv6_2() {
    //          //      let uri = "http:[2b01:e34:ef40:7730:8e70:5aff:fefe:edac]:8080/foo".parse::<Uri>().unwrap();
    //      		assert_eq!(uri.scheme(), "http");
    //      		assert_eq!(uri.host(), "[2b01:e34:ef40:7730:8e70:5aff:fefe:edac]:8080");
    //      		assert_eq!(uri.path(), Some("/foo"));
    //          // }

    //          //      let uri = "http:[2b01:e34:ef40:7730:8e70:5aff:fefe:edac]:/foo".parse::<Uri>().unwrap();
    //      		assert_eq!(uri.scheme(), "http");
    //      		u.host = "[2b01:e34:ef40:7730:8e70:5aff:fefe:edac]:";
    //      		u.path = Some("/foo");
    //          // }

    #[test]
    fn example3() {
        let uri = "http://hello.世界.com/foo".parse::<Uri>().unwrap();
        assert_eq!(uri.scheme(), "http");
        assert_eq!(uri.host(), "hello.世界.com");
        assert_eq!(uri.path(), Some("/foo"));
    }

    //          //      let uri = "http://hello.%e4%b8%96%e7%95%8c.com/foo".parse::<Uri>().unwrap();
    //      		assert_eq!(uri.scheme(), "http");
    //      		u.host = "hello.世界.com";
    //      		u.path = Some("/foo");
    //          //      let uri = "http://hello.%E4%B8%96%E7%95%8C.com/foo".parse::<Uri>().unwrap();
    //      }

    //          //      let uri = "http://hello.%E4%B8%96%E7%95%8C.com/foo".parse::<Uri>().unwrap();
    //      		assert_eq!(uri.scheme(), "http");
    //      		u.host = "hello.世界.com";
    //      		u.path = Some("/foo");
    //          // }

    #[test]
    fn example4() {
        let uri = "http://example.com//foo".parse::<Uri>().unwrap();
        assert_eq!(uri.scheme(), "http");
        assert_eq!(uri.host(), "example.com");
        assert_eq!(uri.path(), Some("//foo"));
    }

    #[test]
    fn test_that_we_can_reparse_the_host_names_we_accept() {
        let uri = "myscheme://authority<\"hi\">/foo".parse::<Uri>().unwrap();
        assert_eq!(uri.scheme(), "myscheme");
        assert_eq!(uri.host(), "authority<\"hi\">");
        assert_eq!(uri.path(), Some("/foo"));
    }

    // #[test]
    // fn example5() {
    //         //     let uri = "tcp:[2020::2020:20:2020:2020%25Windows%20Loves%20Spaces]:2020".parse::<Uri>().unwrap();
    //     u.scheme = Some("tcp");
    //     u.host = "[2020::2020:20:2020:2020%Windows Loves Spaces]:2020";
    //         // }
}
