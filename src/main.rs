extern crate reqwest;

use reqwest::Client;
use reqwest::Response;

fn main() {
    let client = Client::new();
    let xml: String = get_mta_status();
}

fn get_mta_status(client: Client) -> String {
    let resp: Response = client
        .get("http://web.mta.info/status/serviceStatus.txt")
        .send().unwrap();
    let body: String = resp.text().unwrap();

    println!("{:?}", body);
    body
}
