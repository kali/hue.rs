use hyper;
use rustc_serialize::json;
use errors::HueError;

pub fn discover_hue_bridge() -> Result<String, HueError> {
    let mut client = hyper::Client::new();

    let mut res = try!(client.get("https://www.meethue.com/api/nupnp").send());

    let j = try!(::tools::from_reader(&mut res));

    let objects:&json::Array =
        try!(j.as_array().ok_or(HueError::Error("Expected an array".to_string())));
    let ref object = objects[0];

    let ip = try!(object.find("internalipaddress")
        .ok_or(HueError::Error("Expected internalipaddress".to_string())));
    Ok(ip.as_string().unwrap().to_string())
}
