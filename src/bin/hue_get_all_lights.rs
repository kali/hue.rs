#![allow(unstable)]
extern crate hue;
use std::os;

fn main() {
    let args = os::args();
    if args.len() < 2 {
        println!("usage : {} <username>", args[0]);
        return
    }
    let bridge = ::hue::bridge::Bridge::discover_required().with_user(args[1].clone());
    let lights = bridge.get_all_lights();
    println!("{:?}", lights);
}
