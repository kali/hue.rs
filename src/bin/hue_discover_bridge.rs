extern crate hue;

fn main() {
    let bridge = hue::disco::discover_hue_bridge().unwrap();
    println!("Hue bridge found: {}", bridge);
}
