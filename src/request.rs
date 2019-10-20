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

    pub fn headers(&mut self, headers: Headers) -> &mut Self {
        for (key, value) in headers.iter() {
            self.headers.insert(key, &value);
        }
        self
    }

    pub fn header<T: ToString + ?Sized, U: ToString + ?Sized>(
        &mut self,
        key: &T,
        val: &U,
    ) -> &mut Self {
        self.headers.insert(key, val);
        self
    }

    pub fn method(&mut self, method: Method) -> &mut Self {
        self.method = method;
        self
    }

    pub fn version(&mut self, version: Version) -> &mut Self {
        self.version = version;
        self
    }

    pub fn body(&mut self, body: Option<Vec<u8>>) -> &mut Self {
        self.body = body;
        self
    }

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
