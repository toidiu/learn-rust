extern crate reqwest;

fn main() {
    get_mta_status();
}

fn get_mta_status() {
    let body = reqwest::get("http://web.mta.info/status/serviceStatus.txt").unwrap()
            .text().unwrap();

    println!("{:?}", body);
    body
}
