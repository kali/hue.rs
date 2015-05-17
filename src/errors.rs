use hyper;
use std::convert::From;
use std::error::Error;
use serde::json;
use serde::{Serialize, Deserialize, Deserializer};
use std::num::ParseIntError;
use std::fmt::{ Formatter, Display };

#[derive(Debug)]
pub struct AppError {
    pub address:String,
    pub description:String,
    pub code:u8
}

#[derive(Debug)]
pub enum HueError {
    StdError(String),
    BridgeError(AppError),
    JsonError(::serde::json::error::Error),
}

impl Error for HueError {
    fn description(&self) -> &str {
        match self {
            &HueError::StdError(ref s) => s,
            &HueError::BridgeError(ref a) => &a.description,
            &HueError::JsonError(ref json) => &json.description(),
        }
    }
}

impl Display for HueError {
    fn fmt(&self, formatter:&mut Formatter) -> Result<(), ::std::fmt::Error> {
        formatter.write_str(self.description())
    }
}

impl From<::hyper::error::Error> for HueError {
    fn from(err:hyper::error::Error) -> HueError {
        HueError::StdError(err.description().to_string() + " (hyper)")
    }
}

impl From<::std::io::Error> for HueError {
    fn from(err: ::std::io::Error) -> HueError {
        HueError::StdError(err.description().to_string() + " (io)")
    }
}

impl From<::serde::json::error::Error> for HueError {
    fn from(err: ::serde::json::error::Error) -> HueError {
        HueError::JsonError(err)
    }
}

impl From<::std::string::FromUtf8Error> for HueError {
    fn from(err: ::std::string::FromUtf8Error) -> HueError {
        HueError::StdError(err.description().to_string() + " (utf8)")
    }
}

impl From<::std::num::ParseIntError> for HueError {
    fn from(err: ::std::num::ParseIntError) -> HueError {
        HueError::StdError(err.description().to_string() + " (parseint)")
    }
}
