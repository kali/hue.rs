extern crate hueclient;
use std::env;

#[allow(dead_code)]
fn main() {
    #[cfg(feature = "pretty_env_logger")]
    pretty_env_logger::init_custom_env("HUE_LOG");

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("usage : {:?} <username>", args[0]);
        return;
    }
    let bridge = hueclient::Bridge::discover_required().with_user(args[1].to_string());
    match bridge.get_all_lights() {
        Ok(lights) => {
            println!("id name                 on    bri   hue sat temp  x      y");
            for l in lights.iter() {
                println!(
                    "{:2} {:20} {:5} {:3} {:5} {:3} {:4}K {:4} {:4}",
                    l.id,
                    l.light.name,
                    if l.light.state.on { "on" } else { "off" },
                    if l.light.state.bri.is_some() {
                        l.light.state.bri.unwrap()
                    } else {
                        0
                    },
                    if l.light.state.hue.is_some() {
                        l.light.state.hue.unwrap()
                    } else {
                        0
                    },
                    if l.light.state.sat.is_some() {
                        l.light.state.sat.unwrap()
                    } else {
                        0
                    },
                    if l.light.state.ct.is_some() {
                        l.light
                            .state
                            .ct
                            .map(|k| if k != 0 { 1000000u32 / (k as u32) } else { 0 })
                            .unwrap()
                    } else {
                        0
                    },
                    if l.light.state.xy.is_some() {
                        l.light.state.xy.unwrap().0
                    } else {
                        0.0
                    },
                    if l.light.state.xy.is_some() {
                        l.light.state.xy.unwrap().1
                    } else {
                        0.0
                    },
                );
            }
        }
        Err(err) => {
            log::error!("Error: {err:#?}");
            println!("Error: {err}");
            ::std::process::exit(2)
        }
    }
}
