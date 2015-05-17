use hyper;
use serde::json::{ self, Value };
use errors::HueError;

use std::io::Read;

pub fn discover_hue_bridge() -> Result<String, HueError> {
    let mut client = hyper::Client::new();

    let mut res = try!(client.get("https://www.meethue.com/api/nupnp").send());
    let mut body = String::new();
    try!(res.read_to_string(&mut body));

    let j:Value = try!(json::from_str(&*body));

    let objects:&Vec<Value> =
        try!(j.as_array().ok_or(HueError::StdError("Expected an array".to_string())));
    if objects.len() == 0 {
        return Err(HueError::StdError("expected non-empty array".to_string()));
    }
    let ref object = objects[0];

    let ip = try!(object.find("internalipaddress")
        .ok_or(HueError::StdError("Expected internalipaddress".to_string())));
    Ok(ip.as_string().unwrap().to_string())
}
