use std::net::{SocketAddrV4, SocketAddrV6};
use std::str::FromStr;

use crate::error::{Result, Error};
use crate::url::Url;

#[derive(Clone, Debug, PartialEq)]
pub enum Addr {
    Ipv4(SocketAddrV4),
    Ipv6(SocketAddrV6),
    Domain(String),
}

// impl Addr {
//     pub fn from_str(s: &str) -> Result<Self> {
//         if let Ok(ipv4) = s.parse::<SocketAddrV4>() {
//             Ok(Addr::Ipv4(ipv4))
//         } else if let Ok(ipv6) = s.parse::<SocketAddrV6>() {
//             Ok(Addr::Ipv6(ipv6))
//         } else {
//             match s.parse::<Url>() {
//                 Ok(_) => Ok(Addr::Domain(s.to_owned())),
//                 Err(e) => Err(e),
//             }
//         }
//     }
// }

impl FromStr for Addr {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        if let Ok(ipv4) = s.parse::<SocketAddrV4>() {
            Ok(Addr::Ipv4(ipv4))
        } else if let Ok(ipv6) = s.parse::<SocketAddrV6>() {
            Ok(Addr::Ipv6(ipv6))
        } else {
            match s.parse::<Url>() {
                Ok(_) => Ok(Addr::Domain(s.to_owned())),
                Err(e) => Err(e),
            }
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use std::net::{Ipv4Addr, Ipv6Addr, SocketAddrV4, SocketAddrV6};

//     #[test]
//     fn ipv4() {
//         assert_eq!(
//             Addr::from_str("127.0.0.1:5858"),
//             Ok(Addr::Ipv4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 5858)))
//         );
//     }

//     #[test]
//     fn ipv6() {
//         assert_eq!(
//             Addr::from_str("[2001:0db8:11a3:09d7:1f34:8a2e:07a0:765d]:5858"),
//             Addr::Ipv6(SocketAddrV6::new(
//                 Ipv6Addr::new(0x2001, 0xdb8, 0x11a3, 0x9d7, 0x1f34, 0x8a2e, 0x7a0, 0x765d),
//                 5858,
//                 0,
//                 0
//             ))
//         );
//     }

//     #[test]
//     fn domain() {
//         assert_eq!(
//             Addr::from_str("test.com:5858"),
//             Addr::Domain("test.com:5858".to_string())
//         );
//     }
// }
