use rustc_serialize::{Encoder, Decoder, Encodable, Decodable};

#[derive(Debug,Copy,Clone,RustcDecodable)]
/// The state of the light, very similar to `LightCommand` except most fields aren't optional
pub struct LightState {
    /// Whether the light is on
    pub on: bool,
    /// Brightness of the light. This is a scale from the minimum capable brightness, 1, to the maximum, 254.
    pub bri: u8,
    /// Hue of the light. Both 0 and 65535 are red, 25500 is green and 46920 is blue.
    pub hue: u16,
    /// Staturation of the light. 254 is the most saturated (colored) and 0 is the least (white).
    pub sat: u8,
    /// The [mired](http://en.wikipedia.org/wiki/Mired) colour temperature of the light.
    pub ct: Option<u16>,
}

#[derive(Debug,Clone,RustcDecodable)]
/// Details about a specific light
pub struct Light {
    /// The unique name given to the light
    pub name: String,
    /// The hardware model of the light
    pub modelid: String,
    /// The version of the software running on the light
    pub swversion: String,
    /// Unique ID of the device
    pub uniqueid: String,
    /// The state of the light (`LightState` for more)
    pub state: LightState
}

#[derive(Debug,Clone)]
/// A newly identified light
pub struct IdentifiedLight {
    /// The ID number of this light
    pub id: usize,
    /// The light object
    pub light: Light,
}

#[derive(Debug, Default, Clone, Copy, RustcEncodable, RustcDecodable)]
/// Struct for building a command that will be sent to the Hue bridge telling it what to do with a light
///
/// View [the lights-api documention](http://www.developers.meethue.com/documentation/lights-api) for more information
pub struct LightCommand {
    /// Whether to turn the light off or on
    pub on: Option<bool>,
    /// Brightness of the colour of the light
    pub bri: Option<u8>,
    /// The hue of the colour of the light
    pub hue: Option<u16>,
    /// The saturation of the colour of the light
    pub sat: Option<u8>,
    /// The Mired Color temperature of the light. 2012 connected lights are capable of 153 (6500K) to 500 (2000K).
    pub ct: Option<u16>,
}

impl LightCommand {
    /// Returns a `LightCommand` that turns a light on
    pub fn on(self) -> Self {
        LightCommand { on: Some(true), ..self }
    }
    /// Returns a `LightCommand` that turns a light on
    pub fn off(self) -> Self {
        LightCommand { on: Some(false), ..self }
    }
    /// Sets the brightness to set the light to
    pub fn with_bri(self, b: u8) -> Self {
        LightCommand { bri: Some(b), ..self }
    }
    /// Sets the hue to set the light to
    pub fn with_hue(self, h: u16) -> Self {
        LightCommand { hue: Some(h), ..self }
    }
    /// Sets the saturation to set the light to
    pub fn with_sat(self, s: u8) -> Self {
        LightCommand { sat: Some(s), ..self }
    }
    /// Sets the temperature to set the light to
    pub fn with_ct(self, c: u16) -> Self {
        LightCommand { ct: Some(c), ..self }
    }
}

#[derive(Debug, RustcEncodable, RustcDecodable)]
/// A response that either is an error or a success
pub struct HueResponse<T: Encodable + Decodable>{
    /// The result from the bridge if it didn't fail
    pub success: Option<T>,
    /// The error that was returned from the bridge
    pub error: Option<Error>
}

#[derive(Debug, RustcEncodable, RustcDecodable)]
/// A user object returned from the API
pub struct User{
    /// The username of the user
    pub username: String
}

#[derive(Debug)]
/// An error object returned from the API
pub struct Error {
    /// The URI the error happened on
    pub address: String,
    /// A short description of the error
    pub description: String,
    /// Its errorcode
    pub code: u16,
}

impl Encodable for Error {
    fn encode<S: Encoder>(&self, encoder: &mut S) -> Result<(), S::Error> {
        match *self {
            Error { address: ref p_address, description: ref p_description, code: p_code } => {
                encoder.emit_struct("Error", 0, |encoder| {
                    try!(encoder.emit_struct_field("address", 0, |encoder| p_address.encode(encoder)));
                    try!(encoder.emit_struct_field("description", 1, |encoder| p_description.encode(encoder)));
                    try!(encoder.emit_struct_field("type", 2, |encoder| p_code.encode(encoder)));
                    Ok(())
                })
            }
        }
    }
}

impl Decodable for Error {
    fn decode<S: Decoder>(decoder: &mut S) -> Result<Error, S::Error> {
        decoder.read_struct("root", 0, |decoder| {
            Ok(Error {
                address: try!(decoder.read_struct_field("address", 0, |decoder| Decodable::decode(decoder))),
                description: try!(decoder.read_struct_field("description", 1, |decoder| Decodable::decode(decoder))),
                code: try!(decoder.read_struct_field("type", 2, |decoder| Decodable::decode(decoder))),
            })
        })
    }
}
