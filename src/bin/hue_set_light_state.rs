#![allow(unstable)]
extern crate hue;
use std::os;

fn main() {
    let args = os::args();
    if args.len() < 4 {
        println!("usage : {} <username> <light_id>,<light_id>,... on|off|bri:hue:sat [transition_time]", args[0]);
        return
    }
    let bridge = ::hue::bridge::Bridge::discover_required().with_user(args[1].clone());
    let ref lights = args[2];
    let ref command = args[3];
    let parsed = match command.as_slice() {
        "on" => hue::bridge::CommandLight::on(),
        "off" => hue::bridge::CommandLight::off(),
        _ => panic!("can not understand command {:?}", command)
    };
    bridge.set_light_state(8, parsed);
    //println!("{:?}", parsed)
}
