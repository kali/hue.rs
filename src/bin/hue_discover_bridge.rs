extern crate hueclient;
use hueclient::Bridge;

#[allow(dead_code)]
#[tokio::main]
async fn main()  {
    #[cfg(feature = "pretty_env_logger")]
    pretty_env_logger::init_custom_env("HUE_LOG");

    let bridge = Bridge::discover().unwrap();
    println!("Hue bridge found: {:?}", bridge);
}
