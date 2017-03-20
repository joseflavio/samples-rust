extern crate hyper;

use hyper::Client;

fn main() {
    let client = Client::new();
    let res = client.get("http://www.addic7ed.com/").send().unwrap();
    assert_eq!(res.status, hyper::Ok);
    println!("< status code: {}", res.status);
}