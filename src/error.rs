use failure::Fail;

use crate::url::Url;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "{}", _0)]
    Io(#[cause] std::io::Error),
    #[fail(display = "{}", _0)]
    InvalidUri(#[cause] http::uri::InvalidUri),
    #[fail(display = "Uri {} no have host", _0)]
    NoHost(http::Uri),
    #[fail(display = "Uri {:?} no have scheme", _0)]
    NoScheme(http::Uri),
    #[fail(display = "Uri {} no have port", _0)]
    NoPort(http::Uri),
    #[fail(display = "Uri {} no have host and port", _0)]
    NoHostPort(http::Uri),
    #[fail(display = "Uncnown proxy cheme {}", _0)]
    UnknownProxyScheme(http::Uri),
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Error {
        Error::Io(err)
    }
}

impl From<http::uri::InvalidUri> for Error {
    fn from(err: http::uri::InvalidUri) -> Error {
        Error::InvalidUri(err)
    }
}
