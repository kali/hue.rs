extern crate hue;
use hue::bridge::Bridge;

fn main() {
    let bridge = Bridge::discover().unwrap();
    println!("Hue bridge found: {:?}", bridge);
}
