extern crate philipshue;
use philipshue::bridge::Bridge;

fn main() {
    let bridge = Bridge::discover().unwrap();
    println!("Hue bridge found: {:?}", bridge);
}
