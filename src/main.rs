extern crate reqwest;
extern crate xml;

use self::xml::reader::{EventReader, XmlEvent};
use reqwest::Client;
use reqwest::Response;

fn main() {
    let client = Client::new();

    let xml: String = get_mta_status(&client);

    // passing `String` wont work because String doesnt implement std::io::Read
    // parse_xml(xml);
    let readable: &[u8] = xml.as_bytes();
    let lines = parse_xml(readable);
    for line in lines.iter() {
        println!("{:?}", line);
    };
}

fn get_mta_status(client: &Client) -> String {
    let mut resp: Response = client
        .get("http://web.mta.info/status/serviceStatus.txt")
        .send()
        .unwrap();
    let body: String = resp.text().unwrap();

    body
}

#[derive(Debug)]
struct Line {
    name: String,
    status: String,
}

impl Line {
    fn empty() -> Line {
        Line {
            name: "".into(),
            status: "".into(),
        }
    }
}

/// Because we are using a XML event streaming
/// library we need to maintain state of which
/// tag we are processing.
enum XmlTag {
    TimeStamp,
    LineName,
    LineStatus,
    Ignore,
}

fn parse_xml<T>(readable: T) -> Vec<Line>
where
    T: std::io::Read,
{
    let reader = EventReader::new(readable);
    // This will hold the current tag we are processing.
    // We set it to a default value or `Ignore`.
    let mut xml_tag: XmlTag = XmlTag::Ignore;

    let mut lines = Vec::new();

    let mut temp_line = Line::empty();

    for e in reader {

        match e {
            Ok(XmlEvent::StartElement { name, .. }) => {
                let ref_name: &str = name.local_name.as_ref();
                match ref_name {
                    "timestamp" => {
                        xml_tag = XmlTag::TimeStamp;
                        print!("{}: ", name);
                    }

                    "name" => {
                        xml_tag = XmlTag::LineName;
                    }

                    "status" => {
                        xml_tag = XmlTag::LineStatus;
                    }

                    _ => {
                        xml_tag = XmlTag::Ignore;
                    }
                }
            }

            Ok(XmlEvent::Characters(txt)) => match xml_tag {
                XmlTag::TimeStamp => println!("{}", txt),

                XmlTag::LineName => {
                    temp_line.name = txt;
                }

                XmlTag::LineStatus => {
                    temp_line.status = txt;
                }

                _ => (),
            },

            Ok(XmlEvent::EndElement { name }) => {
                let ref_name: &str = name.local_name.as_ref();
                match ref_name {
                    "line" => {
                        lines.push(temp_line);
                        temp_line = Line::empty();
                    }

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

    lines
}
