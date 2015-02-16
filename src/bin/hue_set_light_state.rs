#![feature(collections,core,env)]
extern crate hueclient;
extern crate regex;
use std::env;
use regex::Regex;

#[allow(dead_code)]
fn main() {
    let args:Vec<String> = env::args().collect();
    if args.len() < 4 {
        println!("usage : {:?} <username> <light_id>,<light_id>,... on|off|bri:hue:sat [transition_time]", args[0]);
        return
    }
    let bridge = ::hueclient::bridge::Bridge::discover_required().with_user(args[1].to_string());
    let ref lights:Vec<usize> = args[2].split_str(",").map(|s| s.parse::<usize>().unwrap() ).collect();
    println!("lights: {:?}", lights);
    let ref command = args[3];
    let re = Regex::new("([0-9]{0,3}):([0-9]{0,5}):([0-9]{0,3})").unwrap();
    let mut parsed = match command.as_slice() {
        "on" => hueclient::bridge::CommandLight::on(),
        "off" => hueclient::bridge::CommandLight::off(),
        _ if re.is_match(command.as_slice()) => {
            let caps = re.captures(command.as_slice()).unwrap();
            let mut command = hueclient::bridge::CommandLight::on();
            command.bri = caps.at(1).and_then( |s| s.parse::<u8>().ok() );
            command.hue = caps.at(2).and_then( |s| s.parse::<u16>().ok() );
            command.sat = caps.at(3).and_then( |s| s.parse::<u8>().ok() );
            command
        }
        _ => panic!("can not understand command {:?}", command)
    };
    if args.len() == 5 {
        parsed.transitiontime = args[4].parse::<u16>().ok();
    }
    for l in lights.iter() {
        println!("{:?}", bridge.set_light_state(*l, parsed));
    }
}
