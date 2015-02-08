use hyper::Client;
use hyper::client::Body;
use hyper::client::response::Response;
use disco;
use rustc_serialize::json;
use rustc_serialize::json::Json;
use rustc_serialize::{ Decodable, Encodable };
use errors::HueError;
use errors::AppError;
use regex::Regex;

#[derive(Debug,Copy,Clone,RustcDecodable)]
pub struct LightState {
    pub on: bool,
    pub bri: u8,
    pub hue: u16,
    pub sat: u8
}

#[derive(Debug,Clone,RustcDecodable)]
pub struct Light {
    pub name: String,
    pub modelid: String,
    pub swversion: String,
    pub uniqueid: String,
    pub state: LightState,
}

#[derive(Debug,Clone)]
pub struct IdentifiedLight {
    pub id: usize,
    pub light: Light,
}

#[derive(Debug,Clone,Copy,RustcEncodable,RustcDecodable)]
pub struct CommandLight {
    pub on:Option<bool>,
    pub bri:Option<u8>,
    pub hue:Option<u16>,
    pub sat:Option<u8>,
    pub transitiontime:Option<u16>,
}

impl CommandLight {
    pub fn empty() -> CommandLight {
        CommandLight { on:None, bri:None, hue:None, sat:None, transitiontime:None }
    }
    pub fn on() -> CommandLight {
        CommandLight { on:Some(true), ..CommandLight::empty() }
    }
    pub fn off() -> CommandLight {
        CommandLight { on:Some(false), ..CommandLight::empty() }
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
        disco::discover_hue_bridge().ok().map( |i| Bridge{ ip:i, username:None } )
    }

    pub fn discover_required() -> Bridge {
        Bridge::discover().unwrap_or_else( || panic!("No bridge found!") )
    }

    pub fn with_user(self, username:String) -> Bridge {
        Bridge{ username: Some(username), ..self }
    }

    pub fn register_user(&self, devicetype:&str, username:&str) -> Result<Json,HueError> {
        if username.len() < 10 || username.len() > 40 {
            return HueError::wrap("username must be between 10 and 40 characters")
        }
        #[derive(RustcDecodable, RustcEncodable)]
        struct PostApi {
            devicetype: String,
            username:String
        }
        let obtain = PostApi {
            devicetype:devicetype.to_string(),
            username:username.to_string()
        };
        let body = try!(json::encode(&obtain));
        let mut client = Client::new();
        let url = format!("http://{}/api", self.ip);
        let mut resp = try!(client.post(url.as_slice())
            .body(Body::BufBody(body.as_bytes(), body.as_bytes().len())).send());
        self.parse_write_resp(&mut resp)
    }

    pub fn get_all_lights(&self) -> Result<Vec<IdentifiedLight>,HueError> {
        let url = format!("http://{}/api/{}/lights",
            self.ip, self.username.clone().unwrap());
        let mut client = Client::new();
        let mut resp = try!(client.get(url.as_slice()).send());
        let json = try!(::tools::from_reader(&mut resp));
        let mut lights:Vec<IdentifiedLight> = try!(
            json.as_object().unwrap().iter().map( |(k,v)| {
            let mut decoder = json::Decoder::new(v.clone());
            <Light as Decodable>::decode(&mut decoder).map( |l|
                IdentifiedLight{ id: k.parse().unwrap(), light: l }
            )
        }).collect());
        lights.sort_by( |a,b| a.id.cmp(&b.id) );
        Ok(lights)
    }

    pub fn set_light_state(&self, light:usize, command:CommandLight) -> Result<Json, HueError> {
        let url = format!("http://{}/api/{}/lights/{}/state",
            self.ip, self.username.clone().unwrap(), light);
        let body = try!(json::encode(&command));
        let re1 = Regex::new("\"[a-z]*\":null").unwrap();
        let cleaned1 = re1.replace_all(body.as_slice(),"");
        let re2 = Regex::new(",+").unwrap();
        let cleaned2 = re2.replace_all(cleaned1.as_slice(),",");
        let re3 = Regex::new(",\\}").unwrap();
        let cleaned3 = re3.replace_all(cleaned2.as_slice(),"}");
        let re3 = Regex::new("\\{,").unwrap();
        let cleaned4 = re3.replace_all(cleaned3.as_slice(),"{");
        let mut client = Client::new();
        let mut resp = try!(client.put(url.as_slice())
            .body(Body::BufBody(cleaned4.as_bytes(), cleaned4.as_bytes().len())).send());
        self.parse_write_resp(&mut resp)
    }

    fn parse_write_resp(&self, resp:&mut Response) -> Result<Json,HueError> {
        let json = try!(::tools::from_reader(resp));
        let objects = try!(json.as_array()
            .ok_or(HueError::Error("expected array".to_string())));
        if objects.len() == 0 {
            return HueError::wrap("expected non-empty array");
        }
        let object = try!(objects[0].as_object()
            .ok_or(HueError::Error("expected first item to be an object".to_string())));
        let obj = object.get(&"error".to_string()).and_then( |o| o.as_object() );
        match obj {
            Some(e) => {
                let error = e.clone();
                let mut decoder = json::Decoder::new(json::Json::Object(error));
                let actual_error = AppError::dec(&mut decoder).ok().unwrap();
                //println!("actual: {:?}",actual_error);
                Err(HueError::BridgeError(actual_error))
            },
            None => Ok(json.clone())
        }
    }
}


