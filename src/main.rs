#![deny(warnings)]

extern crate reqwest;
extern crate xml;

use self::xml::reader::{EventReader, XmlEvent};

fn main() {
    let xml = get_mta_status();

    // passing `String` wont work because String doesnt implement std::io::Read
    // parse_xml(xml);
    let readable: &[u8] = xml.as_bytes();
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

#[derive(PartialEq)]
enum XmlTag {
    TimeStamp,
    LineName,
    LineStatus,
    Ignore,
}

fn parse_xml<T>(readable: T)
where
    T: std::io::Read,
{
    let reader = EventReader::new(readable);
    let mut xml_tag: XmlTag = XmlTag::Ignore;

    for e in reader {
        match e {
            Ok(XmlEvent::StartElement { name, .. }) => {
                let ref_name: &str = name.local_name;
                match ref_name {
                    "timestamp" => {
                        xml_tag = XmlTag::TimeStamp;
                        print!("{}: ", name);
                    }

                    "name" => {
                        xml_tag = XmlTag::LineName;
                        print!("{}: ", name);
                    }

                    _ => {
                        xml_tag = XmlTag::Ignore;
                    }
                }
            }

            Ok(XmlEvent::Characters(name)) => match xml_tag {
                XmlTag::TimeStamp => println!("{}", name),
                XmlTag::LineName => println!("{}", name),
                _ => (),
            },

            Ok(XmlEvent::EndElement { name }) => {
                let ref_name: &str = name.local_name;
                match ref_name {
                    // we only care about subway
                    "subway" => break,

                    _ => (),
                }
            }

            Err(e) => {
                println!("Error: {}", e);
                break;
            }

            _ => (),
        }
    }
}
