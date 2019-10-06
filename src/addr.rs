use std::net::{Ipv4Addr, Ipv6Addr};

#[derive(Clone, Debug)]
pub enum Addr {
    Ipv4(Ipv4Addr),
    Ipv6(Ipv6Addr),
    Domain(String),
}
