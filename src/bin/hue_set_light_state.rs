extern crate hueclient;
extern crate regex;
use std::env;
use regex::Regex;

#[allow(dead_code)]
fn main() {
    let args:Vec<String> = env::args().collect();
    if args.len() < 4 {
        println!("usage : {:?} <username> <light_id>,<light_id>,... on|off|[bri]:[hue]:[sat]|[ct]MK:[bri]|[w]K:[bri] [transition_time]", args[0]);
        return
    }
    let bridge = ::hueclient::bridge::Bridge::discover_required().with_user(args[1].to_string());
    let ref lights:Vec<usize> = args[2].split(",").map(|s| s.parse::<usize>().unwrap() ).collect();
    println!("lights: {:?}", lights);
    let ref command = args[3];
    let re_triplet = Regex::new("([0-9]{0,3}):([0-9]{0,5}):([0-9]{0,3})").unwrap();
    let re_mired = Regex::new("([0-9]{0,4})MK:([0-9]{0,5})").unwrap();
    let re_kelvin = Regex::new("([0-9]{4,4})K:([0-9]{0,5})").unwrap();
    let mut parsed = match &command[..] {
        "on" => hueclient::bridge::CommandLight::on(),
        "off" => hueclient::bridge::CommandLight::off(),
        _ if re_triplet.is_match(&command) => {
            let caps = re_triplet.captures(&command).unwrap();
            let mut command = hueclient::bridge::CommandLight::on();
            command.bri = caps.at(1).and_then( |s| s.parse::<u8>().ok() );
            command.hue = caps.at(2).and_then( |s| s.parse::<u16>().ok() );
            command.sat = caps.at(3).and_then( |s| s.parse::<u8>().ok() );
            command
        }
        _ if re_mired.is_match(&command) => {
            let caps = re_mired.captures(&command).unwrap();
            let mut command = hueclient::bridge::CommandLight::on();
            command.ct = caps.at(1).and_then( |s| s.parse::<u16>().ok() );
            command.bri = caps.at(2).and_then( |s| s.parse::<u8>().ok() );
            command
        }
        _ if re_kelvin.is_match(&command) => {
            let caps = re_kelvin.captures(&command).unwrap();
            let mut command = hueclient::bridge::CommandLight::on();
            command.ct = caps.at(1).and_then( |s| s.parse::<u32>().ok().map(|k| (1000000u32/k) as u16));
            command.bri = caps.at(2).and_then( |s| s.parse::<u8>().ok() );
            command
        }
        _ => panic!("can not understand command {:?}", command)
    };
    if args.len() == 5 {
        parsed.transitiontime = args[4].parse::<u16>().ok();
    }
    for l in lights.iter() {
        println!("{:?}", bridge.set_light_state(*l, parsed));
        std::thread::sleep_ms(50)
    }
}
