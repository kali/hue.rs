use reqwest;
use serde_json::*;
use crate::*;

pub fn discover_hue_bridge() -> HueResult<String> {
    let objects: Vec<Map<String, Value>> = reqwest::blocking::get("https://www.meethue.com/api/nupnp")?
        .json()?;

    if objects.len() == 0 {
        Err("expected non-empty array")?
    }
    let ref object = objects[0];

    let ip = object.get("internalipaddress").ok_or(
        "Expected internalipaddress",
    )?;
    Ok(
        ip.as_str()
            .ok_or("expect a string in internalipaddress")?
            .to_string(),
    )
}
