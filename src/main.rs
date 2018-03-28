#![deny(warnings)]
extern crate futures;
extern crate hyper;
extern crate pretty_env_logger;
extern crate quick_xml;

use std::io::Read;
use quick_xml::reader::Reader;
use quick_xml::events::Event;
use hyper::Client;

fn main() {
    println!("=================================");
    let mut xml_resp = String::new();
    get_mta_status(&mut xml_resp);
    parse_xml(&xml_resp);

    fn get_mta_status(xml: &mut String) {
        let client = Client::new();
        client
            .get("http://web.mta.info/status/serviceStatus.txt")
            .send()
            .unwrap()
            .read_to_string(xml)
            .unwrap();
        println!("hi");
    }

    fn parse_xml(xml: &str) {
        let mut reader = Reader::from_str(&xml);
        reader.trim_text(true);

        let mut buf = Vec::new();

        // The `Reader` does not implement `Iterator` because it outputs borrowed data (`Cow`xml_resp)
        loop {
            match reader.read_event(&mut buf) {
                Ok(Event::Start(ref e)) => {
                    println!("{:?}", e.unescape_and_decode(&reader).unwrap());
                }
                Ok(Event::Text(e)) => {
                    println!("{}", e.unescape_and_decode(&reader).unwrap());
                }
                Ok(Event::Eof) => break, // exits the loop when reaching end of file
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (), // There are several other `Event`xml_resp we do not consider here
            }

            buf.clear();
        }

        // debug the mta response
        // println!("{:?}", txt)
    }
}
