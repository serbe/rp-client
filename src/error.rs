use failure::Fail;

// use crate::url::Url;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "{}", _0)]
    Io(#[cause] std::io::Error),
    #[fail(display = "Unknown method {}", _0)]
    UnknownMethod(String),
    #[fail(display = "Unsupported version {}", _0)]
    UnsupportedVersion(String),
    #[fail(display = "Uri no have scheme")]
    EmptyScheme,
    // #[fail(display = "Uri {} no have port", _0)]
    // NoPort(http::Uri),
    // #[fail(display = "Uri {} no have host and port", _0)]
    // NoHostPort(http::Uri),
    // #[fail(display = "Uncnown proxy cheme {}", _0)]
    // UnknownProxyScheme(http::Uri),
    #[fail(display = "Unsupported scheme {}", _0)]
    UnsupportedScheme(String),
    #[fail(display = "None string")]
    NoneString,
    #[fail(display = "Parse fragmeng {}", _0)]
    ParseFragment(String),
    #[fail(display = "Parse query {}", _0)]
    ParseQuery(String),
    #[fail(display = "Parse scheme {}", _0)]
    ParseScheme(String),
    #[fail(display = "Parse user info {}", _0)]
    ParseUserInfo(String),
    #[fail(display = "Parse host {}", _0)]
    ParseHost(String),
    #[fail(display = "Parse ip version 6 {}", _0)]
    ParseIPv6(String),
    #[fail(display = "Parse port {}", _0)]
    ParsePort(String),
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Error {
        Error::Io(err)
    }
}

// impl From<http::uri::InvalidUri> for Error {
//     fn from(err: http::uri::InvalidUri) -> Error {
//         Error::InvalidUri(err)
//     }
// }
