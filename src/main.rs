extern crate reqwest;
extern crate xml;

use self::xml::reader::{EventReader, XmlEvent};

fn main() {
    let xml = get_mta_status();

    // passing `String` wont work because String doesnt implement std::io::Read
    // parse_xml(xml);
    let readable: &[u8] = xml;
    parse_xml(readable);
}

fn get_mta_status() -> String {
    let body = reqwest::get("http://web.mta.info/status/serviceStatus.txt")
        .unwrap()
        .text()
        .unwrap();

    // println!("{:?}", body);
    body
}

fn parse_xml<T>(readable: T) where T: std::io::Read {
    let reader = EventReader::new(readable);

    for e in reader {
        match e {
            Ok(XmlEvent::StartElement { name, .. }) => print!("start: {}", name),
            Ok(XmlEvent::EndElement { name }) => println!("    end: {}", name),
            Err(e) => {
                println!("Error: {}", e);
                break;
            }
            _ => (),
        }
    }
}
