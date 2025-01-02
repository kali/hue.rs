extern crate hueclient;
use std::env;

#[allow(dead_code)]
fn main() {
    #[cfg(feature = "pretty_env_logger")]
    pretty_env_logger::init_custom_env("HUE_LOG");

    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("usage : {:?} <username> <scene>", args[0]);
        return;
    }
    let bridge = hueclient::Bridge::discover_required().with_user(args[1].to_string());
    match bridge.set_scene(args[2].to_string()) {
        Ok(result) => {
            println!("scene: {}, {}", args[2], result)
        }
        Err(err) => {
            log::error!("Error: {err:#?}");
            println!("Error: {err}");
            ::std::process::exit(2)
        }
    }
}
