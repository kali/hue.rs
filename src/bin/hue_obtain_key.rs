#![allow(unstable)]
extern crate hue;
use std::os;

fn main() {
    let args = os::args();
    if args.len() != 3 {
        println!("usage : {} <devicetype> <username>", args[0]);
    } else {
        let bridge = ::hue::bridge::Bridge::discover().unwrap();
        println!("posting user {:?}/{:?} in {:?}", args[1], args[2], bridge);
        let res = bridge.obtain_key(args[1].as_slice(), args[2].as_slice());
        println!("{:?}", res);
    }
}
