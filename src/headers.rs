use std::collections::{hash_map, HashMap};
use std::str::FromStr;
use std::fmt::{Formatter, Display};

use crate::uri::Uri;
use crate::error::{Error, Result};

#[derive(Debug, PartialEq, Clone, Default)]
pub struct Headers(HashMap<String, String>);

impl Headers {
    pub fn new() -> Headers {
        Headers(HashMap::new())
    }

    pub fn with_capacity(capacity: usize) -> Headers {
        Headers(HashMap::with_capacity(capacity))
    }

    pub fn iter(&self) -> hash_map::Iter<String, String> {
        self.0.iter()
    }

    pub fn get<T: ToString + ?Sized>(&self, k: &T) -> Option<&String> {
        self.0.get(&k.to_string())
    }

    pub fn insert<T, U>(&mut self, key: &T, val: &U) -> Option<String>
    where
        T: ToString + ?Sized,
        U: ToString + ?Sized,
    {
        self.0.insert(key.to_string(), val.to_string())
    }

    pub fn default_http(uri: &Uri) -> Headers {
        let mut headers = Headers::with_capacity(4);

        headers.insert("Host", &uri.host_header());
        // headers.insert("Referer", &uri);
        headers.insert("Connection", "Close");

        headers
    }
}

impl FromStr for Headers {
    type Err = Error;

    fn from_str(s: &str) -> Result<Headers> {
        let headers = s.trim();

        if headers.lines().all(|e| e.contains(':')) {
            let headers = headers
                .lines()
                .map(|elem| {
                    let idx = elem.find(':').unwrap();
                    let (key, value) = elem.split_at(idx);
                    (key.to_string(), value[1..].trim().to_string())
                })
                .collect();

            Ok(Headers(headers))
        } else {
            Err(Error::ParseHeaders)
        }
    }
}

impl From<HashMap<String, String>> for Headers {
    fn from(map: HashMap<String, String>) -> Headers {
        Headers(map)
    }
}

impl From<Headers> for HashMap<String, String> {
    fn from(map: Headers) -> HashMap<String, String> {
        map.0
    }
}

impl Display for Headers {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let headers: String = self
            .iter()
            .map(|(key, val)| format!("  {}: {}\r\n", key, val))
            .collect();

        write!(f, "{{\r\n{}}}", headers)
    }
}
