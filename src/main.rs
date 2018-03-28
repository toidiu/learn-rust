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

        let mut txt = Vec::new();
        let mut buf = Vec::new();
        let mut start = String::from("dont_print");

        // The `Reader` does not implement `Iterator` because it outputs borrowed data (`Cow`xml_resp)
        loop {
            match reader.read_event(&mut buf) {
                // for triggering namespaced events, use this instead:
                // match reader.read_namespaced_event(&mut buf) {
                Ok(Event::Start(ref e)) => {
                    // for namespaced:
                    // Ok((ref namespace_value, Event::Start(ref e)))
                    match e.name() {
                        b"timestamp" => {
                            start = String::from("print");
                            print!("timestamp: ")
                        }
                        b"subway" => {
                            println!("sub=========");
                        }
                        b"line" => println!("========="),
                        b"text" => {
                            start = String::from("dont_print");
                            //println!("text: ")
                        }
                        b"name" => print!("name: "),
                        b"status" => print!("status: "),
                        b"Date" => print!("date: "),
                        b"Time" => print!("time: "),
                        b"bus" => {
                            println!("end=======");
                            break;
                        }
                        _ => (),
                    }
                }
                Ok(Event::Text(e)) => {
                    if start != String::from("dont_print") {
                        println!("{}", e.unescape_and_decode(&reader).unwrap());
                        txt.push(e.unescape_and_decode(&reader).unwrap());
                    }
                    start = String::from("print");
                }
                Ok(Event::Eof) => break, // exits the loop when reaching end of file
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (), // There are several other `Event`xml_resp we do not consider here
            }

            // if we don't keep a borrow elsewhere, we can clear the buffer to keep memory usage low
            buf.clear();
        }

        // debug the mta response
        // println!("{:?}", txt)
    }
}
