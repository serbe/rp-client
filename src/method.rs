use bytes::Bytes;
use std::str::FromStr;

use crate::error::{Error, Result};

#[derive(Debug)]
pub enum Method {
    Options,
    Get,
    Post,
    Put,
    Delete,
    Head,
    Trace,
    Connect,
    Patch,
}

pub trait IntoMethod {
    fn into_method(self) -> Result<Method>;
}

impl IntoMethod for Method {
    fn into_method(self) -> Result<Method> {
        Ok(self)
    }
}

impl<'a> IntoMethod for &'a str {
    fn into_method(self) -> Result<Method> {
        self.parse()
    }
}

impl IntoMethod for String {
    fn into_method(self) -> Result<Method> {
        self.parse()
    }
}

impl<'a> IntoMethod for Bytes {
    fn into_method(self) -> Result<Method> {
        Method::from_bytes(self)
    }
}

impl Method {
    pub fn as_str(&self) -> &str {
        match self {
            Method::Options => "OPTIONS",
            Method::Get => "GET",
            Method::Post => "POST",
            Method::Put => "PUT",
            Method::Delete => "DELETE",
            Method::Head => "HEAD",
            Method::Trace => "TRACE",
            Method::Connect => "CONNECT",
            Method::Patch => "PATCH",
        }
    }

    pub fn from_bytes(b: Bytes) -> Result<Self> {
        match &b[..] {
            b"OPTIONS" => Ok(Method::Options),
            b"GET" => Ok(Method::Get),
            b"POST" => Ok(Method::Post),
            b"PUT" => Ok(Method::Put),
            b"DELETE" => Ok(Method::Delete),
            b"HEAD" => Ok(Method::Head),
            b"TRACE" => Ok(Method::Trace),
            b"CONNECT" => Ok(Method::Connect),
            b"PATCH" => Ok(Method::Patch),
            _ => Err(Error::UnknownMethod(
                String::from_utf8_lossy(&b[..]).to_string(),
            )),
        }
    }
}

impl Default for Method {
    fn default() -> Self {
        Method::Get
    }
}

impl FromStr for Method {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s.to_ascii_uppercase().as_str() {
            "OPTIONS" => Ok(Method::Options),
            "GET" => Ok(Method::Get),
            "POST" => Ok(Method::Post),
            "PUT" => Ok(Method::Put),
            "DELETE" => Ok(Method::Delete),
            "HEAD" => Ok(Method::Head),
            "TRACE" => Ok(Method::Trace),
            "CONNECT" => Ok(Method::Connect),
            "PATCH" => Ok(Method::Patch),
            _ => Err(Error::UnknownMethod(s.to_owned())),
        }
    }
}
