use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use futures::Stream;
use futures::StreamExt;
use reqwest::{Method};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceIdentifier {
    pub rid: String,
    pub rtype: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Device {
    pub id: String,
    pub id_v1: Option<String>,
    pub services: Vec<ResourceIdentifier>,
}

impl Device {
    /// Returns the ids of all services of type light associated with this device.
    pub fn get_lights(&self) -> impl Iterator<Item = &str> {
        self.services
            .iter()
            .filter_map(|service| {
                if service.rtype == "light" {
                    Some(service.rid.as_str())
                } else {
                    None
                }
            })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LightMetadata {
    pub name: String,
    pub archetype: String,
    pub fixed_mired: Option<u16>,
    pub function: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct On {
    pub on: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dimming {
    pub brightness: f32,
    pub min_dim_level: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MirekSchema {
    pub mirek_minimum: u16,
    pub mirek_maximum: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorTemperature {
    pub mirek: Option<u16>,
    pub mirek_valid: bool,
    pub mirek_schema: MirekSchema,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XY {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Gamut {
    pub red: XY,
    pub green: XY,
    pub blue: XY,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Color {
    pub xy: XY,
    pub gamut: Option<Gamut>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Light {
    pub id: String,
    pub id_v1: Option<String>,
    pub metadata: LightMetadata,
    pub service_id: u32,
    pub on: On,
    pub dimming: Option<Dimming>,
    pub color_temperature: Option<ColorTemperature>,
    pub color: Option<Color>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metadata {
    pub name: String,
    pub archetype: String,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Room {
    pub id: String,
    pub id_v1: Option<String>,
    pub metadata: Metadata,
    pub children: Vec<ResourceIdentifier>,
    pub services: Vec<ResourceIdentifier>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResolvedRoom {
    pub id: String,
    pub id_v1: Option<String>,
    pub metadata: Metadata,
    pub children: Vec<Light>,
    pub services: Vec<ResourceIdentifier>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Zone {
    pub id: String,
    pub id_v1: Option<String>,
    pub metadata: Metadata,
    pub children: Vec<ResourceIdentifier>,
    pub services: Vec<ResourceIdentifier>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResolvedZone {
    pub id: String,
    pub id_v1: Option<String>,
    pub metadata: Metadata,
    pub children: Vec<Light>,
    pub services: Vec<ResourceIdentifier>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SceneMetadata {
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Scene {
    pub id: String,
    pub id_v1: Option<String>,
    pub metadata: SceneMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SceneRecall {
    pub action: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandScene {
    recall: SceneRecall,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandLightDimming {
    pub brightness: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandLightColorTemperature {
    pub mirek: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandLightColor {
    pub xy: XY,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CommandLightDynamics {
    #[serde(skip_serializing_if = "Option::is_none")]
    duration: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    speed: Option<f32>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CommandLight {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on: Option<On>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimming: Option<CommandLightDimming>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color_temperature: Option<CommandLightColorTemperature>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<CommandLightColor>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamics: Option<CommandLightDynamics>,
}

impl CommandLight {
    pub fn on(self) -> Self {
        Self {
            on: Some(On { on: true }),
            ..self
        }
    }
    pub fn off(self) -> Self {
        Self {
            on: Some(On { on: false }),
            ..self
        }
    }

    pub fn with_brightness(self, brightness: f32) -> Self {
        Self {
            dimming: Some(CommandLightDimming { brightness }),
            ..self
        }
    }

    pub fn with_mirek(self, mirek: u16) -> Self {
        Self {
            color_temperature: Some(CommandLightColorTemperature { mirek }),
            ..self
        }
    }

    pub fn with_xy(self, x: f32, y: f32) -> Self {
        Self {
            color: Some(CommandLightColor { xy: XY { x, y } }),
            ..self
        }
    }

    pub fn with_transition_time(self, ms: u32) -> Self {
        Self {
            dynamics: Some(CommandLightDynamics { duration: Some(ms), ..Default::default() }),
            ..self
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventColorTemperature {
    pub mirek: Option<u16>,
    pub mirek_valid: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub id: String,
    pub id_v1: Option<String>,
    pub on: Option<On>,
    pub dimming: Option<CommandLightDimming>,
    pub color_temperature: Option<EventColorTemperature>,
    pub color: Option<CommandLightColor>,
}

/// An unauthenticated bridge is a bridge that has not
#[derive(Debug, Clone)]
pub struct UnauthBridge {
    /// The IP-address of the bridge.
    pub ip: std::net::IpAddr,
    client: reqwest::Client,
}

impl UnauthBridge {
    /// Consumes the bridge and returns a new one with a configured username.
    /// ### Example
    /// ```no_run
    /// let bridge = hueclient::Bridge::for_ip([192u8, 168, 0, 4])
    ///     .with_user("rVV05G0i52vQMMLn6BK3dpr0F3uDiqtDjPLPK2uj");
    /// ```
    pub fn with_user(self, username: impl Into<String>) -> Bridge {
        let username = username.into();
        Bridge {
            ip: self.ip,
            client: create_reqwest_client(Some(&username)),
            application_key: username,
        }
    }

    /// This function registers a new application at the provided bridge, using `name` as an
    /// identifier for that app. It returns an error if the button of the bridge was not pressed
    /// shortly before running this function.
    /// ### Example
    /// ```no_run
    /// # tokio_test::block_on(async {
    /// let mut bridge = hueclient::Bridge::for_ip([192u8, 168, 0, 4]);
    /// let auth_bridge = bridge.register_application("mylaptop").await.unwrap();
    /// let key = auth_bridge.application_key;
    /// // now this key can be stored and reused
    /// # })
    /// ```
    pub async fn register_application(self, name: &str) -> crate::Result<Bridge> {
        #[derive(Serialize)]
        struct PostApi {
            devicetype: String,
        }
        #[derive(Debug, Deserialize)]
        struct Username {
            username: String,
        }
        let obtain = PostApi {
            devicetype: name.to_string(),
        };
        let url = format!("https://{}/api", self.ip);
        let resp: BridgeResponse<SuccessResponse<Username>> =
            self.client.post(&url).json(&obtain).send().await?.json().await?;
        let resp = resp.get()?;

        let username = resp.success.username;
        Ok(Bridge {
            ip: self.ip,
            client: create_reqwest_client(Some(&username)),
            application_key: username,
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
    pub application_key: String,
    client: reqwest::Client,
}

fn create_reqwest_client(application_key: Option<&str>) -> reqwest::Client {
    reqwest::Client::builder()
        // see https://developers.meethue.com/develop/application-design-guidance/using-https/
        .add_root_certificate(
            reqwest::Certificate::from_pem(
                b"-----BEGIN CERTIFICATE-----
MIICMjCCAdigAwIBAgIUO7FSLbaxikuXAljzVaurLXWmFw4wCgYIKoZIzj0EAwIw
OTELMAkGA1UEBhMCTkwxFDASBgNVBAoMC1BoaWxpcHMgSHVlMRQwEgYDVQQDDAty
b290LWJyaWRnZTAiGA8yMDE3MDEwMTAwMDAwMFoYDzIwMzgwMTE5MDMxNDA3WjA5
MQswCQYDVQQGEwJOTDEUMBIGA1UECgwLUGhpbGlwcyBIdWUxFDASBgNVBAMMC3Jv
b3QtYnJpZGdlMFkwEwYHKoZIzj0CAQYIKoZIzj0DAQcDQgAEjNw2tx2AplOf9x86
aTdvEcL1FU65QDxziKvBpW9XXSIcibAeQiKxegpq8Exbr9v6LBnYbna2VcaK0G22
jOKkTqOBuTCBtjAPBgNVHRMBAf8EBTADAQH/MA4GA1UdDwEB/wQEAwIBhjAdBgNV
HQ4EFgQUZ2ONTFrDT6o8ItRnKfqWKnHFGmQwdAYDVR0jBG0wa4AUZ2ONTFrDT6o8
ItRnKfqWKnHFGmShPaQ7MDkxCzAJBgNVBAYTAk5MMRQwEgYDVQQKDAtQaGlsaXBz
IEh1ZTEUMBIGA1UEAwwLcm9vdC1icmlkZ2WCFDuxUi22sYpLlwJY81Wrqy11phcO
MAoGCCqGSM49BAMCA0gAMEUCIEBYYEOsa07TH7E5MJnGw557lVkORgit2Rm1h3B2
sFgDAiEA1Fj/C3AN5psFMjo0//mrQebo0eKd3aWRx+pQY08mk48=
-----END CERTIFICATE-----",
            )
            .expect("using rustls and this hardcoded certificate should never fail"),
        )
        // TODO properly handle older bridges that still use a self-signed certificate
        .danger_accept_invalid_certs(true)
        .default_headers({
            let mut headers = reqwest::header::HeaderMap::new();
            if let Some(key) = application_key {
                headers.insert(
                    reqwest::header::HeaderName::from_static("hue-application-key"),
                    reqwest::header::HeaderValue::from_str(key).unwrap(),
                );
            }
            headers
        })
        .connection_verbose(true)
        .build()
        .unwrap()
}

impl Bridge {
    /// Create a bridge at this IP. If you know the IP-address, this is the fastest option. Note
    /// that this function does not validate whether a bridge is really present at the IP-address.
    /// ### Example
    /// ```no_run
    /// let bridge = hueclient::Bridge::for_ip([192u8, 168, 0, 4]);
    /// ```
    pub fn for_ip(ip: impl Into<std::net::IpAddr>) -> UnauthBridge {
        UnauthBridge {
            ip: ip.into(),
            client: create_reqwest_client(None),
        }
    }

    /// Scans the current network for Bridges, and if there is at least one, returns the first one
    /// that was found.
    /// ### Example
    /// ```no_run
    /// let maybe_bridge = hueclient::Bridge::discover();
    /// ```
    pub fn discover() -> Option<UnauthBridge> {
        crate::disco::discover_hue_bridge()
            .ok()
            .map(|ip| UnauthBridge {
                ip,
                client: create_reqwest_client(None),
            })
    }

    /// A convience wrapper around `Bridge::disover`, but panics if there is no bridge present.
    /// ### Example
    /// ```no_run
    /// let brige = hueclient::Bridge::discover_required();
    /// ```
    /// ### Panics
    /// This function panics if there is no brige present.
    pub fn discover_required() -> UnauthBridge {
        Self::discover().expect("No bridge found!")
    }

    /// Consumes the bidge and return a new one with a configured username.
    /// ### Example
    /// ```no_run
    /// let bridge = hueclient::Bridge::for_ip([192u8, 168, 0, 4])
    ///    .with_user("rVV05G0i52vQMMLn6BK3dpr0F3uDiqtDjPLPK2uj");
    /// ```
    pub fn with_application_key(self, appplication_key: impl Into<String>) -> Bridge {
        Bridge {
            ip: self.ip,
            application_key: appplication_key.into(),
            client: self.client,
        }
    }

    /// This function registers a new application at the provided bridge, using `name` as an
    /// identifier for that app. It returns an error if the button of the bridge was not pressed
    /// shortly before running this function.
    /// ### Example
    /// ```no_run
    /// # tokio_test::block_on(async {
    /// let bridge = hueclient::Bridge::for_ip([192u8, 168, 0, 4])
    ///     .register_application("mylaptop")
    ///     .await
    ///     .unwrap();
    /// // now this username d can be stored and reused
    /// println!("the password was {}", bridge.application_key);
    /// # })
    /// ```
    pub async fn register_application(self, name: &str) -> crate::Result<Bridge> {
        #[derive(Serialize)]
        struct PostApi {
            devicetype: String,
        }
        #[derive(Debug, Deserialize)]
        struct Username {
            username: String,
        }
        let obtain = PostApi {
            devicetype: name.to_string(),
        };
        let url = format!("https://{}/api", self.ip);
        let resp: BridgeResponse<SuccessResponse<Username>> =
            self.client.post(&url).json(&obtain).send().await?.json().await?;
        let resp = resp.get()?;

        Ok(Bridge {
            ip: self.ip,
            application_key: resp.success.username,
            client: self.client,
        })
    }

    /// Returns a vector of all devices that are registered at this `Bridge`, sorted by their id's.
    /// This function returns an error if `bridge.username` is `None`.
    ///
    /// ### Example
    /// ```no_run
    /// # tokio_test::block_on(async {
    /// let bridge = hueclient::Bridge::for_ip([192u8, 168, 0, 4])
    ///    .with_user("rVV05G0i52vQMMLn6BK3dpr0F3uDiqtDjPLPK2uj");
    /// for device in &bridge.get_all_devices().await.unwrap() {
    ///     println!("{:?}", device);
    /// }
    /// # })
    /// ```
    pub async fn get_all_devices(&self) -> crate::Result<Vec<Device>> {
        let url = format!("https://{}/clip/v2/resource/device", self.ip);
        let resp: BridgeResponseV2<Device> = self.client.get(&url).send().await?.json().await?;
        let mut devices = resp.get()?;
        devices.sort_by(|a, b| a.id.cmp(&b.id));
        Ok(devices)
    }

    pub async fn index_all_devices(&self) -> crate::Result<HashMap<String, Device>> {
        let devices = self.get_all_devices().await?;
        Ok(devices.into_iter().map(|device| (device.id.clone(), device)).collect())
    }

    /// Returns a vector of all lights that are registered at this `Bridge`, sorted by their id's.
    /// This function returns an error if `bridge.username` is `None`.
    ///
    /// ### Example
    /// ```no_run
    /// # tokio_test::block_on(async {
    /// let bridge = hueclient::Bridge::for_ip([192u8, 168, 0, 4])
    ///    .with_user("rVV05G0i52vQMMLn6BK3dpr0F3uDiqtDjPLPK2uj");
    /// for light in &bridge.get_all_lights().await.unwrap() {
    ///     println!("{:?}", light);
    /// }
    /// # })
    /// ```
    pub async fn get_all_lights(&self) -> crate::Result<Vec<Light>> {
        let url = format!("https://{}/clip/v2/resource/light", self.ip);
        let resp: BridgeResponseV2<Light> = self.client.get(&url).send().await?.json().await?;
        let mut lights = resp.get()?;
        lights.sort_by(|a, b| a.id.cmp(&b.id));
        Ok(lights)
    }

    pub async fn index_all_lights(&self) -> crate::Result<HashMap<String, Light>> {
        let lights = self.get_all_lights().await?;
        Ok(lights.into_iter().fold(
            HashMap::new(),
            |mut map: HashMap<String, Light>, light| {
                map.insert(light.id.clone(), light);
                map
            },
        ))
    }

    /// Returns a vector of all rooms that are registered at this `Bridge`, sorted by their id's.
    /// This function returns an error if `bridge.username` is `None`.
    /// ### Example
    /// ```no_run
    /// # tokio_test::block_on(async {
    /// let bridge = hueclient::Bridge::for_ip([192u8, 168, 0, 4])
    ///    .with_user("rVV05G0i52vQMMLn6BK3dpr0F3uDiqtDjPLPK2uj");
    /// for room in &bridge.get_all_rooms().await.unwrap() {
    ///     println!("{:?}", room);
    /// }
    /// # })
    /// ```
    pub async fn get_all_rooms(&self) -> crate::Result<Vec<Room>> {
        let url = format!("https://{}/clip/v2/resource/room", self.ip);
        let resp: BridgeResponseV2<Room> = self.client.get(&url).send().await?.json().await?;
        let mut groups = resp.get()?;
        groups.sort_by(|a, b| a.id.cmp(&b.id));
        Ok(groups)
    }

    pub async fn resolve_all_rooms(&self) -> crate::Result<Vec<ResolvedRoom>> {
        let rooms = self.get_all_rooms().await?;

        let indexed_devices = self.index_all_devices().await?;
        let indexed_lights = self.index_all_lights().await?;

        Ok(rooms
            .into_iter()
            .map(|room: Room| ResolvedRoom {
                metadata: room.metadata,
                children: room
                    .children
                    .into_iter()
                    .flat_map(|child| {
                        indexed_devices.get(&child.rid).map_or(vec![], |device| {
                            device
                                .get_lights()
                                .filter_map(|light_id| {
                                    indexed_lights.get(light_id).map(|light| light.clone())
                                })
                                .collect()
                        })
                    })
                    .collect(),
                id_v1: room.id_v1,
                id: room.id,
                services: room.services,
            })
            .collect())
    }

    /// Returns a vector of all zones that are registered at this `Bridge`, sorted by their id's.
    /// This function returns an error if `bridge.username` is `None`.
    /// ### Example
    /// ```no_run
    /// # tokio_test::block_on(async {
    /// let bridge = hueclient::Bridge::for_ip([192u8, 168, 0, 4])
    ///    .with_user("rVV05G0i52vQMMLn6BK3dpr0F3uDiqtDjPLPK2uj");
    /// for zone in &bridge.get_all_zones().await.unwrap() {
    ///     println!("{:?}", zone);
    /// }
    /// # })
    /// ```
    pub async fn get_all_zones(&self) -> crate::Result<Vec<Zone>> {
        let url = format!("https://{}/clip/v2/resource/zone", self.ip);
        let resp: BridgeResponseV2<Zone> = self.client.get(&url).send().await?.json().await?;
        let mut groups = resp.get()?;
        groups.sort_by(|a, b| a.id.cmp(&b.id));
        Ok(groups)
    }

    pub async fn resolve_all_zones(&self) -> crate::Result<Vec<ResolvedZone>> {
        let zones = self.get_all_zones().await?;

        let indexed_lights = self.index_all_lights().await?;

        Ok(zones
            .into_iter()
            .map(|zone: Zone| ResolvedZone {
                metadata: zone.metadata,
                children: zone
                    .children
                    .into_iter()
                    .filter_map(|child| indexed_lights.get(&child.rid).map(|light| light.clone()))
                    .collect(),
                id_v1: zone.id_v1,
                id: zone.id,
                services: zone.services,
            })
            .collect())
    }

    /// Returns a vector of all scenes that are registered at this `Bridge`, sorted by their id's.
    /// This function returns an error if `bridge.username` is `None`.
    /// ### Example
    /// ```no_run
    /// # tokio_test::block_on(async {
    /// let bridge = hueclient::Bridge::for_ip([192u8, 168, 0, 4])
    ///    .with_user("rVV05G0i52vQMMLn6BK3dpr0F3uDiqtDjPLPK2uj");
    /// for scene in &bridge.get_all_scenes().await.unwrap() {
    ///     println!("{:?}", scene);
    /// }
    /// # })
    /// ```
    pub async fn get_all_scenes(&self) -> crate::Result<Vec<Scene>> {
        let url = format!("https://{}/clip/v2/resource/scene", self.ip);
        let resp: BridgeResponseV2<Scene> = self.client.get(&url).send().await?.json().await?;
        let mut scenes = resp.get()?;
        scenes.sort_by(|a, b| a.id.cmp(&b.id));
        Ok(scenes)
    }

    pub async fn set_scene(&self, scene: String) -> crate::Result<()> {
        let url = format!("https://{}/clip/v2/resource/scene/{}", self.ip, scene);
        let resp: BridgeResponseV2<Value> = self
            .client
            .put(&url)
            .json(&CommandScene {
                recall: SceneRecall {
                    action: "active".to_string(),
                },
            })
            .send().await?
            .json().await?;
        resp.get()?;

        Ok(())
    }

    pub async fn set_group_state(&self, group: &str, command: &CommandLight) -> crate::Result<()> {
        let url = format!(
            "https://{}/clip/v2/resource/grouped_light/{}",
            self.ip, group
        );
        let resp: BridgeResponseV2<Value> = self.client.put(&url).json(command).send().await?.json().await?;
        resp.get()?;
        Ok(())
    }

    pub async fn set_light_state(&self, light: &str, command: &CommandLight) -> crate::Result<()> {
        let url = format!("https://{}/clip/v2/resource/light/{}", self.ip, light);
        let resp: BridgeResponseV2<Value> = self.client.put(&url).json(&command).send().await?.json().await?;
        resp.get()?;
        Ok(())
    }

    pub fn events(&self) -> crate::Result<impl Stream<Item = HueEvent>> {
        let request_builder = self.client.request(Method::GET, &format!("https://{}/eventstream/clip/v2", self.ip));
        Ok(reqwest_eventsource::EventSource::new(request_builder)?.filter_map( |event| async {
            log::debug!("event {:?}", event);
            match event {
                Ok(reqwest_eventsource::Event::Message(msg)) => {
                    log::debug!("message {:?}", msg.data);
                    match serde_json::from_str::<Vec<EventEnvelope>>(&msg.data) {
                        Ok(mut event) => Some(HueEvent::Event {data : event.pop().unwrap().data}),
                        Err(e) => Some(HueEvent::Error(format!("{:?}", e)))
                    }

                }
                Ok(reqwest_eventsource::Event::Open) => None,
                Err(e) => Some(HueEvent::Error(format!("{:?}", e)))
            }
        }))
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
struct EventEnvelope {
    data: Vec<Event>
}

#[derive(Debug, Clone)]
pub enum HueEvent {
    Event {data : Vec<Event>},
    Error(String)
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
struct BridgeErrorV2 {
    description: String,
}

#[derive(Debug, serde::Deserialize)]
struct BridgeResponseV2<T> {
    errors: Vec<BridgeErrorV2>,
    data: Vec<T>,
}

impl<T> BridgeResponseV2<T> {
    fn get(mut self) -> crate::Result<Vec<T>> {
        if let Some(error) = self.errors.pop() {
            Err(crate::HueError::BridgeErrorV2 {
                description: error.description,
            })
        } else {
            Ok(self.data)
        }
    }
}

#[derive(Debug, serde::Deserialize)]
struct BridgeError {
    error: BridgeErrorInner,
}

#[derive(Debug, serde::Deserialize)]
struct BridgeErrorInner {
    #[allow(dead_code)]
    address: String,
    description: String,
    r#type: usize,
}

#[derive(Debug, serde::Deserialize)]
struct SuccessResponse<T> {
    success: T,
}
