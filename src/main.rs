pub mod error;
pub mod uri;

fn main() {
    let u = "http://www.example.org/?foo=bar?#slkdjflkjsdf".parse::<uri::Uri>().unwrap();
    println!("http://www.example.org/?foo=bar?#slkdjflkjsdf");
    println!("scheme {}", u.scheme());
    println!("host {:?}", u.host());
    println!("path {:?}", u.path());
    println!("query {:?}", u.query());
    println!("fragment {:?}", u.fragment());
}