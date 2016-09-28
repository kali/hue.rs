extern crate hueclient;
use hueclient::bridge::Bridge;

fn main() {
    let bridge = Bridge::discover().unwrap();
    println!("Hue bridge found: {:?}", bridge);
}
