extern crate reqwest;

use std::io::Read;
use reqwest::blocking::Client;

fn main() {
    let client = Client::new();
	let origin_url = "http://mapps.cricbuzz.com/cbzios/match/livematches";
    let mut res = client.get(origin_url).send().unwrap();
    println!("Status for {}: {}", origin_url, res.status());

    let mut body  = String::new();
    res.read_to_string(&mut body).unwrap();
    println!("HTML: {}", &body[0..40]);
}
