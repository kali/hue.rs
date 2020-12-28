use std::collections::HashMap;
use std::str::FromStr;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct LightState {
    pub on: bool,
    pub bri: Option<u8>,
    pub hue: Option<u16>,
    pub sat: Option<u8>,
    pub ct: Option<u16>,
    pub xy: Option<(f32, f32)>,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
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
    pub xy: Option<(f32, f32)>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transitiontime: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alert: Option<String>,
}

impl Default for CommandLight {
    fn default() -> CommandLight {
        CommandLight {
            on: None,
            bri: None,
            hue: None,
            sat: None,
            transitiontime: None,
            ct: None,
            xy: None,
            alert: None,
        }
    }
}

impl CommandLight {
    pub fn on(self) -> CommandLight {
        CommandLight {
            on: Some(true),
            ..self
        }
    }
    pub fn off(self) -> CommandLight {
        CommandLight {
            on: Some(false),
            ..self
        }
    }
    pub fn with_bri(self, b: u8) -> CommandLight {
        CommandLight {
            bri: Some(b),
            ..self
        }
    }
    pub fn with_hue(self, h: u16) -> CommandLight {
        CommandLight {
            hue: Some(h),
            ..self
        }
    }
    pub fn with_sat(self, s: u8) -> CommandLight {
        CommandLight {
            sat: Some(s),
            ..self
        }
    }
    pub fn with_ct(self, c: u16) -> CommandLight {
        CommandLight {
            ct: Some(c),
            ..self
        }
    }
    pub fn with_xy(self, x: f32, y: f32) -> CommandLight {
        CommandLight {
            xy: Some((x, y)),
            ..self
        }
    }
    pub fn alert(self) -> CommandLight {
        CommandLight {
            alert: Some("select".into()),
            ..self
        }
    }
}

/// An unauthenticated bridge is a bridge that has not
#[derive(Debug, Clone)]
pub struct UnauthBridge {
    /// The IP-address of the bridge.
    pub ip: std::net::IpAddr,
    pub(self) client: reqwest::blocking::Client,
}

impl UnauthBridge {
    /// Consumes the bridge and returns a new one with a configured username.
    /// ### Example
    /// ```rust
    /// let bridge = hueclient::Bridge::for_ip([192u8, 168, 0, 4])
    ///     .with_user("rVV05G0i52vQMMLn6BK3dpr0F3uDiqtDjPLPK2uj");
    /// ```
    pub fn with_user(self, username: impl Into<String>) -> Bridge {
        Bridge {
            ip: self.ip,
            username: username.into(),
            client: self.client,
        }
    }

    /// This function registers a new user at the provided brige, using `devicetype` as an
    /// identifier for that user. It returns an error if the button of the bridge was not pressed
    /// shortly before running this function.
    /// ### Example
    /// ```rust
    /// let mut bridge = hueclient::Bridge::for_ip([192u8, 168, 0, 4]);
    /// let password = bridge.register_user("mylaptop").unwrap();
    /// // now this password can be stored and reused
    /// ```
    pub fn register_user(self, devicetype: &str) -> crate::Result<Bridge> {
        #[derive(Serialize)]
        struct PostApi {
            devicetype: String,
        }
        #[derive(Debug, Deserialize)]
        struct Username {
            username: String,
        }
        let obtain = PostApi {
            devicetype: devicetype.to_string(),
        };
        let url = format!("http://{}/api", self.ip);
        let resp: BridgeResponse<SuccessResponse<Username>> =
            self.client.post(&url).json(&obtain).send()?.json()?;
        let resp = resp.get()?;

        Ok(Bridge {
            ip: self.ip,
            username: resp.success.username,
            client: self.client,
        })
    }
}

/// The bridge is the central access point of the lamps is a Hue setup, and also the central access
/// point of this library.
#[derive(Debug)]
pub struct Bridge {
    /// The IP-address of the bridge.
    pub ip: std::net::IpAddr,
    /// This is the username of the currently logged in user.
    pub username: String,
    pub(self) client: reqwest::blocking::Client,
}

impl Bridge {
    /// Create a bridge at this IP. If you know the IP-address, this is the fastest option. Note
    /// that this function does not validate whether a bridge is really present at the IP-address.
    /// ### Example
    /// ```rust
    /// let bridge = hueclient::Bridge::for_ip([192u8, 168, 0, 4]);
    /// ```
    pub fn for_ip(ip: impl Into<std::net::IpAddr>) -> UnauthBridge {
        UnauthBridge {
            ip: ip.into(),
            client: reqwest::blocking::Client::new(),
        }
    }

    /// Scans the current network for Bridges, and if there is at least one, returns the first one
    /// that was found.
    /// ### Example
    /// ```rust
    /// let maybe_bridge = hueclient::Bridge::discover();
    /// ```
    pub fn discover() -> Option<UnauthBridge> {
        crate::disco::discover_hue_bridge()
            .ok()
            .map(|ip| UnauthBridge {
                ip,
                client: reqwest::blocking::Client::new(),
            })
    }

    /// A convience wrapper around `Bridge::disover`, but panics if there is no bridge present.
    /// ### Example
    /// ```rust
    /// let brige = hueclient::Bridge::discover_required();
    /// ```
    /// ### Panics
    /// This function panics if there is no brige present.
    pub fn discover_required() -> UnauthBridge {
        Self::discover().expect("No bridge found!")
    }

    /// Consumes the bidge and return a new one with a configured username.
    /// ### Example
    /// ```rust
    /// let bridge = hueclient::Bridge::for_ip([192u8, 168, 0, 4])
    ///    .with_user("rVV05G0i52vQMMLn6BK3dpr0F3uDiqtDjPLPK2uj");
    /// ```
    pub fn with_user(self, username: impl Into<String>) -> Bridge {
        Bridge {
            ip: self.ip,
            username: username.into(),
            client: self.client,
        }
    }

    /// This function registers a new user at the provided brige, using `devicetype` as an
    /// identifier for that user. It returns an error if the button of the bridge was not pressed
    /// shortly before running this function.
    /// ### Example
    /// ```rust
    /// let bridge = hueclient::Bridge::for_ip([192u8, 168, 0, 4])
    ///     .bridge.register_user("mylaptop")
    ///     .unwrap();
    /// // now this password can be stored and reused
    /// println!("the password was {}", bridge.password);
    /// ```
    pub fn register_user(self, devicetype: &str) -> crate::Result<Bridge> {
        #[derive(Serialize)]
        struct PostApi {
            devicetype: String,
        }
        #[derive(Debug, Deserialize)]
        struct Username {
            username: String,
        }
        let obtain = PostApi {
            devicetype: devicetype.to_string(),
        };
        let url = format!("http://{}/api", self.ip);
        let resp: BridgeResponse<SuccessResponse<Username>> =
            self.client.post(&url).json(&obtain).send()?.json()?;
        let resp = resp.get()?;

        Ok(Bridge {
            ip: self.ip,
            username: resp.success.username,
            client: self.client,
        })
    }

    /// Returns a vector of all lights that are registered at this `Bridge`, sorted by their id's.
    /// This function returns an error if `bridge.username` is `None`.
    /// ### Example
    /// ```rust
    /// let bridge = hueclient::Bridge::for_ip([192u8, 168, 0, 4])
    ///    .with_user("rVV05G0i52vQMMLn6BK3dpr0F3uDiqtDjPLPK2uj");
    /// for light in &bridge.get_all_lights().unwrap() {
    ///     println!("{:?}", light);
    /// }
    /// ```
    pub fn get_all_lights(&self) -> crate::Result<Vec<IdentifiedLight>> {
        let url = format!("http://{}/api/{}/lights", self.ip, self.username);
        type Resp = BridgeResponse<HashMap<String, Light>>;
        let resp: Resp = self.client.get(&url).send()?.json()?;
        let mut lights = vec![];
        for (k, light) in resp.get()? {
            let id = usize::from_str(&k)
                .map_err(|_| crate::HueError::protocol_err("Light id should be a number"))?;
            lights.push(IdentifiedLight { id, light });
        }
        lights.sort_by(|a, b| a.id.cmp(&b.id));
        Ok(lights)
    }

    pub fn set_light_state(&self, light: usize, command: &CommandLight) -> crate::Result<Value> {
        let url = format!(
            "http://{}/api/{}/lights/{}/state",
            self.ip, self.username, light
        );
        let resp: BridgeResponse<Value> =
            self.client.put(&url).json(command).send()?.json()?;
        resp.get()
    }
}
#[derive(Debug, serde::Deserialize)]
#[serde(untagged)]
enum BridgeResponse<T> {
    Element(T),
    List(Vec<T>),
    Errors(Vec<BridgeError>),
}

impl<T> BridgeResponse<T> {
    fn get(self) -> crate::Result<T> {
        match self {
            BridgeResponse::Element(t) => Ok(t),
            BridgeResponse::List(mut ts) => ts
                .pop()
                .ok_or_else(|| crate::HueError::protocol_err("expected non-empty array")),
            BridgeResponse::Errors(mut es) => {
                // it is safe to unwrap here, since any empty lists will be treated as the
                // `BridgeResponse::List` case.
                let BridgeError { error } = es.pop().unwrap();
                Err(crate::HueError::BridgeError {
                    code: error.r#type,
                    msg: error.description,
                })
            }
        }
    }
}

#[derive(Debug, serde::Deserialize)]
struct BridgeError {
    error: BridgeErrorInner,
}

#[derive(Debug, serde::Deserialize)]
struct BridgeErrorInner {
    address: String,
    description: String,
    r#type: usize,
}

#[derive(Debug, serde::Deserialize)]
struct SuccessResponse<T> {
    success: T,
}
