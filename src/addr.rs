use std::net::{SocketAddrV4, SocketAddrV6};
use std::str::FromStr;

use crate::error::{Error, Result};
use crate::uri::Uri;

#[derive(Clone, Debug, PartialEq)]
pub enum Addr {
    Ipv4(SocketAddrV4),
    Ipv6(SocketAddrV6),
    Domain(String),
}

pub trait IntoAddr {
    fn into_addr(self) -> Result<Addr>;
}

impl IntoAddr for Addr {
    fn into_addr(self) -> Result<Addr> {
        Ok(self)
    }
}

impl<'a> IntoAddr for &'a str {
    fn into_addr(self) -> Result<Addr> {
        self.parse()
    }
}

impl IntoAddr for String {
    fn into_addr(self) -> Result<Addr> {
        self.parse()
    }
}

impl Into<Addr> for &str {
    fn into(self) -> Addr {
        if let Ok(ipv4) = self.parse::<SocketAddrV4>() {
            Addr::Ipv4(ipv4)
        } else if let Ok(ipv6) = self.parse::<SocketAddrV6>() {
            Addr::Ipv6(ipv6)
        } else {
            Addr::Domain(self.to_owned())
        }
    }
}

impl FromStr for Addr {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        if let Ok(ipv4) = s.parse::<SocketAddrV4>() {
            Ok(Addr::Ipv4(ipv4))
        } else if let Ok(ipv6) = s.parse::<SocketAddrV6>() {
            Ok(Addr::Ipv6(ipv6))
        } else {
            match s.parse::<Uri>() {
                Ok(_) => Ok(Addr::Domain(s.to_owned())),
                Err(e) => Err(e),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::{Ipv4Addr, Ipv6Addr, SocketAddrV4, SocketAddrV6};

    #[test]
    fn ipv4() {
        assert_eq!(
            Addr::from_str("127.0.0.1:5858").unwrap(),
            Addr::Ipv4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 5858))
        );
    }

    #[test]
    fn ipv6() {
        assert_eq!(
            Addr::from_str("[2001:0db8:11a3:09d7:1f34:8a2e:07a0:765d]:5858").unwrap(),
            Addr::Ipv6(SocketAddrV6::new(
                Ipv6Addr::new(0x2001, 0xdb8, 0x11a3, 0x9d7, 0x1f34, 0x8a2e, 0x7a0, 0x765d),
                5858,
                0,
                0
            ))
        );
    }

    #[test]
    fn domain() {
        assert_eq!(
            Addr::from_str("http://test.com:5858").unwrap(),
            Addr::Domain("http://test.com:5858".to_string())
        );
    }

    #[test]
    fn err_domain() {
        assert!(Addr::from_str("test.com::5858").is_err());
    }
}
