use crate::{HueError, HueError::DiscoveryError};
use serde_json::{Map, Value};
use std::net::IpAddr;

pub fn discover_hue_bridge() -> Result<IpAddr, HueError> {
    let n_upnp_result = discover_hue_bridge_n_upnp();
    if n_upnp_result.is_err() {
        discover_hue_bridge_upnp()
    } else {
        n_upnp_result
    }
}

pub fn discover_hue_bridge_n_upnp() -> Result<IpAddr, HueError> {
    let objects: Vec<Map<String, Value>> =
        reqwest::blocking::get("https://discovery.meethue.com/")?.json()?;

    if objects.len() == 0 {
        Err(DiscoveryError {
            msg: "expected non-empty array".into(),
        })?
    }
    let ref object = objects[0];

    let ip = object.get("internalipaddress").ok_or(DiscoveryError {
        msg: "Expected internalipaddress".into(),
    })?;
    Ok(ip
        .as_str()
        .ok_or(DiscoveryError {
            msg: "expect a string in internalipaddress".into(),
        })?
        .parse()?)
}

pub fn discover_hue_bridge_upnp() -> Result<IpAddr, HueError> {
    // use 'IpBridge' as a marker and a max duration of 5s as per
    // https://developers.meethue.com/develop/application-design-guidance/hue-bridge-discovery/
    Ok(
        ssdp_probe::ssdp_probe_v4(br"IpBridge", 1, std::time::Duration::from_secs(5))?
            .first()
            .map(|it| it.to_owned().into())
            .ok_or(DiscoveryError {
                msg: "could not find bridge".into(),
            })?,
    )
}
