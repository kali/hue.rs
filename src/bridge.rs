use regex::Regex;

use hyper::Client;
use hyper::client::Body;

use serde_json::{to_string, from_reader, Map, Value};

use errors::HueError;
use ::hue::*;

/// Attempt to discover bridges using `https://www.meethue.com/api/nupnp`
pub fn discover() -> Result<Vec<Discovery>, HueError> {
    let client = Client::new();

    let mut res = try!(client.get("https://www.meethue.com/api/nupnp").send());

    from_reader(&mut res).map_err(From::from)
}

/// A builder object for a `Bridge` object
#[derive(Debug)]
pub struct BridgeBuilder{
    ip: String
}

impl BridgeBuilder{
    /// Starts building a `Bridge` from the given IP
    pub fn from_ip(ip: String) -> Self{
        BridgeBuilder{
            ip: ip
        }
    }
    /// Returns a `Bridge` from an already existing user
    pub fn from_username(self, username: String) -> Bridge {
        let BridgeBuilder{ip} = self;
        Bridge {
            client: Client::new(),
            username: username,
            ip: ip
        }
    }
    /// Registers a new user on the bridge
    pub fn register_user(self, devicetype: &str) -> RegisterIter{
        RegisterIter(Some(self), devicetype)
    }
}

#[derive(Debug)]
/// Iterator that tries to register a new user each iteration
///
/// It will most likely respond with an error saying that the link button needs to be pressed the first time
///
/// ## Example
/// ```no_run
/// use philipshue::errors::{HueError, BridgeError};
///
/// let mut bridge = None;
/// // Discover a bridge
/// let discovery = philipshue::bridge::discover().unwrap().pop().unwrap();
/// let devicetype = "my_hue_app#iphone";
///
/// // Keep trying to register a user
/// for res in discovery.build_bridge().register_user(devicetype){
///     match res{
///         // A new user has succesfully been registered and a `Bridge` object is returned
///         Ok(r) => {
///             bridge = Some(r);
///         },
///         // Prompt the user to press the link button
///         Err(HueError::BridgeError{error: BridgeError::LinkButtonNotPressed, ..}) => {
///             println!("Please, press the link on the bridge. Retrying in 5 seconds");
///             std::thread::sleep(std::time::Duration::from_secs(5));
///         },
///         // Some other error happened
///         Err(e) => {
///             println!("Unexpected error occured: {:?}", e);
///             break
///         }
///     }
/// }
/// ```
pub struct RegisterIter<'a>(Option<BridgeBuilder>, &'a str);

impl<'a> Iterator for RegisterIter<'a> {
    type Item = Result<Bridge, HueError>;
    fn next(&mut self) -> Option<Self::Item>{
        if let Some(bb) = ::std::mem::replace(&mut self.0, None){
            let client = Client::new();

            let body = format!("{{\"devicetype\": {:?}}}", self.1);
            let body = body.as_bytes();
            let url = format!("http://{}/api", bb.ip);
            let mut resp = match client.post(&url)
                .body(Body::BufBody(body, body.len()))
                .send() {
                    Ok(r) => r,
                    Err(e) => return Some(Err(HueError::from(e)))
                };


            let rur = match from_reader::<_, Vec<HueResponse<User>>>(&mut resp) {
                Ok(mut r) => r.pop().unwrap(),
                Err(e) => return Some(Err(HueError::from(e)))
            };

            Some(if let Some(User{username}) = rur.success{
                let BridgeBuilder{ip} = bb;

                Ok(Bridge{
                    ip: ip,
                    client: client,
                    username: username
                })
            }else if let Some(error) = rur.error{
                self.0 = Some(bb);
                Err(error.into())
            }else{
                Err(HueError::MalformedResponse)
            })
        }else{
            None
        }
    }
}

#[derive(Debug)]
/// The bridge connection
pub struct Bridge {
    client: Client,
    /// The IP address of the bridge
    pub ip: String,
    /// The username for the user on the bridge
    pub username: String,
}

impl Bridge {
    /// Gets all lights from the bridge
    pub fn get_all_lights(&self) -> Result<Vec<IdentifiedLight>, HueError> {
        let url = format!("http://{}/api/{}/lights",
                          self.ip,
                          self.username);

        let mut resp = try!(self.client.get(&url).send());
        let json: Map<usize, Light> = try!(from_reader(&mut resp));

        let mut lights: Vec<IdentifiedLight> = try!(json.into_iter()
            .map(|(id, light)| -> Result<IdentifiedLight, HueError> {
                Ok(IdentifiedLight {
                    id: id,
                    light: light,
                })
            })
            .collect());
        lights.sort_by_key(|x| x.id);
        Ok(lights)
    }
    /// Sends a `LightCommand` to set the state of a light
    pub fn set_light_state(&self, light: usize, command: LightCommand) -> Result<Value, HueError> {
        let url = format!("http://{}/api/{}/lights/{}/state",
                          self.ip,
                          self.username,
                          light);
        let body = try!(to_string(&command));
        let re1 = Regex::new("\"[a-z]*\":null,?").unwrap();
        let cleaned1 = re1.replace_all(&body, "");
        let re2 = Regex::new(",\\}").unwrap();
        let cleaned2 = re2.replace_all(&cleaned1, "}");
        let body = cleaned2.as_bytes();

        let mut resp = try!(self.client.put(&url)
            .body(Body::BufBody(body, body.len()))
            .send());

        from_reader(&mut resp).map_err(From::from)
    }
}
