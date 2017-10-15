extern crate hueclient;
extern crate regex;

use std::env;
use regex::Regex;

#[allow(dead_code)]
fn main() {
    let args:Vec<String> = env::args().collect();
    if args.len() < 4 {
        println!("usage : {:?} <username> <light_id>,<light_id>,... on|off|[bri]:[hue]:[sat]|[ct]MK:[bri]|[w]K:[bri]|[RR][GG][BB]:[bri] [transition_time]", args[0]);
        return
    }
    let bridge = ::hueclient::bridge::Bridge::discover_required().with_user(args[1].to_string());
    let ref lights:Vec<usize> = args[2].split(",").map(|s| s.parse::<usize>().unwrap() ).collect();
    println!("lights: {:?}", lights);
    let ref command = args[3];
    let re_triplet = Regex::new("([0-9]{0,3}):([0-9]{0,5}):([0-9]{0,3})").unwrap();
    let re_mired = Regex::new("([0-9]{0,4})MK:([0-9]{0,5})").unwrap();
    let re_kelvin = Regex::new("([0-9]{4,4})K:([0-9]{0,5})").unwrap();
    let re_rrggbb = Regex::new("([0-9a-fA-F]{2})([0-9a-fA-F]{2})([0-9a-fA-F]{2})").unwrap();
    let mut parsed = match &command[..] {
        "on" => hueclient::bridge::CommandLight::on(),
        "off" => hueclient::bridge::CommandLight::off(),
        _ if re_triplet.is_match(&command) => {
            let caps = re_triplet.captures(&command).unwrap();
            let mut command = hueclient::bridge::CommandLight::on();
            command.bri = caps.get(1).and_then( |s| s.as_str().parse::<u8>().ok() );
            command.hue = caps.get(2).and_then( |s| s.as_str().parse::<u16>().ok() );
            command.sat = caps.get(3).and_then( |s| s.as_str().parse::<u8>().ok() );
            command
        }
        _ if re_mired.is_match(&command) => {
            let caps = re_mired.captures(&command).unwrap();
            let mut command = hueclient::bridge::CommandLight::on();
            command.ct = caps.get(1).and_then( |s| s.as_str().parse::<u16>().ok() );
            command.bri = caps.get(2).and_then( |s| s.as_str().parse::<u8>().ok() );
            command.sat = Some(254);
            command
        }
        _ if re_kelvin.is_match(&command) => {
            let caps = re_kelvin.captures(&command).unwrap();
            let mut command = hueclient::bridge::CommandLight::on();
            command.ct = caps.get(1).and_then( |s| s.as_str().parse::<u32>().ok().map(|k| (1000000u32/k) as u16));
            command.bri = caps.get(2).and_then( |s| s.as_str().parse::<u8>().ok() );
            command.sat = Some(254);
            command
        }
        _ if re_rrggbb.is_match(&command) => {
            let caps = re_rrggbb.captures(&command).unwrap();
            let mut command = hueclient::bridge::CommandLight::on();
            let rgb:Vec<u8> = [ caps.get(1), caps.get(2), caps.get(3) ].iter().map( |s| u8::from_str_radix(s.unwrap().as_str(), 16).unwrap() ).collect();
            let hsv = rgb_to_hsv(rgb[0], rgb[1], rgb[2]);
            println!("{:?}", hsv);
            command.hue = Some((hsv.0 * 65535f64) as u16);
            command.sat = Some((hsv.1 * 255f64) as u8);
            command.bri = Some((hsv.2 * 255f64) as u8);
            command
        }
        _ => panic!("can not understand command {:?}", command)
    };
    if args.len() == 5 {
        parsed.transitiontime = args[4].parse::<u16>().ok();
    }
    for l in lights.iter() {
        println!("{:?}", bridge.set_light_state(*l, parsed));
        std::thread::sleep(::std::time::Duration::from_millis(50))
    }
}


fn rgb_to_hsv(r:u8, g:u8, b:u8) -> (f64,f64,f64) {
    let r = r as f64/255f64;
    let g = g as f64/255f64;
    let b = b as f64/255f64;
    let max = r.max(g.max(b));
    let min = r.min(g.min(b));

    if max == min {
        (0f64, 0f64, max)
    } else {
        let d = max-min;
        let s = d/max;
        let h = if max==r {
            (g-b) / d + (if g < b { 6f64 } else { 0f64 })
        } else if max==g {
            (b-r) / d + 2f64
        } else {
            (r-g) / d + 4f64
        };
        (h/6f64, s, max)
    }
}
