use crate::http::HttpStream;
use crate::uri::Uri;

pub mod addr;
pub mod authority;
pub mod client;
pub mod error;
pub mod headers;
pub mod http;
pub mod method;
pub mod proxy;
pub mod range;
pub mod request;
pub mod response;
pub mod socks;
pub mod status;
pub mod stream;
pub mod transport;
pub mod uri;
pub mod userinfo;
pub mod version;

fn main() {
    let mut client = HttpStream::connect(&"http://api.ipify.org".parse::<Uri>().unwrap()).unwrap();
    client.send_request(b"GET / HTTP/1.0\r\nHost: api.ipify.org\r\n\r\n").unwrap();
    let response = client.get_response().unwrap();
    println!("response: {:?}", response);
    println!("content_len: {:?}", response.content_len());
    let body = client.get_body(response.content_len().unwrap()).unwrap();
    let body = String::from_utf8(body).unwrap();
    println!("body: {:?}", body);
}
