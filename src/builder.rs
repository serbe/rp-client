use bytes::Bytes;

// use crate::error::Error; // , Result};
use crate::headers::Headers;
use crate::method::Method;
use crate::uri::Uri;
use crate::version::Version; // IntoVersion

#[derive(Debug, Clone)]
pub struct RequestBuilder {
    pub uri: Uri,
    pub method: Method,
    pub version: Version,
    pub headers: Headers,
    pub body: Option<Bytes>,
    // pub err: Option<Error>,
}

impl RequestBuilder {
    pub fn new(uri: Uri) -> RequestBuilder {
        RequestBuilder {
            headers: Headers::default_http(&uri),
            uri,
            method: Method::GET,
            version: Version::Http11,
            body: None,
            // err: None,
        }
    }

    pub fn method<T>(&mut self, method: T) -> &mut Self
    where
        Method: From<T>,
    {
        self.method = Method::from(method);
        self
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

    pub fn body(&mut self, body: Bytes) -> &mut Self {
        self.body = Some(body);
        self
    }

    pub fn get(&mut self) -> &mut Self {
        self.method = Method::GET;
        self
    }

    pub fn post(&mut self) -> &mut Self {
        self.method = Method::POST;
        self
    }

    // pub fn uri(&mut self, uri: &str) -> &mut Self {
    //     match uri.parse::<Uri>() {
    //         Ok(uri) => self.uri = uri,
    //         Err(e) => self.err = Some(e),
    //     }
    //     self
    // }

    // pub fn version<T: IntoVersion>(&mut self, version: T) -> &mut Self {
    //     match version.into_version() {
    //         Ok(version) => self.version = version,
    //         Err(e) => self.err = Some(e),
    //     }
    //     self
    // }
}
