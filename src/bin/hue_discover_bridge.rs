extern crate philipshue;
use philipshue::bridge;

fn main() {
    let discovery = bridge::discover().unwrap().pop().unwrap();

    println!("Hue bridge found: {}", discovery.ip());
}
