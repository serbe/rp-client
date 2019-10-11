use std::collections::HashMap;

use bytes::Bytes;
use url::Url;

use crate::error::Error; // , Result};
use crate::method::{IntoMethod, Method};
use crate::version::{IntoVersion, Version};

#[derive(Debug)]
pub struct Builder<'a> {
    pub url: Url,
    pub method: Method,
    pub version: Version,
    pub headers: HashMap<Bytes, Bytes>,
    pub body: Option<&'a [u8]>,
    pub err: Option<Error>,
}

impl<'a> Builder<'a> {
    pub fn new(url: Url) -> Builder<'a> {
        Builder {
            url,
            headers: HashMap::new(),
            method: Method::Get,
            version: Version::Http11,
            body: None,
            err: None,
        }
    }

    pub fn method<T>(&mut self, method: T) -> &mut Self
    where
        Method: From<T>,
    {
        self.method = Method::from(method);
        self
    }

    // pub fn headers<T>(&mut self, headers: T) -> &mut Self
    // where
    //     Headers: From<T>,
    // {
    //     self.headers = Headers::from(headers);
    //     self
    // }

    pub fn header<T, U>(&mut self, key: Bytes, val: Bytes) -> &mut Self {
        self.headers.insert(key, val);
        self
    }

    pub fn body(&mut self, body: &'a [u8]) -> &mut Self {
        self.body = Some(body);
        self
    }

    pub fn get(&mut self) -> &mut Self {
        self.method = Method::Get;
        self
    }

    pub fn post(&mut self) -> &mut Self {
        self.method = Method::Post;
        self
    }

    pub fn uri(&mut self, url: &str) -> &mut Self {
        match url.parse::<Url>() {
            Ok(url) => self.url = url,
            Err(e) => self.err = Some(e.into()),
        }
        self
    }

    pub fn version<T: IntoVersion>(&mut self, version: T) -> &mut Self {
        match version.into_version() {
            Ok(version) => self.version = version,
            Err(e) => self.err = Some(e),
        }
        self
    }
}
