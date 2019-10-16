// use std::net::TcpStream;
// use std::io::{Write, Read};

// use crate::builder::{RequestBuilder};
use crate::error::{Result};
use crate::method::Method;
use crate::request::Request;
use crate::transport::Transport;
use crate::uri::{IntoUri, Uri};

#[derive(Debug)]
pub struct Client {
    request: Request,
    uri: Uri,
    proxy: Option<Uri>,
    transport: Transport,
}

impl Client {
    pub fn new<U: IntoUri>(uri: U) -> Result<Self> {
        let uri = uri.into_uri()?;
        Ok(Client {
            request: Request::new(&uri),
            uri,
            proxy: None,
            transport: Transport::new(),
        })
    }

    pub fn get(&mut self) -> &mut Self {
        self.request.method(Method::GET);
        self
    }

    pub fn post(&mut self) -> &mut Self {
        self.request.method(Method::POST);
        self
    }

    pub fn put(&mut self) -> &mut Self {
        self.request.method(Method::PUT);
        self
    }

    pub fn patch(&mut self) -> &mut Self {
        self.request.method(Method::PATCH);
        self
    }

    pub fn delete(&mut self) -> &mut Self {
        self.request.method(Method::DELETE);
        self
    }

    pub fn head(&mut self) -> &mut Self {
        self.request.method(Method::HEAD);
        self
    }

    pub fn proxy<U: IntoUri>(&mut self, uri: U) -> Result<&mut Self> {
        self.proxy = Some(uri.into_uri()?.check_supported_proxy()?);
        Ok(self)
    }

    // pub fn http_proxy<U: IntoUri>(&mut self, uri: U) -> Result<()> {
    //     self.transport = Transport::proxy_http(&uri.into_uri()?)?;
    //     Ok(())
    // }

    // pub fn htts_proxy<U: IntoUri>(&mut self, uri: U) -> Result<()> {
    //     self.transport = Transport::proxy_https(&uri.into_uri()?)?;
    //     Ok(())
    // }

    // pub fn socks_proxy<U: IntoUri>(&mut self, uri: U) -> Result<()> {
    //     self.transport = Transport::proxy_socks(&uri.into_uri()?)?;
    //     Ok(())
    // }

    pub fn connect(&mut self) -> Result<&mut Self> {
        if let Some(ref proxy) = self.proxy {
            self.transport = Transport::proxy(&self.uri, proxy)?;
        } else {
            self.transport = Transport::stream(&self.uri)?;
        }
        Ok(self)
    }

    // pub fn build(&mut self) -> Result<String> {
    //     self.connect()?;
    //     println!("transport: {:?}", self.transport);
    //     let body = match self.transport {
    //         Transport::Proxy(_) => Err(Error::WrongHttp),
    //         Transport::Stream(ref mut stream) => {
    //             stream.write_all(&self.request.msg())?;
    //             println!("msg: {:?}", String::from_utf8_lossy(&self.request.msg()));
    //             stream.flush()?;
    //             let mut response = vec![];
    //             stream.read_to_end(&mut response)?;
    //             println!("response: {:?}", String::from_utf8_lossy(&response));
    //             let pos = response
    //                 .windows(4)
    //                 .position(|x| x == b"\r\n\r\n")
    //                 .ok_or_else(|| Error::WrongHttp)?;
    //             let body = &response[pos + 4..response.len()];
    //             let s = String::from_utf8_lossy(body);
    //             Ok(s.to_string())
    //         },
    //         Transport::None => Err(Error::WrongHttp),
    //     };
    //     body
    // }
}

// pub fn request<U: IntoUri>(&self, method: Method) -> RequestBuilder {
//     let req = url
//         .into_uri()
//         .map(move |url| Request::new(method, url));
//     RequestBuilder::new(self.clone(), req)
// }

// pub fn execute(&self, request: Request) -> ::Result<Response> {
//     self.inner.execute_request(request)
// }
