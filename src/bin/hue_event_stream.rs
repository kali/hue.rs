extern crate hueclient;
use futures::StreamExt;
use std::env;

#[tokio::main]
async fn main() {
    #[cfg(feature = "pretty_env_logger")]
    pretty_env_logger::init_custom_env("HUE_LOG");

    log::info!("Starting hue_event_stream");

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("usage : {:?} <username>", args[0]);
        return;
    }
    let bridge = hueclient::Bridge::discover()
        .unwrap()
        .with_user(args[1].to_string());

    println!("got bridge");

    match bridge.events() {
        Ok(events) => {
            events
                .for_each(|event| async move {
                    println!("{:?}", event);
                })
                .await
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}
