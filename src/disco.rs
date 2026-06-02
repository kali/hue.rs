use crate::{HueError, HueError::DiscoveryError};
use futures::executor::block_on;
use futures_util::{pin_mut, stream::StreamExt};
use mdns_sd::ScopedIp;
use serde_json::{Map, Value};
use smol_timeout::TimeoutExt;
use std::{net::IpAddr, time::Duration};

// As Per instrucitons at
// https://developers.meethue.com/develop/application-design-guidance/hue-bridge-discovery/
pub async fn discover_hue_bridge() -> Result<IpAddr, HueError> {
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
            let n_upnp_result = discover_hue_bridge_n_upnp().await;
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

pub async fn discover_hue_bridge_n_upnp() -> Result<IpAddr, HueError> {
    let objects: Vec<Map<String, Value>> = reqwest::get("https://discovery.meethue.com/")
        .await?
        .json()
        .await?;

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
    let mdns = mdns_sd::ServiceDaemon::new().map_err(|e| DiscoveryError { msg: e.to_string() })?;
    let receiver = mdns
        .browse(SERVICE_NAME)
        .map_err(|e| DiscoveryError { msg: e.to_string() })?;

    let stream = receiver.into_stream().filter_map(|event| async {
        match event {
            mdns_sd::ServiceEvent::ServiceResolved(service) => Some(service),
            _ => None,
        }
    });

    pin_mut!(stream);
    let response = stream
        .next()
        .timeout(Duration::from_secs(5))
        .await
        .flatten();
    let result = match response {
        Some(service) => service
            .get_addresses()
            .iter()
            .next()
            .map(ScopedIp::to_ip_addr)
            .ok_or_else(|| DiscoveryError {
                msg: "No IP address found in response".into(),
            }),
        None => Err(DiscoveryError {
            msg: "No response from bridge".into(),
        }),
    };

    let _ = mdns.shutdown();
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[ignore]
    async fn test_discover_hue_bridge() {
        let ip = discover_hue_bridge().await;
        assert!(ip.is_ok());
        let ip = ip.unwrap();
        assert_eq!(ip.to_string(), "192.168.1.149");
    }
}
