use ::bridge::BridgeBuilder;

use serde::{Serialize, Deserialize};

#[derive(Debug, Copy, Clone, Deserialize)]
/// The state of the light with similar structure to `LightCommand`
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

#[derive(Debug, Clone, Deserialize)]
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
    /// The state of the light (See `LightState` for more)
    pub state: LightState
}

#[derive(Debug, Clone)]
/// A newly identified light
pub struct IdentifiedLight {
    /// The ID number of this light
    pub id: usize,
    /// The light object
    pub light: Light,
}

#[derive(Debug, Default, Clone, Copy, Serialize)]
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

#[derive(Debug, Clone, Deserialize)]
/// Responses from the `discover` function
pub struct Discovery{
    id: String,
    internalipaddress: String
}

impl Discovery {
    /// Returns a `BridgeBuilder` with the ip of the bridge discovered
    pub fn build_bridge(self) -> BridgeBuilder{
        let Discovery{internalipaddress,..} = self;
        BridgeBuilder::from_ip(internalipaddress)
    }
    /// The ip of this discovered bridge
    pub fn ip(&self) -> &str{
        &self.internalipaddress
    }
    /// The id of this discovered bridge
    pub fn id(&self) -> &str{
        &self.id
    }
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

#[derive(Debug, Deserialize)]
/// A response that either is an error or a success
pub struct HueResponse<T: Serialize + Deserialize>{
    /// The result from the bridge if it didn't fail
    pub success: Option<T>,
    /// The error that was returned from the bridge
    pub error: Option<Error>
}

#[derive(Debug, Deserialize, Serialize)]
/// A user object returned from the API
pub struct User{
    /// The username of the user
    pub username: String
}

#[derive(Debug, Deserialize)]
/// An error object returned from the API
pub struct Error {
    /// The URI the error happened on
    pub address: String,
    /// A short description of the error
    pub description: String,
    /// Its errorcode
    #[serde(rename="type")]
    pub code: u16,
}
