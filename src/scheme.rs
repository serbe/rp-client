use std::str::FromStr;
use std::fmt;

use crate::error::{Error, Result};

#[derive(Clone, Debug)]
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
    pub fn from_opt(scheme: Option<String>) -> Result<Self> {
        match scheme {
            Some(scheme) => match scheme.to_lowercase().as_str() {
                "http" => Ok(Scheme::HTTP),
                "https" => Ok(Scheme::HTTPS),
                "socks5" => Ok(Scheme::SOCKS5),
                "socks5h" => Ok(Scheme::SOCKS5H),
                _ => Err(Error::UnsupportedScheme(scheme.to_owned())),
            },
            None => Err(Error::UnsupportedScheme("empty scheme".to_string())),
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

    pub fn default_port(&self) -> u16 {
        match self {
            Scheme::HTTP => 80,
            Scheme::HTTPS => 443,
            Scheme::SOCKS5 => 1080,
            Scheme::SOCKS5H => 1080,
        }
    }
}

impl Default for Scheme {
    fn default() -> Self {
        Scheme::HTTP
    }
}

impl<'a> IntoScheme for &'a str {
    fn into_scheme(self) -> Result<Scheme> {
        Scheme::from_str(self)
    }
}

impl FromStr for Scheme {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "http" => Ok(Scheme::HTTP),
            "https" => Ok(Scheme::HTTPS),
            "socks5" => Ok(Scheme::SOCKS5),
            "socks5h" => Ok(Scheme::SOCKS5H),
            _ => Err(Error::UnsupportedScheme(s.to_owned())),
        }
    }
}

impl fmt::Display for Scheme {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Scheme::HTTP => write!(f, "http"),
            Scheme::HTTPS => write!(f, "https"),
            Scheme::SOCKS5 => write!(f, "socks5"),
            Scheme::SOCKS5H => write!(f, "socks5h"),
        }
    }
}