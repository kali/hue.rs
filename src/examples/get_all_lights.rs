extern crate philipshue;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("usage : {:?} <username>", args[0]);
        return;
    }
    let bridge = philipshue::bridge::discover().unwrap().pop().unwrap().build_bridge().from_username(args[1].clone());
    
    match bridge.get_all_lights() {
        Ok(lights) => {
            println!("id name                 on    bri   hue sat temp");
            for ref l in lights.iter() {
                println!("{:2} {:20} {:5} {:3} {:5} {:3} {:4}K",
                         l.id,
                         l.light.name,
                         if l.light.state.on { "on" } else { "off" },
                         l.light.state.bri,
                         l.light.state.hue,
                         l.light.state.sat,
                         l.light.state.ct.map(|k| 1000000u32 / (k as u32)).unwrap_or(0));
            }
        }
        Err(err) => panic!(err),
    }
}
