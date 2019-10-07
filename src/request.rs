use std::collections::HashMap;

use bytes::Bytes;

use crate::method::Method;
// use crate::error::{Error, Result};
use crate::version::Version;
use crate::url::Url;

#[derive(Debug, Default)]
pub struct Builder {
    pub method: Method,
    pub url: Url,
    pub version: Version,
    pub headers: HashMap<Bytes, Bytes>,
}

impl Builder {
    pub fn new() -> Builder {
        Builder::default()
    }
}