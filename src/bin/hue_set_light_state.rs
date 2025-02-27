extern crate hueclient;
extern crate regex;

use std::env;

#[allow(dead_code)]
#[tokio::main]
async fn main() {
    #[cfg(feature = "pretty_env_logger")]
    pretty_env_logger::init_custom_env("HUE_LOG");

    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        println!(
            "usage : {:?} <username> <light_id>,<light_id>,... on|off|[bri]:[hue]:[sat]|[ct]MK:[bri]|[w]K:[bri]|[RR][GG][BB]:[bri]|[x,y]:[bri] [transition_time]",
            args[0]
        );
        return;
    }
    let bridge = hueclient::Bridge::discover_required()
        .await
        .with_user(args[1].to_string());

    let light_ids = args[2].clone();
    let lights: Vec<&str> = light_ids.split(",").collect();
    let parsed = hueclient::parse_command(args);

    println!("lights: {:?}", lights);
    for l in lights.iter() {
        println!("{:?}", bridge.set_light_state(l, &parsed).await);
        std::thread::sleep(::std::time::Duration::from_millis(50))
    }
}
