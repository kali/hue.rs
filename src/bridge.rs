use std::str::FromStr;
use std::collections::HashMap;

use reqwest;
use serde_json::Value;

use disco;
use Result;


#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct LightState {
    pub on: bool,
    pub bri: u8,
    pub hue: u16,
    pub sat: u8,
    pub ct: Option<u16>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Light {
    pub name: String,
    pub modelid: String,
    pub swversion: String,
    pub uniqueid: String,
    pub state: LightState,
}

#[derive(Debug, Clone)]
pub struct IdentifiedLight {
    pub id: usize,
    pub light: Light,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct CommandLight {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bri: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hue: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sat: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ct: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transitiontime: Option<u16>,
}

impl CommandLight {
    pub fn empty() -> CommandLight {
        CommandLight {
            on: None,
            bri: None,
            hue: None,
            sat: None,
            transitiontime: None,
            ct: None,
        }
    }
    pub fn on() -> CommandLight {
        CommandLight {
            on: Some(true),
            ..CommandLight::empty()
        }
    }
    pub fn off() -> CommandLight {
        CommandLight {
            on: Some(false),
            ..CommandLight::empty()
        }
    }
    pub fn with_bri(&self, b: u8) -> CommandLight {
        CommandLight {
            bri: Some(b),
            ..*self
        }
    }
    pub fn with_hue(&self, h: u16) -> CommandLight {
        CommandLight {
            hue: Some(h),
            ..*self
        }
    }
    pub fn with_sat(&self, s: u8) -> CommandLight {
        CommandLight {
            sat: Some(s),
            ..*self
        }
    }
    pub fn with_ct(&self, c: u16) -> CommandLight {
        CommandLight {
            ct: Some(c),
            ..*self
        }
    }
}

#[derive(Debug)]
pub struct Bridge {
    ip: String,
    username: Option<String>,
}

impl Bridge {
    #[allow(dead_code)]
    pub fn discover() -> Option<Bridge> {
        disco::discover_hue_bridge().ok().map(|i| {
            Bridge {
                ip: i,
                username: None,
            }
        })
    }

    pub fn discover_required() -> Bridge {
        Bridge::discover().unwrap_or_else(|| panic!("No bridge found!"))
    }

    pub fn with_user(self, username: String) -> Bridge {
        Bridge {
            username: Some(username),
            ..self
        }
    }

    pub fn register_user(&self, devicetype: &str) -> Result<String> {
        #[derive(Serialize, Deserialize)]
        struct PostApi {
            devicetype: String,
        }
        #[derive(Serialize, Deserialize)]
        struct Success {
            success: Username,
        }
        #[derive(Serialize, Deserialize)]
        struct Username {
            username: String,
        }
        let obtain = PostApi { devicetype: devicetype.to_string() };
        let url = format!("http://{}/api", self.ip);
        let client = reqwest::Client::new();
        let success: Success = self.parse(
            client.post(&url[..]).json(&obtain).send()?.json()?,
        )?;
        Ok(success.success.username)
    }

    pub fn get_all_lights(&self) -> Result<Vec<IdentifiedLight>> {
        let url = format!(
            "http://{}/api/{}/lights",
            self.ip,
            self.username.clone().unwrap()
        );
        let resp: HashMap<String, Light> = self.parse(reqwest::get(&url[..])?.json()?)?;
        let mut lights = vec![];
        for (k, v) in resp {
            let id: usize = usize::from_str(&k).or(Err(::ErrorKind::ProtocolError(
                "Light id should be a number".to_string(),
            )))?;
            lights.push(IdentifiedLight { id: id, light: v });
        }
        lights.sort_by(|a, b| a.id.cmp(&b.id));
        Ok(lights)
    }

    pub fn set_light_state(&self, light: usize, command: CommandLight) -> Result<Value> {
        let url = format!(
            "http://{}/api/{}/lights/{}/state",
            self.ip,
            self.username.clone().unwrap(),
            light
        );
        let body = ::serde_json::to_vec(&command)?;
        let client = reqwest::Client::new();
        let resp = client
            .put(&url[..])
            .body(::reqwest::Body::from(body))
            .send()?
            .json()?;
        self.parse(resp)
    }

    fn parse<T: ::serde::de::DeserializeOwned>(&self, value: Value) -> Result<T> {
        use serde_json::*;
        if !value.is_array() {
            return Ok(from_value(value)?);
        }
        let mut objects: Vec<Value> = from_value(value)?;
        if objects.len() == 0 {
            Err(::ErrorKind::ProtocolError(
                "expected non-empty array".to_string(),
            ))?
        }
        let value = objects.remove(0);
        {
            let object = value.as_object().ok_or(::ErrorKind::ProtocolError(
                "expected first item to be an object"
                    .to_string(),
            ))?;
            if let Some(e) = object.get("error").and_then(|o| o.as_object()) {
                let code: u64 = e.get("type").and_then(|s| s.as_u64()).unwrap_or(0);
                let desc = e.get("description")
                    .and_then(|s| s.as_str())
                    .unwrap_or("")
                    .to_string();
                Err(::ErrorKind::BridgeError(code as usize, desc))?
            }
        }
        Ok(from_value(value)?)
    }
}
