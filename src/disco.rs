use hyper;
use rustc_serialize::json;
use std::error;

#[derive(Show)]
pub enum HueError {
    JsonError(json::ParserError),
    HttpError(hyper::HttpError),
    Error(String)
}

impl error::FromError<json::ParserError> for HueError {
    fn from_error(err: json::ParserError) -> HueError {
        HueError::JsonError(err)
    }
}

impl error::FromError<hyper::HttpError> for HueError {
    fn from_error(err: hyper::HttpError) -> HueError {
        HueError::HttpError(err)
    }
}

pub fn discover_hue_bridge() -> Result<String, HueError> {
    let mut client = hyper::Client::new();

    let mut res = try!(client.get("https://www.meethue.com/api/nupnp").send());

    let j = try!(::tools::from_reader(&mut res));

    let objects:&json::Array =
        try!(j.as_array().ok_or(HueError::Error("Expected an array".to_string())));
    let ref object = objects[0];

    let ip = try!(object.find("internalipaddress")
        .ok_or(HueError::Error("Expected internalipaddress".to_string())));
    Ok(ip.to_string())
}

#[test]
fn it_does_find_the_bridge() {
    discover_hue_bridge();
}
