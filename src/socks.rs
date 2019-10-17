use std::io::{self, Read, Write};
use std::net::{Ipv4Addr, Ipv6Addr, TcpStream};

use crate::addr::Addr;
use crate::error::{Error, Result};
use crate::request::Request;
use crate::stream::Stream;
use crate::uri::Uri;

#[derive(Clone, Copy)]
enum AuthMethod {
    NoAuth = 0,
    Plain = 2,
}

struct SocksAuth {
    method: AuthMethod,
    username: Vec<u8>,
    password: Vec<u8>,
}

impl SocksAuth {
    pub fn new_plain(username: &str, password: &str) -> Self {
        SocksAuth {
            method: AuthMethod::Plain,
            username: username.as_bytes().to_vec(),
            password: password.as_bytes().to_vec(),
        }
    }

    pub fn new() -> Self {
        SocksAuth {
            method: AuthMethod::NoAuth,
            username: Vec::new(),
            password: Vec::new(),
        }
    }
}

fn initial_greeting(socket: &mut TcpStream, auth: &SocksAuth) -> io::Result<()> {
    // The initial greeting from the client
    //      field 1: SOCKS version, 1 byte (0x05 for this version)
    //      field 2: number of authentication methods supported, 1 byte
    //      field 3: authentication methods, variable length, 1 byte per method supported
    socket.write_all(&[5u8, 1u8, auth.method as u8])
}

fn choise_communicated(socket: &mut TcpStream) -> Result<[u8; 2]> {
    // The server's choice is communicated:
    //      field 1: SOCKS version, 1 byte (0x05 for this version)
    //      field 2: chosen authentication method, 1 byte, or 0xFF if no acceptable methods were offered
    let mut buf = [0u8; 2];
    socket.read_exact(&mut buf)?;
    Ok(buf)
}

fn is_valid_socks_version(value: u8) -> Result<()> {
    match value {
        5u8 => Ok(()),
        _ => Err(Error::InvalidServerVersion),
    }
}

fn try_auth(socket: &mut TcpStream, value: u8, auth: &SocksAuth) -> Result<()> {
    if value == auth.method as u8 && value == 2u8 {
        // For username/password authentication the client's authentication request is
        //     field 1: version number, 1 byte (0x01 for current version of username/password authentication)
        let mut packet = vec![1u8];
        //     field 2: username length, 1 byte
        packet.push(auth.username.len() as u8);
        //     field 3: username, 1–255 bytes
        packet.append(&mut auth.username.clone());
        //     field 4: password length, 1 byte
        packet.push(auth.password.len() as u8);
        //     field 5: password, 1–255 bytes
        packet.append(&mut auth.password.clone());
        socket.write_all(&packet)?;
        let mut buf = [0u8; 2];
        socket.read_exact(&mut buf)?;
        // Server response for username/password authentication:
        //     field 1: version, 1 byte (0x01 for current version of username/password authentication)
        //     field 2: status code, 1 byte
        //         0x00: success
        //         any other value is a failure, connection must be closed
        match (buf[0] != 1u8, buf[1] != 0u8) {
            (true, _) => Err(Error::InvalidAuthVersion),
            (_, true) => Err(Error::AuthFailure),
            _ => Ok(()),
        }
    } else if value == auth.method as u8 {
        Ok(())
    } else {
        Err(Error::InvalidAuthMethod)
    }
}

fn request_connection(socket: &mut TcpStream, target: Vec<u8>) -> io::Result<()> {
    let mut packet = Vec::new();
    // The client's connection request is
    //     field 1: SOCKS version number, 1 byte (0x05 for this version)
    packet.push(5u8);
    //     field 2: command code, 1 byte:
    //         0x01: establish a TCP/IP stream connection
    //         0x02: establish a TCP/IP port binding
    //         0x03: associate a UDP port
    packet.push(1u8);
    //     field 3: reserved, must be 0x00, 1 byte
    packet.push(0u8);
    //     field 4: address type, 1 byte:
    //         0x01: IPv4 address
    //         0x03: Domain name
    //         0x04: IPv6 address
    //     field 5: destination address of
    //         4 bytes for IPv4 address
    //         1 byte of name length followed by 1–255 bytes the domain name
    //         16 bytes for IPv6 address
    //     field 6: port number in a network byte order, 2 bytes
    packet.append(&mut target.clone());
    socket.write_all(&packet)
}

fn get_server_reponse(socket: &mut TcpStream) -> Result<()> {
    let mut buf = [0u8; 3];
    socket.read_exact(&mut buf)?;
    // Server response:
    //     field 1: SOCKS protocol version, 1 byte (0x05 for this version)
    is_valid_socks_version(buf[0])?;
    //     field 2: status, 1 byte:
    //         0x00: request granted
    //         0x01: general failure
    //         0x02: connection not allowed by ruleset
    //         0x03: network unreachable
    //         0x04: host unreachable
    //         0x05: connection refused by destination host
    //         0x06: TTL expired
    //         0x07: command not supported / protocol error
    //         0x08: address type not supported
    match buf[1] {
        0 => Ok(()),
        1 => Err(Error::GeneralFailure),
        2 => Err(Error::InvalidRuleset),
        3 => Err(Error::NetworkUnreachable),
        4 => Err(Error::HostUnreachable),
        5 => Err(Error::RefusedByHost),
        6 => Err(Error::TtlExpired),
        7 => Err(Error::InvalidCommandProtocol),
        8 => Err(Error::InvalidAddressType),
        _ => Err(Error::UnknownError),
    }?;
    //     field 3: reserved, must be 0x00, 1 byte
    if buf[2] != 0u8 {
        Err(Error::InvalidReservedByte)
    } else {
        Ok(())
    }
}

fn get_host(socket: &mut TcpStream) -> Result<Addr> {
    let mut buf = [0u8; 1];
    //     field 4: address type, 1 byte:
    //         0x01: IPv4 address
    //         0x03: Domain name
    //         0x04: IPv6 address
    //     field 5: server bound address of
    //         4 bytes for IPv4 address
    //         1 byte of name length followed by 1–255 bytes the domain name
    //         16 bytes for IPv6 address
    socket.read_exact(&mut buf)?;
    match buf[0] {
        1 => {
            let mut buf = [0u8; 4];
            socket.read_exact(&mut buf)?;
            Ok(Addr::Ipv4(Ipv4Addr::from(buf)))
        }
        3 => {
            let mut len = [0u8; 1];
            socket.read_exact(&mut len)?;
            let mut buf = vec![0u8; len[0] as usize];
            socket.read_exact(&mut buf)?;
            Ok(Addr::Domain(String::from_utf8_lossy(&buf).into_owned()))
        }
        4 => {
            let mut buf = [0u8; 16];
            socket.read_exact(&mut buf)?;
            Ok(Addr::Ipv6(Ipv6Addr::from(buf)))
        }
        _ => Err(Error::InvalidAddressType),
    }
}

fn get_port(socket: &mut TcpStream) -> Result<[u8; 2]> {
    let mut bind_port = [0u8; 2];
    //     field 6: server bound port number in a network byte order, 2 bytes
    socket.read_exact(&mut bind_port)?;
    Ok(bind_port)
}

#[derive(Debug)]
pub struct SocksStream {
    stream: Stream,
    target: Addr,
}

impl SocksStream {
    pub fn connect(proxy: &Uri, target: &Uri) -> Result<SocksStream> {
        Self::handshake(proxy, target, &SocksAuth::new())
    }

    pub fn connect_plain(
        proxy: &Uri,
        target: &Uri,
        username: &str,
        password: &str,
    ) -> Result<SocksStream> {
        Self::handshake(proxy, target, &SocksAuth::new_plain(username, password))
    }

    fn handshake(proxy: &Uri, target: &Uri, auth: &SocksAuth) -> Result<SocksStream> {
        let proxy_addr = proxy.socket_addr()?;
        let mut socket = TcpStream::connect(proxy_addr)?;
        initial_greeting(&mut socket, auth)?;
        let buf = choise_communicated(&mut socket)?;
        is_valid_socks_version(buf[0])?;
        try_auth(&mut socket, buf[1], auth)?;
        request_connection(&mut socket, target.to_vec())?;
        get_server_reponse(&mut socket)?;
        let _host = get_host(&mut socket)?;
        let _port = get_port(&mut socket)?;
        let stream = if target.is_ssl() {
            Stream::new_tls(&target.host(), socket)?
        } else {
            Stream::new_tcp(socket)
        };

        Ok(SocksStream {
            stream,
            target: target.addr(),
        })
    }

    pub fn get_body(&mut self, req: &Request) -> Result<String> {
        self.stream.write_all(&req.msg())?;
        self.stream.flush()?;
        let mut response = vec![];
        self.stream.read_to_end(&mut response)?;
        Ok(String::from_utf8_lossy(&response).to_string())
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     lazy_static! {
//         static ref IP: String = crate::my_ip();
//     }

//     #[test]
//     fn socks() {
//         let mut client = SocksStream::connect("127.0.0.1:5959", "https://api.ipify.org").unwrap();
//         let body = client.get().unwrap();
//         let txt = String::from_utf8_lossy(&body);
//         assert!(txt.contains(crate::tests::IP.as_str()));
//     }

//     #[test]
//     fn socks_auth() {
//         let mut client =
//             SocksStream::connect_plain("127.0.0.1:5757", "https://api.ipify.org", "test", "tset")
//                 .unwrap();
//         let body = client.get().unwrap();
//         let txt = String::from_utf8_lossy(&body);
//         assert!(txt.contains(crate::tests::IP.as_str()));
//     }

//     #[test]
//     fn socks_bad_auth() {
//         let client =
//             SocksStream::connect_plain("127.0.0.1:5757", "https://api.ipify.org", "test", "test");
//         assert!(client.is_err());
//     }
// }
