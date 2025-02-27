extern crate hueclient;
use std::env;

#[allow(dead_code)]
#[tokio::main]
async fn main() {
    #[cfg(feature = "pretty_env_logger")]
    pretty_env_logger::init_custom_env("HUE_LOG");

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("usage : {:?} <username>", args[0]);
        return;
    }
    let bridge = hueclient::Bridge::discover_required()
        .await
        .with_user(args[1].to_string());
    match bridge.get_all_lights().await {
        Ok(lights) => {
            println!("id                                   name                 on    bri   hue sat temp  x      y");
            for l in lights.iter() {
                println!(
                    "{} {:20} {:5} {:3} {:5} {:3} {:4}K {:4} {:4}",
                    l.id,
                    l.metadata.name,
                    if l.on.on { "on" } else { "off" },
                    l.dimming.as_ref().map(|d| d.brightness).unwrap_or(0.0),
                    0, // hue
                    0, // sat
                    l.color_temperature
                        .as_ref()
                        .map(|ct| if let Some(mirek) = ct.mirek {
                            if mirek != 0 {
                                1000000u32 / (mirek as u32)
                            } else {
                                0
                            }
                        } else {
                            0
                        })
                        .unwrap_or(0),
                    l.color.as_ref().map(|color| color.xy.x).unwrap_or(0.0),
                    l.color.as_ref().map(|color| color.xy.y).unwrap_or(0.0),
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
