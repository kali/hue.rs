use hyper;
use std::convert::From;
use rustc_serialize::json;
use std::num::ParseIntError;

#[derive(Debug)]
// TODO FIXME
/// Errors that can occur in this crate
pub enum HueError {
    /// Error that occurs when the response from the bridge is malformed
    MalformedResponse,
    /// An error that occured in the bridge
    BridgeError{
        /// The URI the error happened on
        address: String,
        /// The `BridgeError`
        error: BridgeError
    },
    /// A `json::EncoderError`
    EncoderError(json::EncoderError),
    /// A `json::DecoderError`
    DecoderError(json::DecoderError),
    /// A `json::ParserError`
    ParserError(json::ParserError),
    /// A `hyper::Error`
    HyperError(hyper::Error),
    /// An `std::num::ParseIntError`
    ParseIntError(ParseIntError)
}

/// All errors defined in [http://www.developers.meethue.com/documentation/error-messages]
#[repr(u16)]
#[allow(missing_docs)]
#[derive(Debug, Copy, Clone)]
pub enum BridgeError{
    // Generic Errors
    UnauthorizedUser = 1,
    BodyContainsInvalidJSON = 2,
    ResourceNotAvailable = 3,
    MethodNotAvailableForResource = 4,
    MissingParametersInBody = 5,
    ParameterNotAvailable = 6,
    InvalidValueForParameter = 7,
    ParameterIsNotModifiable = 8,
    TooManyItemsInList = 11,
    ProtalConnectionRequired = 12,
    InternalError = 901,

    // Command Specific Errors
    LinkButtonNotPressed = 101,
    DHCPCannotBeDisabled = 110,
    InvalidUpdateState = 111,
    // TODO add the rest of the command specific errors
    Other
}

use std::mem::transmute;

impl From<::hue::Error> for HueError {
    fn from(::hue::Error{address, code,..}: ::hue::Error) -> Self {
        HueError::BridgeError{
            address: address,
            error: unsafe{transmute(code)}
        }
    }
}

impl From<json::EncoderError> for HueError {
    fn from(err: json::EncoderError) -> HueError {
        HueError::EncoderError(err)
    }
}

impl From<json::DecoderError> for HueError {
    fn from(err: json::DecoderError) -> HueError {
        HueError::DecoderError(err)
    }
}

impl From<json::ParserError> for HueError {
    fn from(err: json::ParserError) -> HueError {
        HueError::ParserError(err)
    }
}

impl From<hyper::error::Error> for HueError {
    fn from(err: hyper::error::Error) -> HueError {
        HueError::HyperError(err)
    }
}

impl From<ParseIntError> for HueError {
    fn from(err: ParseIntError) -> HueError {
        HueError::ParseIntError(err)
    }
}
