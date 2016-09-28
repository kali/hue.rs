extern crate hueclient;
use std::env;
use std::time::Duration;
use hueclient::errors::HueError;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("usage : {:?} <devicetype> <username>", args[0]);
    } else {
        let bridge = ::hueclient::bridge::Bridge::discover_required();
        println!("posting user {:?}/{:?} in {:?}", args[1], args[2], bridge);
        loop {
            let r = bridge.register_user(&args[1], &args[2]);
            match r {
                Ok(r) => {
                    println!("done: {:?}", r);
                    break;
                }
                Err(HueError::BridgeError(ref error)) if error.code == 101 => {
                    println!("Push the bridge button");
                    std::thread::sleep(Duration::from_secs(5));
                }
                Err(e) => panic!(e),
            }
        }
    }
}
