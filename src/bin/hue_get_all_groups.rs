extern crate hueclient;
use std::env;

#[allow(dead_code)]
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("usage : {:?} <username>", args[0]);
        return;
    }
    let bridge = hueclient::Bridge::discover_required().with_user(args[1].to_string());
    match bridge.get_all_groups() {
        Ok(groups) => {
            println!("id name                 on    bri   hue sat temp  x      y");
            for ref l in groups.iter() {
                println!(
                    "{:2} {:20} {:5} {:3} {:5} {:3} {:4}K {:4} {:4}",
                    l.id,
                    l.group.name,
                    if l.group.state.all_on {
                        "all on"
                    } else if l.group.state.any_on {
                        "some on"
                    } else {
                        "all off"
                    },
                    if l.group.action.bri.is_some() {
                        l.group.action.bri.unwrap()
                    } else {
                        0
                    },
                    if l.group.action.hue.is_some() {
                        l.group.action.hue.unwrap()
                    } else {
                        0
                    },
                    if l.group.action.sat.is_some() {
                        l.group.action.sat.unwrap()
                    } else {
                        0
                    },
                    if l.group.action.ct.is_some() {
                        l.group
                            .action
                            .ct
                            .map(|k| if k != 0 { 1000000u32 / (k as u32) } else { 0 })
                            .unwrap()
                    } else {
                        0
                    },
                    if l.group.action.xy.is_some() {
                        l.group.action.xy.unwrap().0
                    } else {
                        0.0
                    },
                    if l.group.action.xy.is_some() {
                        l.group.action.xy.unwrap().1
                    } else {
                        0.0
                    },
                );
            }
        }
        Err(err) => {
            println!("Error: {:?}", err);
            ::std::process::exit(2)
        }
    }
}
