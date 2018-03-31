extern crate reqwest;

use reqwest::Client;
use reqwest::Response;

fn main() {
    let client = Client::new();

    let xml: String = get_mta_status(client);

    let request2: String = get_mta_status(client);
}

// hint: pass a reference instead of moving ownership
fn get_mta_status(client: Client) -> String {
    let mut resp: Response = client
        .get("http://web.mta.info/status/serviceStatus.txt")
        .send().unwrap();
    let body: String = resp.text().unwrap();

    println!("{:?}", body);
    body
}
