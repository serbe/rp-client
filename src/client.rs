use crate::error::{Error, Result};
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

    pub fn header<T, U>(&mut self, key: &T, val: &U) -> &mut Self
    where
        T: ToString + ?Sized,
        U: ToString + ?Sized,
    {
        self.request.header(key, val);
        self
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

    pub fn body(&mut self) -> Result<String> {
        self.connect()?;
        match self.transport {
            Transport::Proxy(ref mut proxy) => proxy.get_body(&self.request),
            Transport::Stream(ref mut stream) => stream.get_body(&self.request),
            Transport::None => Err(Error::WrongHttp),
        }
    }
}
