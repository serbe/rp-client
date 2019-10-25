// use std::time::Duration;

use crate::client::Client;
use crate::error::{Error, Result};
use crate::headers::Headers;
use crate::method::{IntoMethod, Method};
use crate::request::Request;
use crate::transport::Transport;
use crate::uri::{IntoUri, Uri};
use crate::version::{IntoVersion, Version};

pub struct ClientBuilder {
    uri: Option<Uri>,
    headers: Headers,
    method: Method,
    version: Version,
    body: Option<Vec<u8>>,
    referer: bool,
    proxy: Option<Uri>,
    nodelay: bool,
    // timeout: Option<Duration>,
    // connect_timeout: Option<Duration>,
}

impl ClientBuilder {
    pub fn new() -> Self {
        let headers = Headers::new();
        ClientBuilder {
            uri: None,
            headers,
            method: Method::GET,
            version: Version::Http11,
            body: None,
            referer: true,
            proxy: None,
            nodelay: false,
            // timeout: None,
            // connect_timeout: None,
        }
    }

    pub fn build(self) -> Result<Client> {
        let uri = self.uri.ok_or(Error::EmptyUri)?;
        let transport = if let Some(proxy) = &self.proxy {
            Transport::proxy(&uri, &proxy)?
        } else {
            Transport::stream(&uri)?
        };
        let mut request = Request::new(&uri, self.proxy.is_some());
        request.method(self.method);
        request.headers(self.headers);
        request.version(self.version);
        request.body(self.body);

        Ok(Client::from(request, uri, transport, None))
    }

    pub fn uri<T: IntoUri>(mut self, uri: T) -> ClientBuilder {
        match uri.into_uri() {
            Ok(uri) => self.uri = Some(uri),
            _ => self.uri = None,
        }
        self
    }

    pub fn proxy<T: IntoUri>(mut self, proxy: T) -> ClientBuilder {
        match proxy.into_uri() {
            Ok(uri) => self.proxy = Some(uri),
            _ => self.proxy = None,
        }
        self
    }

    pub fn headers(mut self, headers: Headers) -> ClientBuilder {
        for (key, value) in headers.iter() {
            self.headers.insert(key, &value);
        }
        self
    }

    pub fn header<T: ToString + ?Sized, U: ToString + ?Sized>(
        mut self,
        key: &T,
        value: &U,
    ) -> ClientBuilder {
        self.headers.insert(key, value);
        self
    }

    pub fn method<T: IntoMethod>(mut self, method: T) -> ClientBuilder {
        if let Ok(method) = method.into_method() {
            self.method = method;
        }
        self
    }

    pub fn get<T: IntoUri>(mut self, uri: T) -> ClientBuilder {
        if let Ok(uri) = uri.into_uri() {
            self.uri = Some(uri);
        }
        self.method = Method::GET;
        self
    }

    pub fn post<T: IntoUri>(mut self, uri: T) -> ClientBuilder {
        if let Ok(uri) = uri.into_uri() {
            self.uri = Some(uri);
        }
        self.method = Method::POST;
        self
    }

    pub fn put<T: IntoUri>(mut self, uri: T) -> ClientBuilder {
        if let Ok(uri) = uri.into_uri() {
            self.uri = Some(uri);
        }
        self.method = Method::PUT;
        self
    }

    pub fn patch<T: IntoUri>(mut self, uri: T) -> ClientBuilder {
        if let Ok(uri) = uri.into_uri() {
            self.uri = Some(uri);
        }
        self.method = Method::PATCH;
        self
    }

    pub fn delete<T: IntoUri>(mut self, uri: T) -> ClientBuilder {
        if let Ok(uri) = uri.into_uri() {
            self.uri = Some(uri);
        }
        self.method = Method::DELETE;
        self
    }

    pub fn head<T: IntoUri>(mut self, uri: T) -> ClientBuilder {
        if let Ok(uri) = uri.into_uri() {
            self.uri = Some(uri);
        }
        self.method = Method::HEAD;
        self
    }

    pub fn version<T: IntoVersion>(mut self, version: T) -> ClientBuilder {
        if let Ok(version) = version.into_version() {
            self.version = version;
        }
        self
    }

    pub fn body(mut self, body: &[u8]) -> ClientBuilder {
        self.body = Some(body.to_vec());
        self
    }

    pub fn tcp_nodelay(mut self) -> ClientBuilder {
        self.nodelay = true;
        self
    }

    pub fn referer(mut self, enable: bool) -> ClientBuilder {
        self.referer = enable;
        self
    }

    // pub fn timeout(mut self, timeout: Duration) -> ClientBuilder {
    //     self.timeout = Some(timeout);
    //     self
    // }

    // pub fn connect_timeout(mut self, timeout: Duration) -> ClientBuilder {
    //     self.connect_timeout = Some(timeout);
    //     self
    // }
}
