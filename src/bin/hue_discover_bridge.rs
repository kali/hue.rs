extern crate hueclient;
use hueclient::bridge::Bridge;

#[allow(dead_code)]
fn main() {
    let bridge = Bridge::discover().unwrap();
    println!("Hue bridge found: {:?}", bridge);
}
