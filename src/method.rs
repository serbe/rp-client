// use bytes::Bytes;
use std::str::FromStr;

use crate::error::{Error, Result};

#[derive(Debug, Clone)]
pub enum Method {
    OPTIONS,
    GET,
    POST,
    PUT,
    DELETE,
    HEAD,
    TRACE,
    CONNECT,
    PATCH,
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

// impl<'a> IntoMethod for Bytes {
//     fn into_method(self) -> Result<Method> {
//         Method::from_bytes(self)
//     }
// }

impl Method {
    pub fn as_str(&self) -> &str {
        match self {
            Method::OPTIONS => "OPTIONS",
            Method::GET => "GET",
            Method::POST => "POST",
            Method::PUT => "PUT",
            Method::DELETE => "DELETE",
            Method::HEAD => "HEAD",
            Method::TRACE => "TRACE",
            Method::CONNECT => "CONNECT",
            Method::PATCH => "PATCH",
        }
    }

    // pub fn from_bytes(b: Bytes) -> Result<Self> {
    //     match &b[..] {
    //         b"OPTIONS" => Ok(Method::OPTIONS),
    //         b"GET" => Ok(Method::GET),
    //         b"POST" => Ok(Method::POST),
    //         b"PUT" => Ok(Method::PUT),
    //         b"DELETE" => Ok(Method::DELETE),
    //         b"HEAD" => Ok(Method::HEAD),
    //         b"TRACE" => Ok(Method::TRACE),
    //         b"CONNECT" => Ok(Method::CONNECT),
    //         b"PATCH" => Ok(Method::PATCH),
    //         _ => Err(Error::UnknownMethod(
    //             String::from_utf8_lossy(&b[..]).to_string(),
    //         )),
    //     }
    // }
}

impl Default for Method {
    fn default() -> Self {
        Method::GET
    }
}

impl FromStr for Method {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s.to_ascii_uppercase().as_str() {
            "OPTIONS" => Ok(Method::OPTIONS),
            "GET" => Ok(Method::GET),
            "POST" => Ok(Method::POST),
            "PUT" => Ok(Method::PUT),
            "DELETE" => Ok(Method::DELETE),
            "HEAD" => Ok(Method::HEAD),
            "TRACE" => Ok(Method::TRACE),
            "CONNECT" => Ok(Method::CONNECT),
            "PATCH" => Ok(Method::PATCH),
            _ => Err(Error::UnknownMethod(s.to_owned())),
        }
    }
}
