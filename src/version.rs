use std::fmt;
use std::str::FromStr;

use crate::error::{Error, Result};

#[derive(PartialEq, PartialOrd, Copy, Clone, Eq, Ord, Hash)]
pub enum Version {
    Http09,
    Http10,
    Http11,
}

pub trait IntoVersion {
    fn into_version(self) -> Result<Version>;
}

impl IntoVersion for Version {
    fn into_version(self) -> Result<Version> {
        Ok(self)
    }
}

impl<'a> IntoVersion for &'a str {
    fn into_version(self) -> Result<Version> {
        self.parse()
    }
}

impl<'a> IntoVersion for String {
    fn into_version(self) -> Result<Version> {
        self.parse()
    }
}

impl Default for Version {
    fn default() -> Self {
        Version::Http11
    }
}

impl FromStr for Version {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s.to_uppercase().as_str() {
            "HTTP/0.9" => Ok(Version::Http09),
            "HTTP/1.0" => Ok(Version::Http10),
            "HTTP/1.1" => Ok(Version::Http11),
            _ => Err(Error::UnsupportedVersion(s.to_owned())),
        }
    }
}

impl fmt::Debug for Version {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match self {
            Version::Http09 => "HTTP/0.9",
            Version::Http10 => "HTTP/1.0",
            Version::Http11 => "HTTP/1.1",
        })
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     // #[test]

// }
