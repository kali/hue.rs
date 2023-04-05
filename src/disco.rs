use crate::{HueError, HueError::DiscoveryError};
use serde_json::{Map, Value};
use futures_util::{pin_mut, stream::StreamExt};
use futures::executor::block_on;
use mdns::{Record, RecordKind};
use std::{net::IpAddr, time::Duration};
use async_std::future;

// As Per instrucitons at
// https://developers.meethue.com/develop/application-design-guidance/hue-bridge-discovery/
pub fn discover_hue_bridge() -> Result<IpAddr, HueError> {

    let bridge_ftr = discover_hue_bridge_m_dns();
    let bridge = block_on(bridge_ftr);
    match  bridge{
        Ok(bridge_ip) => Ok(bridge_ip),
        Err(e) => {
            let n_upnp_result = discover_hue_bridge_n_upnp();
            if n_upnp_result.is_err() {
                Err(DiscoveryError {
                    msg: "Could not discover bridge".into(),
                })?
            } else {
                n_upnp_result
            }
        },
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
    // this method is now deprecated
    Ok(
        ssdp_probe::ssdp_probe_v4(br"IpBridge", 1, std::time::Duration::from_secs(5))?
            .first()
            .map(|it| it.to_owned().into())
            .ok_or(DiscoveryError {
                msg: "could not find bridge with ssdp_probe".into(),
            })?,
    )
}

// Define the service name for hue bridge
const SERVICE_NAME: &str = "_hue._tcp.local";

// Define a function that discovers a hue bridge using mDNS
pub async fn discover_hue_bridge_m_dns() -> Result<IpAddr, HueError> {
    // Iterate through responses from each hue bridge device, asking for new devices every 15s
    let stream_disc = mdns::discover::all(SERVICE_NAME, Duration::from_secs(1));
    let stream = match stream_disc {
        Ok(s) => s.listen(),
        Err(_e) => {
            return Err(DiscoveryError {
                msg: _e.to_string(),
            })
        }
    };
    pin_mut!(stream);
    let response = async_std::future::timeout(Duration::from_secs(5), stream.next()).await;
    match response {
        Ok(Some(Ok(response))) => {
            // Get the first IP address from the response
            let ip = response
                .records()
                .filter_map(to_ip_addr)
                .next()
                .ok_or(DiscoveryError {
                    msg: "No IP address found in response".into(),
                })?;
            Ok(ip)
        }
        Ok(Some(Err(e))) => Err(DiscoveryError {
            msg: e.to_string(),
        }),
        Ok(None) => Err(DiscoveryError {
            msg: "No response from bridge".into(),
        }),
        Err(_e) => Err(DiscoveryError {
            msg: "No response from bridge".into(),
        }),
    }
}

// Define a helper function that converts a record to an IP address
fn to_ip_addr(record: &Record) -> Option<IpAddr> {
    match record.kind {
        RecordKind::A(addr) => Some(addr.into()),
        RecordKind::AAAA(addr) => Some(addr.into()),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_discover_hue_bridge() {
        let ip = discover_hue_bridge();
        assert!(ip.is_ok());
        let ip = ip.unwrap();
        assert_eq!(ip.to_string(), "192.168.1.149");
    }
}