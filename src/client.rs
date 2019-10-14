// use crate::builder::{RequestBuilder};
use crate::uri::{IntoUri};
use crate::request::Request;
use crate::transport::Transport;
use crate::method::Method;
use crate::error::Result;

// #[derive(Clone)]
pub struct Client {
    // uri: Uri,
    transport: Transport,
    request: Request,
}

impl Client {
    pub fn new<U: IntoUri>(uri: U) -> Self {
        Client{
            transport: Transport::new(),
            request: Request::new(uri.into_uri().expect("bad uri")),
        }
    }

    pub fn get(&mut self) -> &mut Self {
        &self.request.method(Method::GET);
        self
    }

    pub fn post(&mut self) -> &mut Self {
        &self.request.method(Method::POST);
        self
    }

    pub fn put(&mut self) -> &mut Self {
        &self.request.method(Method::PUT);
        self
    }

    pub fn patch(&mut self) -> &mut Self {
        &self.request.method(Method::PATCH);
        self
    }

    pub fn delete(&mut self) -> &mut Self {
        &self.request.method(Method::DELETE);
        self
    }

    pub fn head(&mut self) -> &mut Self {
        &self.request.method(Method::HEAD);
        self
    }

    pub fn proxy<U: IntoUri>(&mut self, uri: U) -> Result<&mut Self> {
        self.transport = Transport::proxy(uri.into_uri()?)?;
        Ok(self)
    }

    pub fn http_proxy<U: IntoUri>(&mut self, uri: U) -> Result<&mut Self> {
        self.transport = Transport::proxy_http(uri.into_uri()?)?;
        Ok(self)
    }

    pub fn htts_proxy<U: IntoUri>(&mut self, uri: U) -> Result<&mut Self> {
        self.transport = Transport::proxy_https(uri.into_uri()?)?;
        Ok(self)
    }

    pub fn socks_proxy<U: IntoUri>(&mut self, uri: U) -> Result<&mut Self> {
        self.transport = Transport::proxy_socks(uri.into_uri()?)?;
        Ok(self)
    }
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
