use std::net::{TcpStream, ToSocketAddrs};
// use std::io::Write;
use std::time::Duration;

use bytes::Bytes;

use crate::builder::RequestBuilder;
use crate::headers::Headers;
use crate::method::Method;
use crate::uri::{IntoUri, Uri};

#[derive(Clone, Debug)]
pub struct Request {
    inner: RequestBuilder,
    connect_timeout: Option<Duration>,
    read_timeout: Option<Duration>,
    write_timeout: Option<Duration>,
    // root_cert_file_pem: Option<&'a Path>,
}

impl Request {
    pub fn new(uri: Uri) -> Request {
        let mut builder = RequestBuilder::new(uri);
        builder.header("Connection", "Close");

        Request {
            inner: builder,
            connect_timeout: Some(Duration::from_secs(60)),
            read_timeout: Some(Duration::from_secs(60)),
            write_timeout: Some(Duration::from_secs(60)),
            // root_cert_file_pem: None,
        }
    }

    pub fn headers<T>(&mut self, headers: T) -> &mut Self
    where
        Headers: From<T>,
    {
        self.inner.headers(headers);

        self
    }

    pub fn header<T, U>(&mut self, key: &T, val: &U) -> &mut Self
    where
        T: ToString + ?Sized,
        U: ToString + ?Sized,
    {
        self.inner.header(key, val);
        self
    }

    pub fn method<T>(&mut self, method: T) -> &mut Self
    where
        Method: From<T>,
    {
        self.inner.method(method);
        self
    }

    pub fn body(&mut self, body: Bytes) -> &mut Self {
        self.inner.body(body);

        self
    }

    pub fn connect_timeout<T>(&mut self, timeout: Option<T>) -> &mut Self
    where
        Duration: From<T>,
    {
        self.connect_timeout = timeout.map(Duration::from);
        self
    }

    pub fn read_timeout<T>(&mut self, timeout: Option<T>) -> &mut Self
    where
        Duration: From<T>,
    {
        self.read_timeout = timeout.map(Duration::from);
        self
    }

    pub fn write_timeout<T>(&mut self, timeout: Option<T>) -> &mut Self
    where
        Duration: From<T>,
    {
        self.write_timeout = timeout.map(Duration::from);
        self
    }

    // pub fn root_cert_file_pem(&mut self, file_path: &'a Path) -> &mut Self {
    //     self.root_cert_file_pem = Some(file_path);
    //     self
    // }

    // pub fn send<T: Write>(&self, writer: &mut T) -> Result<Response> {
    //     let host = self.inner.uri.host();
    //     let port = self.inner.uri.default_port();
    //     let mut stream = match self.connect_timeout {
    //         Some(timeout) => connect_timeout(host, port, timeout)?,
    //         None => TcpStream::connect((host, port))?,
    //     };

    //     stream.set_read_timeout(self.read_timeout)?;
    //     stream.set_write_timeout(self.write_timeout)?;

    //     // if self.inner.uri.scheme() == "https" {
    //     //     let mut cnf = tls::Config::default();
    //     //     let cnf = match self.root_cert_file_pem {
    //     //         Some(p) => cnf.add_root_cert_file_pem(p)?,
    //     //         None => &mut cnf,
    //     //     };
    //     //     let mut stream = cnf.connect(host, stream)?;
    //     //     self.inner.send(&mut stream, writer)
    //     // } else {
    //         self.inner.send(&mut stream, writer)
    //     // }
    // }
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

// pub fn get<T: AsRef<str>, U: Write>(uri: T, writer: &mut U) -> Result<Response, error::Error> {
//     let uri = uri.as_ref().parse::<Uri>()?;

//     Request::new(&uri).send(writer)
// }

// pub fn head<T: AsRef<str>>(uri: T) -> Result<Response, error::Error> {
//     let mut writer = Vec::new();
//     let uri = uri.as_ref().parse::<Uri>()?;

//     Request::new(&uri).method(Method::HEAD).send(&mut writer)
// }
