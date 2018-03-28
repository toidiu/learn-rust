#![deny(warnings)]
extern crate futures;
extern crate hyper;
extern crate pretty_env_logger;
extern crate quick_xml;

use std::io::Read;
use hyper::Client;

fn main() {
    println!("=================================");
    let mut xml_resp = String::new();
    get_mta_status(&mut xml_resp);
    // parse_xml(&xml_resp);
}

fn get_mta_status(xml: &mut String) {
    let client = Client::new();
    client
        .get("http://web.mta.info/status/serviceStatus.txt")
        .send()
        .unwrap()
        .read_to_string(xml)
        .unwrap();
    println!("{}", xml);
}
