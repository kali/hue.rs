use hyper::Client;
use hyper::client::Body;
use disco;
use rustc_serialize::json;
use rustc_serialize::json::Json;
use errors::HueError;
use errors::AppError;

#[derive(Show)]
pub struct Bridge {
    ip: String
}

impl Bridge {
    #[allow(dead_code)]
    pub fn discover() -> Option<Bridge> {
        disco::discover_hue_bridge().ok().map( |i| Bridge{ ip:i } )
    }

    pub fn discover_required() -> Bridge {
        Bridge::discover().unwrap_or_else( || panic!("No bridge found!"))
    }

    pub fn register_user(&self, devicetype:&str, username:&str) -> Result<String,HueError> {
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
        //println!("{}", json::encode(&obtain));
        let b = try!(self.post("/api", json::encode(&obtain)));
        Ok(b.to_string())
    }

    pub fn post(&self, path:&str, body:String) -> Result<Json,HueError> {
        let mut client = Client::new();
        let url = format!("http://{}{}", self.ip, path);
        //println!("url: {}", url);
        let mut resp = try!(client.post(url.as_slice())
            .body(Body::BufBody(body.as_bytes(), body.as_bytes().len()))
            .send());
        let json = try!(::tools::from_reader(&mut resp));
        //println!("{}", json);
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


