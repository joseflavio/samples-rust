extern crate hyper;

use std::io::Read;
use hyper::Client;

fn main() {
    let client = Client::new();
    let mut res = client.get("http://www.addic7ed.com/").send().unwrap();
    assert_eq!(res.status, hyper::Ok);
    println!("< status code: {}", res.status);
    let mut s = String::new();
    res.read_to_string(&mut s).unwrap();
    println!("{}", s);
}
