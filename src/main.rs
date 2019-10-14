use crate::client::Client;

pub mod addr;
pub mod authority;
pub mod builder;
pub mod client;
pub mod error;
pub mod headers;
pub mod method;
pub mod proxy;
pub mod range;
pub mod request;
pub mod transport;
pub mod uri;
pub mod userinfo;
pub mod version;

fn main() {
    let mut client = Client::new("http://ya.ru").unwrap();
    client.proxy("socks5h://127.0.0.1:3128");
    println!("{:?}", client);
}