use crate::{HueError, HueError::DiscoveryError};
use futures::executor::block_on;
use futures_util::{pin_mut, stream::StreamExt};
use mdns::{Record, RecordKind};
use serde_json::{Map, Value};
use std::{net::IpAddr, time::Duration};

// As Per instrucitons at
// https://developers.meethue.com/develop/application-design-guidance/hue-bridge-discovery/
pub fn discover_hue_bridge() -> Result<IpAddr, HueError> {
    let bridge_ftr = discover_hue_bridge_m_dns();
    let bridge = block_on(bridge_ftr);
    match bridge {
        Ok(bridge_ip) => {
            log::info!("discovered bridge at {bridge_ip} using mDNS");
            Ok(bridge_ip)
        }
        Err(mdns_error) => {
            log::debug!(
                "Error in mDNS discovery: {}, falling back to n-upnp",
                mdns_error
            );
            let n_upnp_result = discover_hue_bridge_n_upnp();
            match n_upnp_result {
                Ok(bridge_ip) => {
                    log::info!("discovered bridge at {bridge_ip} using n-upnp");
                    Ok(bridge_ip)
                }
                Err(nupnp_error) => {
                    log::debug!("Failed to discover bridge using or n-upnp: {nupnp_error}");
                    Err(DiscoveryError {
                        msg: "Could not discover bridge".into(),
                    })?
                }
            }
        }
    }
}

pub fn discover_hue_bridge_n_upnp() -> Result<IpAddr, HueError> {
    let objects: Vec<Map<String, Value>> =
        reqwest::blocking::get("https://discovery.meethue.com/")?.json()?;

    if objects.is_empty() {
        Err(DiscoveryError {
            msg: "expected non-empty array".into(),
        })?
    }
    let object = &objects[0];

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
        Ok(Some(Err(e))) => Err(DiscoveryError { msg: e.to_string() }),
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
