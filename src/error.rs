use std::{error, fmt, io, net, result};
// use failure::Fail;

pub type Result<T> = result::Result<T, Error>;

// #[derive(Debug, Fail)]
// pub enum Error {
//     #[fail(display = "{}", _0)]
//     Io(#[cause] std::io::Error),
//     #[fail(display = "Unknown method {}", _0)]
//     UnknownMethod(String),
//     #[fail(display = "Unsupported version {}", _0)]
//     UnsupportedVersion(String),
//     #[fail(display = "Uri no have scheme")]
//     EmptyScheme,
//     // #[fail(display = "Uri {} no have port", _0)]
//     // NoPort(http::Uri),
//     // #[fail(display = "Uri {} no have host and port", _0)]
//     // NoHostPort(http::Uri),
//     #[fail(display = "Unsupported proxy cheme")]
//     UnsupportedProxyScheme,
//     #[fail(display = "Unsupported scheme {}", _0)]
//     UnsupportedScheme(String),
//     #[fail(display = "None string")]
//     NoneString,
//     #[fail(display = "Parse fragmeng {}", _0)]
//     ParseFragment(String),
//     #[fail(display = "Parse query {}", _0)]
//     ParseQuery(String),
//     #[fail(display = "Parse scheme {}", _0)]
//     ParseScheme(String),
//     #[fail(display = "Parse user info {}", _0)]
//     ParseUserInfo(String),
//     #[fail(display = "Parse host {}", _0)]
//     ParseHost(String),
//     #[fail(display = "Parse ip version 6 {}", _0)]
//     ParseIPv6(String),
//     #[fail(display = "Parse port {}", _0)]
//     ParsePort(String),
// }

// impl From<std::io::Error> for Error {
//     fn from(err: std::io::Error) -> Error {
//         Error::Io(err)
//     }
// }

#[derive(Debug)]
pub enum Error {
    EmptyScheme,
    EmptyAuthority,
    Io(io::Error),
    // Fmt(fmt::Error),
    StdParseAddr(net::AddrParseError),
    NoneString,
    ParseFragment(&'static str),
    ParseHost,
    ParseAddr,
    ParseHeaders,
    ParseIPv6,
    ParsePort,
    ParseQuery(&'static str),
    ParseScheme,
    ParseUserInfo(&'static str),
    UnknownMethod(String),
    UnsupportedProxyScheme,
    UnsupportedScheme(String),
    UnsupportedVersion(String),
}

impl fmt::Display for Error {
    fn fmt(&self, w: &mut fmt::Formatter) -> fmt::Result {
        use self::Error::*;

        match self {
            EmptyScheme => write!(w, "Uri no have scheme"),
            EmptyAuthority => write!(w, "Uri no have authority"),
            Io(e) => write!(w, "{}", e),
            // Fmt(e) => write!(w, "{}", e),
            StdParseAddr(e) => write!(w, "{}", e),
            NoneString => write!(w, "none string"),
            ParseFragment(e) => write!(w, "parse fragmeng {}", e),
            ParseHost => write!(w, "parse host"),
            ParseAddr => write!(w, "parse addr"),
            ParseHeaders => write!(w, "parse headers"),
            ParseIPv6 => write!(w, "parse ip version 6"),
            ParsePort => write!(w, "parse port"),
            ParseQuery(e) => write!(w, "parse query {}", e),
            ParseScheme => write!(w, "parse scheme"),
            ParseUserInfo(e) => write!(w, "parse user info {}", e),
            UnknownMethod(e) => write!(w, "unknown method {}", e),
            UnsupportedProxyScheme => write!(w, "unsupported proxy scheme"),
            UnsupportedScheme(e) => write!(w, "unsupported scheme {}", e),
            UnsupportedVersion(e) => write!(w, "unsupported version {}", e),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        use self::Error::*;

        match self {
            EmptyScheme => "Uri no have scheme",
            EmptyAuthority => "Uri no have authority",
            Io(e) => e.description(),
            // Fmt(e) => e.description(),
            StdParseAddr(e) => e.description(),
            // ParseUrl(e) => e.description(),
            NoneString => "none string",
            ParseFragment(_) => "parse fragmeng",
            ParseHost => "parse host",
            ParseAddr => "parse addr",
            ParseHeaders => "parse headers",
            ParseIPv6 => "parse ip version 6",
            ParsePort => "parse port",
            ParseQuery(_) => "parse query",
            ParseScheme => "parse scheme",
            ParseUserInfo(_) => "parse user info",
            UnknownMethod(_) => "unknown method",
            UnsupportedProxyScheme => "unsupported proxy scheme",
            UnsupportedScheme(_) => "unsupported scheme",
            UnsupportedVersion(_) => "unsupported version",
        }
    }

    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        use self::Error::*;

        match self {
            EmptyScheme => None,
            EmptyAuthority => None,
            Io(e) => e.source(),
            // Fmt(e) => e.source(),
            StdParseAddr(e) => e.source(),
            // ParseUrl(e) => e.source(),
            NoneString => None,
            ParseFragment(_) => None,
            ParseHost => None,
            ParseAddr => None,
            ParseHeaders => None,
            ParseIPv6 => None,
            ParsePort => None,
            ParseQuery(_) => None,
            ParseScheme => None,
            ParseUserInfo(_) => None,
            UnknownMethod(_) => None,
            UnsupportedProxyScheme => None,
            UnsupportedScheme(_) => None,
            UnsupportedVersion(_) => None,
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::Io(err)
    }
}

// impl From<fmt::Error> for Error {
//     fn from(err: fmt::Error) -> Error {
//         Error::Fmt(err)
//     }
// }

impl From<net::AddrParseError> for Error {
    fn from(err: net::AddrParseError) -> Error {
        Error::StdParseAddr(err)
    }
}

// impl From<url::ParseError> for Error {
//     fn from(err: url::ParseError) -> Error {
//         Error::ParseUrl(err)
//     }
// }

// Io{source: std::io::Error} = "IO Error: {:?}",
// UnknownMethod{method: String} = "Unknown method {method}",
// UnsupportedVersion{version: String} = "Unsupported version {version}",
// EmptyScheme = "Uri no have scheme",
// UnsupportedProxyScheme = "Unsupported proxy scheme",
// UnsupportedScheme{scheme: String} = "Unsupported scheme {scheme}",
// NoneString = "None string",
// ParseFragment{fragmeng: String} = "Parse fragmeng {fragmeng}",
// ParseQuery{query: String} = "Parse query {query}",
// ParseScheme{scheme: String} = "Parse scheme {scheme}",
// ParseUserInfo{userinfo:String} = "Parse user info {userinfo}",
// ParseHost{host:String} = "Parse host {host}",
// ParseIPv6{ipv6: String} = "Parse ip version 6 {ipv6}",
// ParsePort{port:String} = "Parse port {port}",

// #[fail(display = "Uri {} no have port", _0)]
// NoPort(http::Uri),
// #[fail(display = "Uri {} no have host and port", _0)]
// NoHostPort(http::Uri),
