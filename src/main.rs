// use crate::client::Client;

pub mod addr;
pub mod authority;
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
// pub mod http;
pub mod stream;

fn main() {
    // let mut client = Client::new("http://api.ipify.org").unwrap();
    // // client.proxy("http://127.0.0.1:5858").unwrap();
    // let body = client.build();
    // println!("client: {:?}", client);
    // println!("body: {:?}", body);
}