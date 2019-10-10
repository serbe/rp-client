use crate::error::{Error, Result};
use std::{
    fmt,
    ops::{Index, Range},
    str,
    string::ToString,
};

const HTTP_PORT: u16 = 80;
const HTTPS_PORT: u16 = 443;

///A (half-open) range bounded inclusively below and exclusively above (start..end) with `Copy`.
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

///Representation of Uniform Resource Identifier
///
///# Example
///```
///use http_req::uri::Uri;
///
///let uri: Uri = "https://user:info@foo.com:12/bar/baz?query#fragment".parse().unwrap();
///assert_eq!(uri.host(), Some("foo.com"));
///```
#[derive(Clone, Debug, PartialEq)]
pub struct Uri {
    inner: String,
    scheme: RangeC,
    authority: Option<Authority>,
    path: Option<RangeC>,
    query: Option<RangeC>,
    fragment: Option<RangeC>,
}

impl Uri {
    ///Returns scheme of this `Uri`.
    ///
    ///# Example
    ///```
    ///use http_req::uri::Uri;
    ///
    ///let uri: Uri = "https://user:info@foo.com:12/bar/baz?query#fragment".parse().unwrap();
    ///assert_eq!(uri.scheme(), "https");
    ///```
    pub fn scheme(&self) -> &str {
        &self.inner[self.scheme]
    }

    ///Returns information about the user included in this `Uri`.
    ///     
    ///# Example
    ///```
    ///use http_req::uri::Uri;
    ///
    ///let uri: Uri = "https://user:info@foo.com:12/bar/baz?query#fragment".parse().unwrap();
    ///assert_eq!(uri.user_info(), Some("user:info"));
    ///```
    pub fn user_info(&self) -> Option<&str> {
        match self.authority {
            Some(ref a) => a.user_info(),
            None => None,
        }
    }

    ///Returns host of this `Uri`.
    ///     
    ///# Example
    ///```
    ///use http_req::uri::Uri;
    ///
    ///let uri: Uri = "https://user:info@foo.com:12/bar/baz?query#fragment".parse().unwrap();
    ///assert_eq!(uri.host(), Some("foo.com"));
    ///```
    pub fn host(&self) -> Option<&str> {
        match self.authority {
            Some(ref a) => Some(a.host()),
            None => None,
        }
    }

    ///Returns host of this `Uri` to use in a header.
    ///     
    ///# Example
    ///```
    ///use http_req::uri::Uri;
    ///
    ///let uri: Uri = "https://user:info@foo.com:12/bar/baz?query#fragment".parse().unwrap();
    ///assert_eq!(uri.host_header(), Some("foo.com:12".to_string()));
    ///```
    pub fn host_header(&self) -> Option<String> {
        match self.host() {
            Some(h) => match self.corr_port() {
                HTTP_PORT | HTTPS_PORT => Some(h.to_string()),
                p => Some(format!("{}:{}", h, p)),
            },
            _ => None,
        }
    }

    ///Returns port of this `Uri`
    ///     
    ///# Example
    ///```
    ///use http_req::uri::Uri;
    ///
    ///let uri: Uri = "https://user:info@foo.com:12/bar/baz?query#fragment".parse().unwrap();
    ///assert_eq!(uri.port(), Some(12));
    ///```
    pub fn port(&self) -> Option<u16> {
        match &self.authority {
            Some(a) => a.port(),
            None => None,
        }
    }

    ///Returns port corresponding to this `Uri`.
    ///Returns default port if it hasn't been set in the uri.
    ///  
    ///# Example
    ///```
    ///use http_req::uri::Uri;
    ///
    ///let uri: Uri = "https://user:info@foo.com:12/bar/baz?query#fragment".parse().unwrap();
    ///assert_eq!(uri.corr_port(), 12);
    ///```
    pub fn corr_port(&self) -> u16 {
        let default_port = match self.scheme() {
            "https" => HTTPS_PORT,
            _ => HTTP_PORT,
        };

        match self.authority {
            Some(ref a) => a.port().unwrap_or(default_port),
            None => default_port,
        }
    }

    ///Returns path of this `Uri`.
    ///  
    ///# Example
    ///```
    ///use http_req::uri::Uri;
    ///
    ///let uri: Uri = "https://user:info@foo.com:12/bar/baz?query#fragment".parse().unwrap();
    ///assert_eq!(uri.path(), Some("/bar/baz"));
    ///```
    pub fn path(&self) -> Option<&str> {
        self.path.map(|r| &self.inner[r])
    }

    ///Returns query of this `Uri`.
    ///  
    ///# Example
    ///```
    ///use http_req::uri::Uri;
    ///
    ///let uri: Uri = "https://user:info@foo.com:12/bar/baz?query#fragment".parse().unwrap();
    ///assert_eq!(uri.query(), Some("query"));
    ///```
    pub fn query(&self) -> Option<&str> {
        self.query.map(|r| &self.inner[r])
    }

    ///Returns fragment of this `Uri`.
    ///  
    ///# Example
    ///```
    ///use http_req::uri::Uri;
    ///
    ///let uri: Uri = "https://user:info@foo.com:12/bar/baz?query#fragment".parse().unwrap();
    ///assert_eq!(uri.fragment(), Some("fragment"));
    ///```
    pub fn fragment(&self) -> Option<&str> {
        self.fragment.map(|r| &self.inner[r])
    }

    ///Returns resource `Uri` points to.
    ///  
    ///# Example
    ///```
    ///use http_req::uri::Uri;
    ///
    ///let uri: Uri = "https://user:info@foo.com:12/bar/baz?query#fragment".parse().unwrap();
    ///assert_eq!(uri.resource(), "/bar/baz?query#fragment");
    ///```
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
}

impl fmt::Display for Uri {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let uri = if let Some(auth) = &self.authority {
            let mut uri = self.inner.to_string();
            let auth = auth.to_string();
            let start = self.scheme.end + 3;

            uri.replace_range(start..(start + auth.len()), &auth);
            uri
        } else {
            self.inner.to_string()
        };

        write!(f, "{}", uri)
    }
}

impl str::FromStr for Uri {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut s = s.to_string();
        remove_spaces(&mut s);

        let (uri_part, fragment) = get_chunks(&s, Some(RangeC::new(0, s.len())), "#", true);
        let (uri_part, query) = get_chunks(&s, uri_part, "?", true);

        let (scheme, maybe_part) = get_chunks(&s, uri_part, ":", true);
        let (scheme, uri_part) = if let Some(scheme) = get_scheme(&s, scheme) {
            (scheme, maybe_part)
        } else {
            (RangeC::new(0, 0), uri_part)
        };

        let mut authority = None;

        if let Some(u) = &uri_part {
            if s[*u].contains("//") {
                let (auth, part) = get_chunks(&s, Some(RangeC::new(u.start + 2, u.end)), "/", true);

                authority = if let Some(a) = auth {
                    Some(s[a].parse()?)
                } else {
                    None
                };

                uri_part = part;
            }
        }

        let (mut path, uri_part) = get_chunks(&s, uri_part, "?", false);

        if authority.is_some() || &s[scheme] == "file" {
            path = path.map(|p| RangeC::new(p.start - 1, p.end));
        }


        Ok(Uri {
            inner: s,
            scheme,
            authority,
            path,
            query,
            fragment,
        })
    }
}

// impl str::FromStr for Uri {
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


///Authority of Uri
///
///# Example
///```
///use http_req::uri::Authority;
///
///let auth: Authority = "user:info@foo.com:443".parse().unwrap();
///assert_eq!(auth.host(), "foo.com");
///```
#[derive(Clone, Debug, PartialEq)]
pub struct Authority {
    inner: String,
    username: Option<RangeC>,
    password: Option<RangeC>,
    host: RangeC,
    port: Option<RangeC>,
}

impl Authority {
    ///Returns username of this `Authority`
    ///
    ///# Example
    ///```
    ///use http_req::uri::Authority;
    ///
    ///let auth: Authority = "user:info@foo.com:443".parse().unwrap();
    ///assert_eq!(auth.username(), Some("user"));
    ///```
    pub fn username(&self) -> Option<&str> {
        self.username.map(|r| &self.inner[r])
    }

    ///Returns password of this `Authority`
    ///
    ///# Example
    ///```
    ///use http_req::uri::Authority;
    ///
    ///let auth: Authority = "user:info@foo.com:443".parse().unwrap();
    ///assert_eq!(auth.password(), Some("info"));
    ///```
    pub fn password(&self) -> Option<&str> {
        self.password.map(|r| &self.inner[r])
    }

    ///Returns information about the user
    ///
    ///# Example
    ///```
    ///use http_req::uri::Authority;
    ///
    ///let auth: Authority = "user:info@foo.com:443".parse().unwrap();
    ///assert_eq!(auth.user_info(), Some("user:info"));
    ///```
    pub fn user_info(&self) -> Option<&str> {
        match (&self.username, &self.password) {
            (Some(u), Some(p)) => Some(&self.inner[u.start..p.end]),
            (Some(u), None) => Some(&self.inner[*u]),
            _ => None,
        }
    }

    ///Returns host of this `Authority`
    ///
    ///# Example
    ///```
    ///use http_req::uri::Authority;
    ///
    ///let auth: Authority = "user:info@foo.com:443".parse().unwrap();
    ///assert_eq!(auth.host(), "foo.com");
    ///```
    pub fn host(&self) -> &str {
        &self.inner[self.host]
    }

    ///Returns port of this `Authority`
    ///
    ///# Example
    ///```
    ///use http_req::uri::Authority;
    ///
    ///let auth: Authority = "user:info@foo.com:443".parse().unwrap();
    ///assert_eq!(auth.port(), Some(443));
    ///```
    pub fn port(&self) -> Option<u16> {
        match &self.port {
            Some(p) => Some(self.inner[*p].parse().unwrap()),
            None => None,
        }
    }
}

impl str::FromStr for Authority {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let inner = s.to_string();

        let mut username = None;
        let mut password = None;

        let uri_part = if s.contains('@') {
            let (info, part) = get_chunks(&s, Some(RangeC::new(0, s.len())), "@", true);
            let (name, pass) = get_chunks(&s, info, ":", true);

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
        let (host, port) = get_chunks(&s, uri_part, split_by, true);
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

//Splits `s` by `separator`. If `separator` is found inside `s`, it will return two `Some` values
//consisting `RangeC` of each `&str`. If `separator` is at the end of `s` or it's not found,
//it will return tuple consisting `Some` with `RangeC` of entire `s` inside and None.
fn get_chunks<'a>(
    s: &'a str,
    range: Option<RangeC>,
    separator: &'a str,
    cut: bool,
) -> (Option<RangeC>, Option<RangeC>) {
    if let Some(r) = range {
        let range = Range::from(r);
        let c = if cut { 0 } else { 1 };

        match s[range.clone()].find(separator) {
            Some(i) => {
                let mid = r.start + i + separator.len();
                let before = Some(RangeC::new(r.start, mid - 1)).filter(|r| r.start != r.end);
                let after = Some(RangeC::new(mid - c, r.end)).filter(|r| r.start != r.end);

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

fn get_scheme<'a>(
    s: &mut String,
    range: Option<RangeC>,
) -> Option<RangeC> {
    if let Some(r) = range {
        let range = Range::from(r);

        if s[range].chars().all(|c| c.is_alphanumeric()) {
            s.replacen(s[range], s[range].to_lowercase(), 1);
                    Ok(Scheme::Other(s.to_lowercase()))
                }
        match s[range.clone()].find(separator) {
            Some(i) => {
                let mid = r.start + i + separator.len();
                let before = Some(RangeC::new(r.start, mid - 1)).filter(|r| r.start != r.end);
                let after = Some(RangeC::new(mid - c, r.end)).filter(|r| r.start != r.end);

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
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_path() {
        let uri = "http://www.example.org".parse::<Uri>().unwrap();
        assert_eq!(uri.scheme(), "http");
        assert_eq!(uri.host(), Some("www.example.org"));
    }

    #[test]
    fn with_path() {
        let uri = "http://www.example.org/".parse::<Uri>().unwrap();
        assert_eq!(uri.scheme(), "http");
        assert_eq!(uri.host(), Some("www.example.org"));
        assert_eq!(uri.path(), Some("/"));
    }

    #[test]
    fn path_with_hex_escaping() {
        let uri = "http://www.example.org/file%20one%26two"
            .parse::<Uri>()
            .unwrap();
        assert_eq!(uri.scheme(), "http");
        assert_eq!(uri.host(), Some("www.example.org"));
        assert_eq!(uri.path(), Some("/file%20one%26two"));
        // assert_eq!(u.decode_path(), Some("/file one&two".to_owned()));
    }

    #[test]
    fn user() {
        let uri = "ftp://webmaster@www.example.org/".parse::<Uri>().unwrap();
        assert_eq!(uri.scheme(), "ftp");
        assert_eq!(uri.user_info(), Some("webmaster"));
        assert_eq!(uri.host(), Some("www.example.org"));
        assert_eq!(uri.path(), Some("/"));
    }

    #[test]
    fn escape_sequence_in_username() {
        let uri = "ftp://john%20doe@www.example.org/".parse::<Uri>().unwrap();
        assert_eq!(uri.scheme(), "ftp");
        assert_eq!(uri.user_info(), Some("john%20doe"));
        assert_eq!(uri.host(), Some("www.example.org"));
        assert_eq!(uri.path(), Some("/"));
        // assert_eq!(u.decode_user(), Some("john doe".to_owned()));
    }

    #[test]
    fn empty_query() {
        let uri = "http://www.example.org/?".parse::<Uri>().unwrap();
        assert_eq!(uri.scheme(), "http");
        assert_eq!(uri.host(), Some("www.example.org"));
        assert_eq!(uri.path(), Some("/"));
        assert_eq!(uri.query(), Some(""));
    }

    #[test]
    fn query_ending_in_question_mark() {
        let uri = "http://www.example.org/?foo=bar?".parse::<Uri>().unwrap();
        assert_eq!(uri.scheme(), "http");
        assert_eq!(uri.host(), Some("www.example.org"));
        assert_eq!(uri.path(), Some("/"));
        assert_eq!(uri.query(), Some("foo=bar?"));
    }

    #[test]
    fn query() {
        let uri = "http://www.example.org/?q=rust+language"
            .parse::<Uri>()
            .unwrap();
        assert_eq!(uri.scheme(), "http");
        assert_eq!(uri.host(), Some("www.example.org"));
        assert_eq!(uri.path(), Some("/"));
        assert_eq!(uri.query(), Some("q=rust+language"));
    }

    #[test]
    fn query_with_hex_escaping() {
        let uri = "http://www.example.org/?q=go%20language"
            .parse::<Uri>()
            .unwrap();
        assert_eq!(uri.scheme(), "http");
        assert_eq!(uri.host(), Some("www.example.org"));
        assert_eq!(uri.path(), Some("/"));
        assert_eq!(uri.query(), Some("q=go%20language"));
    }

    #[test]
    fn outside_query() {
        let uri = "http://www.example.org/a%20b?q=c+d".parse::<Uri>().unwrap();
        assert_eq!(uri.scheme(), "http");
        assert_eq!(uri.host(), Some("www.example.org"));
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
        assert_eq!(uri.host(), Some("www.example.org"));
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
        assert_eq!(uri.host(), Some("example.org"));
    }

    #[test]
    fn unescaped() {
        let uri = "/foo?query=http://bad".parse::<Uri>().unwrap();
        assert_eq!(uri.path(), Some("/foo"));
        assert_eq!(uri.query(), Some("query=http://bad"));
    }

    #[test]
    fn leading() {
        let uri = "//foo".parse::<Uri>().unwrap();
        assert_eq!(uri.host(), Some("foo"));
    }

    #[test]
    fn leading2() {
        let uri = "user@foo/path?a=b".parse::<Uri>().unwrap();
        assert_eq!(uri.user_info(), Some("user"));
        assert_eq!(uri.host(), Some("foo"));
        assert_eq!(uri.path(), Some("/path"));
        assert_eq!(uri.query(), Some("a=b"));
    }

    #[test]
    fn same_codepath() {
        let uri = "/threeslashes".parse::<Uri>().unwrap();
        assert_eq!(uri.path(), Some("/threeslashes"));
    }

    // #[test]
    // fn relative_path() {
    // 	    // 	let uri = "a/b/c".parse::<Uri>().unwrap();
    // 	assert_eq!(uri.path(), Some("a/b/c"));
    // 	    // }

    #[test]
    fn escaped() {
        let uri = "http://%3Fam:pa%3Fsword@google.com".parse::<Uri>().unwrap();
        assert_eq!(uri.scheme(), "http");
        assert_eq!(uri.user_info(), Some("%3Fam:pa%3Fsword"));
        assert_eq!(uri.host(), Some("google.com"));
        // assert_eq!(s.decode_user(), Some("?am".to_owned()));
        // assert_eq!(s.decode_password(), Some("pa?sword".to_owned()));
    }

    #[test]
    fn host_subcomponent() {
        let uri = "http://192.168.0.1/".parse::<Uri>().unwrap();
        assert_eq!(uri.scheme(), "http");
        assert_eq!(uri.host(), Some("192.168.0.1"));
        assert_eq!(uri.path(), Some("/"));
    }

    #[test]
    fn host_and_port_subcomponents() {
        let uri = "http://192.168.0.1:8080/".parse::<Uri>().unwrap();
        assert_eq!(uri.scheme(), "http");
        assert_eq!(uri.host(), Some("192.168.0.1"));
        assert_eq!(uri.port(), Some(8080));
        assert_eq!(uri.path(), Some("/"));
    }

    #[test]
    fn host_subcomponent2() {
        let uri = "http://[fe80::1]/".parse::<Uri>().unwrap();
        assert_eq!(uri.scheme(), "http");
        assert_eq!(uri.host(), Some("[fe80::1]"));
        assert_eq!(uri.path(), Some("/"));
    }

    #[test]
    fn host_and_port_subcomponents2() {
        let uri = "http://[fe80::1]:8080/".parse::<Uri>().unwrap();
        assert_eq!(uri.scheme(), "http");
        assert_eq!(uri.host(), Some("[fe80::1]"));
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
    //         //     let uri = "http:[fe80::1%25%65%6e%301-._~]:8080/".parse::<Uri>().unwrap();
    //     assert_eq!(uri.scheme(), "http");
    //     u.host = "[fe80::1%en01-._~]";
    //     u.port = Some("8080");
    //     assert_eq!(uri.path(), Some("/"));
    //         // }

    #[test]
    fn alternate_escapings_of_path_survive_round_trip() {
        let uri = "http://rest.rsc.io/foo%2fbar/baz%2Fquux?alt=media"
            .parse::<Uri>()
            .unwrap();
        assert_eq!(uri.scheme(), "http");
        assert_eq!(uri.host(), Some("rest.rsc.io"));
        assert_eq!(uri.path(), Some("/foo%2fbar/baz%2Fquux"));
        assert_eq!(uri.query(), Some("alt=media"));
        // assert_eq!(s.decode_path(), Some("/foo/bar/baz/quux".to_owned()));
    }

    #[test]
    fn issue_12036() {
        let uri = "mysql://a,b,c/bar".parse::<Uri>().unwrap();
        assert_eq!(uri.scheme(), "mysql");
        assert_eq!(uri.host(), Some("a,b,c"));
        assert_eq!(uri.path(), Some("/bar"));
    }

    #[test]
    fn worst_case_host() {
        let uri = "scheme://!$&'()*+,;=hello!:23/path".parse::<Uri>().unwrap();
        assert_eq!(uri.scheme(), "scheme");
        assert_eq!(uri.host(), Some("!$&'()*+,;=hello!"));
        assert_eq!(uri.port(), Some(23));
        assert_eq!(uri.path(), Some("/path"));
    }

    // #[test]
    // fn worst_case_path() {
    //         //     let uri = "http://host/!$&'()*+,;=:@[hello]".parse::<Uri>().unwrap();
    //     assert_eq!(uri.scheme(), "http");
    //     assert_eq!(uri.host(), Some("host"));
    //     assert_eq!(uri.path(), Some("/!$&'()*+,;=:@[hello]"));
    //     // Rawu.path = Some("/!$&'()*+,;=:@[hello]");
    //         // }

    #[test]
    fn example() {
        let uri = "http://example.com/oid/[order_id]".parse::<Uri>().unwrap();
        assert_eq!(uri.scheme(), "http");
        assert_eq!(uri.host(), Some("example.com"));
        assert_eq!(uri.path(), Some("/oid/[order_id]"));
        // Rawu.path = Some("/oid/[order_id]");
    }

    #[test]
    fn example2() {
        let uri = "http://192.168.0.2:8080/foo".parse::<Uri>().unwrap();
        assert_eq!(uri.scheme(), "http");
        assert_eq!(uri.host(), Some("192.168.0.2"));
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
    //      		assert_eq!(uri.host(), Some("[2b01:e34:ef40:7730:8e70:5aff:fefe:edac]:8080"));
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
        assert_eq!(uri.host(), Some("hello.世界.com"));
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
        assert_eq!(uri.host(), Some("example.com"));
        assert_eq!(uri.path(), Some("//foo"));
    }

    #[test]
    fn test_that_we_can_reparse_the_host_names_we_accept() {
        let uri = "myscheme://authority<\"hi\">/foo".parse::<Uri>().unwrap();
        assert_eq!(uri.scheme(), "myscheme");
        assert_eq!(uri.host(), Some("authority<\"hi\">"));
        assert_eq!(uri.path(), Some("/foo"));
    }

    // #[test]
    // fn example5() {
    //         //     let uri = "tcp:[2020::2020:20:2020:2020%25Windows%20Loves%20Spaces]:2020".parse::<Uri>().unwrap();
    //     u.scheme = Some("tcp");
    //     u.host = "[2020::2020:20:2020:2020%Windows Loves Spaces]:2020";
    //         // }
}
