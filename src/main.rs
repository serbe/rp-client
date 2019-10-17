use crate::client::Client;

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
pub mod socks;
pub mod stream;
pub mod transport;
pub mod uri;
pub mod userinfo;
pub mod version;

fn main() {
    let mut client = Client::new("http://api.ipify.org").unwrap();
    client.proxy("http://127.0.0.1:5858").unwrap();
    client.connect().unwrap();
    println!("client: {:?}", client);
    let body = client.body();
    println!("body: {:?}", body);
    println!("----------------------------------------------");
    let mut client = Client::new("http://api.ipify.org").unwrap();
    client.proxy("socks5://127.0.0.1:5959").unwrap();
    client.connect().unwrap();
    println!("client: {:?}", client);
    let body = client.body();
    println!("body: {:?}", body);
    println!("----------------------------------------------");
    let mut client = Client::new("http://api.ipify.org").unwrap();
    client.connect().unwrap();
    println!("client: {:?}", client);
    let body = client.body();
    println!("body: {:?}", body);
}
