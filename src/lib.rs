#[macro_use]
extern crate error_chain;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate reqwest;

error_chain! {
    foreign_links {
        Reqwest(::reqwest::Error);
        SerdeJson(::serde_json::Error);
    }

    errors {
        ProtocolError(msg: String)
        BridgeError(code: usize, msg: String)
    }
}

mod disco;
pub mod bridge;
