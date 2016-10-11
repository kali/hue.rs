#![warn(missing_docs)]

//! Crate for communicating with the hue API

extern crate serde;
extern crate serde_json;
extern crate hyper;
extern crate regex;

/// Errors that can occur in the crate
pub mod errors;
/// Handles all the communication with the bridge
pub mod bridge;
/// Structs mapping the different JSON-objects used with Hue API
pub mod hue;
