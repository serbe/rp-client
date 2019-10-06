use crate::error::{Error, Result};

pub enum Scheme {
    HTTP,
    HTTPS,
    SOCKS5,
    SOCKS5H,
}

pub trait IntoScheme {
    fn into_scheme(self) -> Result<Scheme>;
}

impl Scheme {
    pub fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "http" => Ok(Scheme::HTTP),
            "https" => Ok(Scheme::HTTPS),
            "socks5" => Ok(Scheme::SOCKS5),
            "socks5h" => Ok(Scheme::SOCKS5H),
            _ => Err(Error::UnsupportedScheme(s.to_owned())),
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            Scheme::HTTP => "http",
            Scheme::HTTPS => "https",
            Scheme::SOCKS5 => "socks5",
            Scheme::SOCKS5H => "socks5h",
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Scheme::HTTP => "http".to_string(),
            Scheme::HTTPS => "https".to_string(),
            Scheme::SOCKS5 => "socks5".to_string(),
            Scheme::SOCKS5H => "socks5h".to_string(),
        }
    }
}

impl<'a> IntoScheme for &'a str {
    fn into_scheme(self) -> Result<Scheme> {
        Scheme::from_str(self)
    }
}
