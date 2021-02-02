//! This library aims to enable communicating with _Philips Hue_ lights via the correspnding Bridge.
//!
//! # Examples
//! A short overview of the most common use cases of this library.
//! ### Initial Setup
//! ```rust
//! let bridge = hueclient::Bridge::discover_required()
//!     .register_user("mycomputer") // Press the bridge before running this
//!     .unwrap();
//! println!("the username was {}", bridge.username); // handy for later
//! ```
//! ### Second run
//! ```rust
//! const USERNAME: &str = "the username that was generated in the previous example";
//! let bridge = hueclient::Bridge::discover_required()
//!    .with_user(USERNAME);
//! ```
//! ### Good night
//! ```rust
//! # const USERNAME: &str = "the username that was generated in the previous example";
//! # let bridge = hueclient::Bridge::discover_required()
//! #   .with_user(USERNAME);
//! let cmd = hueclient::light::LightCommand::new().off();
//! for light in &bridge.lights().unwrap() {
//!     bridge.set_light_state(light.id, &cmd).unwrap();
//! }
//! ```

/// Represents any of the ways that usage of this library may fail.
#[derive(thiserror::Error, Debug)]
pub enum HueError {
    /// Returned when a network error occurs.
    #[error("An error occurred while performing an HTTP request")]
    Reqwest(#[from] reqwest::Error),
    /// Returned on a JSON failure, which will usually be a problem with deserializing the bridge
    /// response.
    #[error("An error occurred while manipulating JSON")]
    SerdeJson(#[from] serde_json::Error),
    /// Returned when discovery.meethue.com returns an invalid IP-address.
    #[error("An error occurred while parsing an address")]
    AddrParse(#[from] std::net::AddrParseError),
    /// Returned when the SSDP probe fails to scan the current network for a bridge.
    #[error("An error occurred during SSDP discovery")]
    SSDP(#[from] ssdp_probe::SsdpProbeError),
    /// Returned when the Bridge returns a response that does not confirm to the API spec.
    #[error("A protocol error occurred: {}", msg)]
    ProtocolError {
        /// An error message describing the failure.
        msg: String,
    },
    /// Returned when the Bridge returns an error response
    #[error("The bridge reported error code {}: {}", code, msg)]
    BridgeError {
        /// The error code.
        code: usize,
        /// An error message describing the failure.
        msg: String,
    },
    /// Returned when discovering a bridge in the local network fails.
    #[error("A discovery error occurred: {}", msg)]
    DiscoveryError {
        /// An error message describing the failure.
        msg: String,
    },
}

impl HueError {
    pub(crate) fn protocol_err(err: impl std::fmt::Display) -> Self {
        Self::ProtocolError {
            msg: err.to_string(),
        }
    }
}

/// A type alias used for convenience and consiceness throughout the library.
pub type Result<T> = std::result::Result<T, HueError>;

mod bridge;
mod disco;
mod command_parser;

pub use bridge::*;
pub use command_parser::*;
