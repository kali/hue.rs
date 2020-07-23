error_chain::error_chain! {
    types  { HueError, HueErrorKind, HueResultExt, HueResult; }
    foreign_links {
        Reqwest(reqwest::Error);
        SerdeJson(serde_json::Error);
        AddrParse(std::net::AddrParseError);
        SSDP(ssdp_probe::SsdpProbeError);
    }

    errors {
        ProtocolError(msg: String)
        BridgeError(code: usize, msg: String)
    }
}

pub mod bridge;
mod disco;
