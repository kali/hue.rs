use hyper;
use std::convert::From;
use serde_json::Error as JsonError;
use std::num::ParseIntError;

#[derive(Debug)]
/// Wrapper for all errors that can occur in this crate
pub enum HueError {
    /// The response from the bridge was malformed
    ///
    /// This doesn't happen in practice
    MalformedResponse,
    /// An error that occured in the bridge
    BridgeError{
        /// The URI the error happened on
        address: String,
        /// The `BridgeError`
        error: BridgeError
    },
    /// A `serde_json::error::Error`
    JsonError(JsonError),
    /// A `hyper::Error`
    HyperError(hyper::Error),
    /// An `std::num::ParseIntError`
    ParseIntError(ParseIntError)
}

macro_rules! error_enum {
    (
        $(#[$meta:meta])*
        pub enum $name:ident{
            $($err:ident = $n:expr),+;
            $other:ident
        }
    ) => (
        $(#[$meta])*
        pub enum $name{
            $($err = $n,)+
            $other
        }
        impl From<u16> for $name{
            fn from(n: u16) -> Self{
                match n {
                    $($n => $name::$err,)+
                    _ => $name::$other
                }
            }
        }
    );
}

error_enum!{
    /// All errors defined in http://www.developers.meethue.com/documentation/error-messages
    #[repr(u16)]
    #[allow(missing_docs)]
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum BridgeError{
        // Generic Errors
        UnauthorizedUser = 1,
        BodyContainsInvalidJson = 2,
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
        DeviceIsSetToOff = 201,
        GroupCouldNotBeCreatedGroupFull = 301,
        DeviceCouldNotBeAddedGroupFull = 302,
        DeviceIsUnreachable = 304,
        UpdateOrDeleteGroupOfThisTypeNotAllowed = 305,
        LightAlreadyUsed = 306,
        SceneCouldNotBeCreated = 401,
        SceneCouldNotBeCreatedBufferFull = 402,
        SceneCouldNotBeRemoved = 403,
        NotAllowedToCreateSensorType = 501,
        SensorListIsFull = 502,
        RuleEngineFull = 601,
        ConditionError = 607,
        ActionError = 608,
        UnableToActivae = 609,
        ScheduleListIsFull = 701,
        ScheduleTimezoneNotValid = 702,
        ScheduleCannotSetTimeAndLocalTime = 703,
        CannotCreateSchedule = 704,
        CannotEnableScheduleTimeInPast = 705,
        CommandError = 706,
        SourceModelInvalid = 801,
        SourceFactoryNew = 802,
        InvalidState = 803;
        Other
    }
}

#[test]
fn bridge_errors() {
    use self::BridgeError::*;

    assert_eq!(BridgeError::from(101), LinkButtonNotPressed);
    assert_eq!(BridgeError::from(0), Other);
    assert_eq!(BridgeError::from(51234), Other);
    assert_eq!(BridgeError::from(4), MethodNotAvailableForResource);
    assert_eq!(SceneCouldNotBeRemoved as u16, 403);
    assert_eq!(InternalError as u16, 901);
}

impl From<::hue::Error> for HueError {
    fn from(::hue::Error{address, code,..}: ::hue::Error) -> Self {
        HueError::BridgeError{
            address: address,
            error: From::from(code)
        }
    }
}

impl From<JsonError> for HueError {
    fn from(err: JsonError) -> HueError {
        HueError::JsonError(err)
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
