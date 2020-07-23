use thiserror::Error;

#[derive(Error, Debug)]
pub enum HueError {
    #[error("An error occurred while performing an HTTP request")]
    Reqwest(#[from] reqwest::Error),
    #[error("An error occurred while manipulating JSON")]
    SerdeJson(#[from] serde_json::Error),
    #[error("An error occurred while parsing an address")]
    AddrParse(#[from] std::net::AddrParseError),
    #[error("An error occurred during SSDP discovery")]
    SSDP(#[from] ssdp_probe::SsdpProbeError),
    #[error("A protocol error occurred: {}", msg)]
    ProtocolError { msg: String },
    #[error("The bridge reported error code {}: {}", code, msg)]
    BridgeError { code: usize, msg: String },
    #[error("A discovery error occurred: {}", msg)]
    DiscoveryError { msg: String },
    #[error("This action requires an username to be registered")]
    NoUsername,
}

pub mod bridge;
mod disco;
