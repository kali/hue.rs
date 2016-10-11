extern crate philipshue;

use std::env;
use std::thread;
use std::time::Duration;

use philipshue::errors::HueError;
use philipshue::errors::BridgeError;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("usage : {:?} <devicetype>", args[0]);
    } else {
        let mut bridge = None;
        let discovery = philipshue::bridge::discover().unwrap().pop().unwrap();

        for res in discovery.build_bridge().register_user(&*args[1]){
            match res{
                Ok(r) => {
                    bridge = Some(r);
                },
                Err(HueError::BridgeError{error: BridgeError::LinkButtonNotPressed, ..}) => {
                    println!("Please, press the link on the bridge. Retrying in 5 seconds");
                    thread::sleep(Duration::from_secs(5));
                },
                Err(e) => {
                    println!("Unexpected error occured: {:?}", e);
                    break
                }
            }
        }
        println!("{:?}", bridge);
    }
}
