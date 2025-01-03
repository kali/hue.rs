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
    println!("Rooms");
    match bridge.resolve_all_rooms() {
        Ok(rooms) => {
            println!("id                                   name                 on");
            for  r in rooms.iter() {
                println!(
                    "{:2} {:20} {:5}",
                    r.id,
                    r.metadata.name,
                    if r.children.iter().all(|l| l.on.on) {
                        "all on"
                    } else if r.children.iter().any(|l| l.on.on) {
                        "some on"
                    } else {
                        "all off"
                    },
                );
                for service in &r.services {
                    println!("  service: {} {}", service.rid, service.rtype);
                }
            }
        }
        Err(err) => {
            log::error!("Error: {err:#?}");
            println!("Error: {err}");
            ::std::process::exit(2)
        }
    }
    println!("Zones");
    match bridge.resolve_all_zones() {
        Ok(rooms) => {
            println!("id                                   name                 on");
            for  r in rooms.iter() {
                println!(
                    "{:2} {:20} {:5}",
                    r.id,
                    r.metadata.name,
                    if r.children.iter().all(|l| l.on.on) {
                        "all on"
                    } else if r.children.iter().any(|l| l.on.on) {
                        "some on"
                    } else {
                        "all off"
                    },
                );
                for service in &r.services {
                    println!("  service: {} {}", service.rid, service.rtype);
                }
            }
        }
        Err(err) => {
            log::error!("Error: {err:#?}");
            println!("Error: {err}");
            ::std::process::exit(2)
        }
    }
}
