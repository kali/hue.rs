#![warn(missing_docs)]

//! Crate for communicating with the hue API

extern crate serde;
extern crate serde_json;
extern crate hyper;
extern crate regex;

/// All things errors
pub mod errors;
/// Module responsible for communicating with the Hue bridge
pub mod bridge;
/// Module with structs mapping the different JSON-objects used with Hue API
pub mod hue;
