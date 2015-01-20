use hyper::Client;
use hyper::client::Body;
use disco;
use rustc_serialize::json;
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

    pub fn obtain_key(&self, devicetype:&str, username:&str) -> Result<String,HueError> {
        #[derive(RustcDecodable, RustcEncodable)]
        struct ObtainKeyRequest {
            devicetype: String, username:String
        }
        let obtain = ObtainKeyRequest {
            devicetype:devicetype.to_string(),
            username:username.to_string()
        };
        println!("{}", json::encode(&obtain));
        let mut client = Client::new();
        let url = format!("http://{}/api", self.ip);
        println!("url: {}", url);
        let reqbody = json::encode(&obtain);
        let mut resp = try!(client.post(url.as_slice())
            .body(Body::BufBody(reqbody.as_bytes(), reqbody.as_bytes().len()))
            .send());
        println!("{:?}", resp.status);
        let json = try!(::tools::from_reader(&mut resp));
        println!("{}", json);
        let objects = json.as_array().unwrap();
        let object = objects[0].as_object().unwrap();
        let obj = object["error".to_string()].as_object();
        match obj {
            Some(e) => {
                let error = e.clone();
                println!("error: {:?}",error);
                let mut decoder = json::Decoder::new(json::Json::Object(error));
                let actualError = AppError::dec(&mut decoder).ok().unwrap();
                println!("actual: {:?}",actualError);
                Err(HueError::BridgeError(actualError))
            },
            None => Ok("blah".to_string())
        }
    }
}


