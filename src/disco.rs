use crate::{HueError, HueError::DiscoveryError};
use serde_json::{Map, Value};
use futures_util::{pin_mut, stream::StreamExt};
use futures::executor::block_on;
use mdns::{Error, Record, Response};
use std::{net::IpAddr, time::Duration};
use async_std::prelude::Stream;


// As Per instrucitons at
// https://developers.meethue.com/develop/application-design-guidance/hue-bridge-discovery/
pub fn discover_hue_bridge() -> Result<IpAddr, HueError> {
    let bridge = discover_hue_bridge_m_dns();
    match  bridge{
        Ok(bridge_ip) => Ok(bridge_ip),
        Err(_e) => {
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
        ssdp_probe::ssdp_probe_v4(br"IpBridge", 1, std::time::Duration::from_secs(1))?
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
pub fn discover_hue_bridge_m_dns() -> Result<IpAddr, HueError> {
    read_mdns_response(mdns::discover::all(SERVICE_NAME, Duration::from_secs(1)).unwrap().listen())
}

fn read_mdns_response(stream: impl Stream<Item=Result<Response, Error>> + Sized) -> Result<IpAddr, HueError> {
    pin_mut!(stream);
    let response_or = block_on(async_std::future::timeout(Duration::from_secs(5), stream.next()));
    let response = match response_or {
        Ok(Some(Ok(response))) => response,
        Ok(Some(Err(e))) => Err(DiscoveryError { msg: format!("Error reading mDNS response: {}", e) })?,
        Ok(None) => Err(DiscoveryError { msg: "No mDNS response found".into() })?,
        Err(_) => Err(DiscoveryError { msg: "Timed out waiting for mDNS response".into() })?,
    };
    response.ip_addr().ok_or(DiscoveryError { msg: "No IP address found".into() })
}


#[cfg(test)]
mod tests {
    use std::future;
    use mdns::RecordKind::A;
    use futures::FutureExt;
    use super::*;

    #[test]
    #[ignore]
    fn test_discover_hue_bridge() {
        let ip = discover_hue_bridge();
        assert!(ip.is_ok());
        let ip = ip.unwrap();
        assert_eq!(ip.to_string(), "192.168.1.149");
    }

    // test resolve_mdns_result using mock response
    #[test]
    fn test_read_mdns_response() {

        let record = Record {
            name: "_hue._tcp.local".to_string(),
            class: dns_parser::Class::IN,
            ttl: 0,
            kind: (A("192.168.1.149".parse().unwrap())),
        };

        let response = Response {
            answers: vec![record],
            nameservers: vec![],
            additional: vec![],
        };

        let stream = futures::stream::iter(vec![Ok::<mdns::Response, Error>(response)]);
        let ip = read_mdns_response(stream).unwrap();
        assert_eq!(ip.to_string(), "192.168.1.149");
    }

    #[test]
    fn should_error_when_no_mdns_bridge_found() {
        let stream = futures::stream::iter(vec![]);
        let ip = read_mdns_response(stream);
        assert!(ip.is_err());
    }

    #[test]
    fn should_timeout_when_timeout_exceeded() {
        // this stream never returns a value
        let stream =  futures::future::pending::<Result<Response, Error>>().into_stream();
        let ip = read_mdns_response(stream);
        //assert that the error message is "Timed out waiting for mDNS response"
        assert!(ip.is_err());
        assert_eq!(ip.err().unwrap().to_string(), "A discovery error occurred: Timed out waiting for mDNS response");
    }
}