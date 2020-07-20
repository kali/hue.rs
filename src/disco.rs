use reqwest;
use serde_json::*;
use std::net::IpAddr;

use crate::*;

pub fn discover_hue_bridge() -> HueResult<IpAddr> {
    let n_upnp_result = discover_hue_bridge_n_upnp();
    if n_upnp_result.is_err() {
        discover_hue_bridge_upnp()
    } else {
        n_upnp_result
    }
}

pub fn discover_hue_bridge_n_upnp() -> HueResult<IpAddr> {
    let objects: Vec<Map<String, Value>> =
        reqwest::blocking::get("https://discovery.meethue.com/")?.json()?;

    if objects.len() == 0 {
        Err("expected non-empty array")?
    }
    let ref object = objects[0];

    let ip = object
        .get("internalipaddress")
        .ok_or("Expected internalipaddress")?;
    Ok(ip
        .as_str()
        .ok_or("expect a string in internalipaddress")?
        .parse()?)
}

pub fn discover_hue_bridge_upnp() -> HueResult<IpAddr> {
    // use 'IpBridge' as a marker and a max duration of 5s as per
    // https://developers.meethue.com/develop/application-design-guidance/hue-bridge-discovery/
    Ok(
        ssdp_probe::ssdp_probe_v4(br"IpBridge", 1, std::time::Duration::from_secs(5))?
            .first()
            .map(|it| it.to_owned().into())
            .ok_or("could not find bridge")?,
    )
}
