use crate::headers::Headers;
use crate::method::Method;
use crate::uri::Uri;
use crate::version::Version;

use base64::encode;

#[derive(Clone, Debug)]
pub struct Request {
    method: Method,
    request_uri: String,
    version: Version,
    headers: Headers,
    host: String,
    content_len: usize,
    body: Option<Vec<u8>>,
    using_proxy: bool,
}

impl Request {
    pub fn new(uri: &Uri, using_proxy: bool) -> Request {
//         let request_uri = if using_proxy {
//             uri.request_uri().to_string()
//         } else {
//             uri.proxy_request_uri()
//         };
        let request_uri = if using_proxy {
            uri.proxy_request_uri()
        } else {
            uri.request_uri().to_string()
        };
        Request {
            method: Method::GET,
            request_uri,
            version: Version::Http11,
            headers: Headers::default_http(&uri.host_header()),
            host: uri.host_port(),
            content_len: 0,
            body: None,
            using_proxy,
        }
    }

    pub fn user_agent(&self) -> Option<String> {
        self.headers.get("User-Agent")
    }

    pub fn refferer(&self) -> Option<String> {
        self.headers.get("Referer")
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

    pub fn set_basic_auth(&mut self, username: &str, password: &str) -> &mut Self {
        self.header(
            "Authorization",
            &format!("Basic {}", encode(&format!("{}:{}", username, password))),
        );
        self
    }

    pub fn msg(&self) -> Vec<u8> {
        let request_line = format!(
            "{} {} {}{}",
            self.method, self.request_uri, self.version, "\r\n"
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
