use std::net::{TcpStream, ToSocketAddrs};
use std::time::Duration;

use crate::headers::Headers;
use crate::method::Method;
use crate::uri::Uri;
use crate::version::Version;

#[derive(Clone, Debug)]
pub struct Request {
    resource: String,
    headers: Headers,
    method: Method,
    version: Version,
    body: Option<Vec<u8>>,
    // connect_timeout: Option<Duration>,
    // read_timeout: Option<Duration>,
    // write_timeout: Option<Duration>,
    // root_cert_file_pem: Option<&'a Path>,
}

impl Request {
    pub fn new(uri: &Uri) -> Request {
        Request {
            headers: Headers::default_http(&uri.host_header()),
            resource: uri.resource().to_string(),
            method: Method::GET,
            version: Version::Http11,
            body: None,
        }
    }

    pub fn headers<T>(&mut self, headers: T) -> &mut Self
    where
        Headers: From<T>,
    {
        self.headers = Headers::from(headers);
        self
    }

    pub fn header<T, U>(&mut self, key: &T, val: &U) -> &mut Self
    where
        T: ToString + ?Sized,
        U: ToString + ?Sized,
    {
        self.headers.insert(key, val);
        self
    }

    pub fn method<T>(&mut self, method: T) -> &mut Self
    where
        Method: From<T>,
    {
        self.method = Method::from(method);
        self
    }

    pub fn body(&mut self, body: Vec<u8>) -> &mut Self {
        self.body = Some(body);
        self
    }

    // pub fn connect_timeout<T>(&mut self, timeout: Option<T>) -> &mut Self
    // where
    //     Duration: From<T>,
    // {
    //     self.connect_timeout = timeout.map(Duration::from);
    //     self
    // }

    // pub fn read_timeout<T>(&mut self, timeout: Option<T>) -> &mut Self
    // where
    //     Duration: From<T>,
    // {
    //     self.read_timeout = timeout.map(Duration::from);
    //     self
    // }

    // pub fn write_timeout<T>(&mut self, timeout: Option<T>) -> &mut Self
    // where
    //     Duration: From<T>,
    // {
    //     self.write_timeout = timeout.map(Duration::from);
    //     self
    // }

    pub fn msg(&self) -> Vec<u8> {
        let request_line = format!(
            "{} {} {}{}",
            self.method, self.resource, self.version, "\r\n"
        );

        let headers: String = self
            .headers
            .iter()
            .map(|(k, v)| format!("{}: {}{}", k, v, "\r\n"))
            .collect();

        let mut request_msg = (request_line + &headers + "\r\n").as_bytes().to_vec();

        if let Some(b) = &self.body {
            request_msg.extend(b);
        }

        request_msg
    }
}

pub fn connect_timeout<T, U>(host: T, port: u16, timeout: U) -> std::io::Result<TcpStream>
where
    Duration: From<U>,
    T: AsRef<str>,
{
    let host = host.as_ref();
    let timeout = Duration::from(timeout);
    let addrs: Vec<_> = (host, port).to_socket_addrs()?.collect();
    let count = addrs.len();

    for (idx, addr) in addrs.into_iter().enumerate() {
        match TcpStream::connect_timeout(&addr, timeout) {
            Ok(stream) => return Ok(stream),
            Err(err) => match err.kind() {
                std::io::ErrorKind::TimedOut => return Err(err),
                _ => {
                    if idx + 1 == count {
                        return Err(err);
                    }
                }
            },
        };
    }

    Err(std::io::Error::new(
        std::io::ErrorKind::AddrNotAvailable,
        format!("Could not resolve address for {:?}", host),
    ))
}
