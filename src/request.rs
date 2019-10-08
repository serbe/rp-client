use std::collections::HashMap;

use bytes::Bytes;

use crate::error::Error; // , Result};
use crate::method::{IntoMethod, Method};
use crate::uri::{IntoUri, Uri};
use crate::version::{IntoVersion, Version};

#[derive(Debug, Default)]
pub struct Builder {
    pub method: Method,
    pub uri: Uri,
    pub version: Version,
    pub headers: HashMap<Bytes, Bytes>,
    pub body: Option<Bytes>,
    pub err: Option<Error>,
}

impl Builder {
    pub fn new() -> Builder {
        Builder::default()
    }

    pub fn method<T: IntoMethod>(&mut self, method: T) -> &mut Builder {
        match method.into_method() {
            Ok(method) => self.method = method,
            Err(e) => self.err = Some(e),
        }
        self
    }

    pub fn get(&mut self) -> &mut Builder {
        self.method = Method::Get;
        self
    }

    pub fn post(&mut self) -> &mut Builder {
        self.method = Method::Post;
        self
    }

    pub fn uri<T: IntoUri>(&mut self, uri: T) -> &mut Builder {
        match uri.into_uri() {
            Ok(uri) => self.uri = uri,
            Err(e) => self.err = Some(e),
        }
        self
    }

    pub fn version<T: IntoVersion>(&mut self, version: T) -> &mut Builder {
        match version.into_version() {
            Ok(version) => self.version = version,
            Err(e) => self.err = Some(e),
        }
        self
    }

    pub fn header(&mut self, key: Bytes, value: Bytes) -> &mut Builder {
        self.headers.insert(key, value);
        self
    }

    pub fn body(&mut self, body: Bytes) -> &mut Builder {
        self.body = Some(body);
        self
    }
}
