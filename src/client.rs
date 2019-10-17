use crate::error::{Error, Result};
use crate::method::Method;
use crate::request::Request;
use crate::response::Response;
use crate::transport::Transport;
use crate::uri::{IntoUri, Uri};

#[derive(Debug)]
pub struct Client {
    request: Request,
    uri: Uri,
    proxy: Option<Uri>,
    transport: Transport,
    response: Option<Response>,
}

impl Client {
    pub fn new<U: IntoUri>(uri: U) -> Result<Self> {
        let uri = uri.into_uri()?;
        Ok(Client {
            request: Request::new(&uri),
            uri,
            proxy: None,
            transport: Transport::new(),
            response: None,
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

    pub fn body(&mut self, body: &[u8]) -> &mut Self {
        self.request.body(body.to_vec());
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

    pub fn send_request(&mut self) -> Result<()> {
        match self.transport {
            Transport::Proxy(ref mut proxy) => proxy.send_request(&self.request),
            Transport::Stream(ref mut stream) => stream.send_request(&self.request),
            Transport::None => return Err(Error::WrongHttp),
        }
    }

    pub fn send(&mut self) -> Result<Response> {
        self.connect()?;
        self.send_request()?;
        let response = match self.transport {
            Transport::Proxy(ref mut proxy) => proxy.get_response(),
            Transport::Stream(ref mut stream) => stream.get_response(),
            Transport::None => return Err(Error::WrongHttp),
        }?;
        self.response = Some(response.clone());
        Ok(response)
    }

    fn content_len(&self) -> Result<usize> {
        if let Some(response) = &self.response {
            response.content_len()
        } else {
            Err(Error::EmptyResponse)
        }
    }

    pub fn get_body(&mut self) -> Result<Vec<u8>> {
        let content_len = self.content_len()?;
        match self.transport {
            Transport::Proxy(ref mut proxy) => proxy.get_body(content_len),
            Transport::Stream(ref mut stream) => stream.get_body(content_len),
            Transport::None => return Err(Error::WrongHttp),
        }
    }

    pub fn text(&mut self) -> Result<String> {
        let body = self.get_body()?;
        Ok(String::from_utf8_lossy(&body).to_string())
    }
}
