#![deny(warnings)]

extern crate reqwest;
extern crate xml;

use reqwest::Client;
use reqwest::Response;
use self::xml::reader::{EventReader, XmlEvent};

fn main() {
    let client = Client::new();

    let xml: String = get_mta_status(&client);

    // passing `String` wont work because String doesnt implement std::io::Read
    // parse_xml(xml);
    let readable: &[u8] = xml.as_bytes();
    parse_xml(readable);
}

fn get_mta_status(client: &Client) -> String {
    let mut resp: Response = client
        .get("http://web.mta.info/status/serviceStatus.txt")
        .send()
        .unwrap();
    let body: String = resp.text().unwrap();

    println!("{:?}", body);
    body
}

/// Because we are using a XML event streaming
/// library we need to maintain state of which
/// tag we are processing.
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
    // This will hold the current tag we are processing.
    // We set it to a default value or `Ignore`.
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
