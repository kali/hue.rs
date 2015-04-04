use hyper;
use std::convert::From;
use std::error::Error;
use rustc_serialize::json;
use rustc_serialize::{Encoder, Encodable, Decoder, Decodable};
use std::num::ParseIntError;

#[derive(Debug)]
pub struct AppError {
    pub address:String,
    pub description:String,
    pub code:u8
}

impl Encodable for AppError {
  fn encode<S:Encoder>(&self, encoder: &mut S) -> Result<(), S::Error> {
    match *self {
      AppError{  address: ref p_address, description: ref p_description,
                    code:p_code} => {
        encoder.emit_struct("AppError", 0, |encoder| {
          try!(encoder.emit_struct_field( "address", 0, |encoder| p_address.encode(encoder)));
          try!(encoder.emit_struct_field( "description", 1, |encoder| p_description.encode(encoder)));
          try!(encoder.emit_struct_field( "type", 2, |encoder| p_code.encode(encoder)));
          Ok(())
        })
      }
    }
  }
}

impl AppError {
  pub fn dec<S:Decoder>(decoder: &mut S) -> Result<AppError, S::Error> {
    decoder.read_struct("root", 0, |decoder| {
        Ok(AppError{
            address: try!(decoder.read_struct_field("address", 0, |decoder| Decodable::decode(decoder))),
            description: try!(decoder.read_struct_field("description", 1, |decoder| Decodable::decode(decoder))),
            code: try!(decoder.read_struct_field("type", 2, |decoder| Decodable::decode(decoder)))
        })
    })
  }
}

#[derive(Debug)]
pub enum HueError {
    ProtocolError(String),
    BridgeError(AppError),
}

impl HueError {
    pub fn wrap<O> (a:&str) -> ::std::result::Result<O, HueError> {
        Err(HueError::ProtocolError(a.to_string()))
    }
}

impl From<json::EncoderError> for HueError {
    fn from(err: json::EncoderError) -> HueError {
        HueError::ProtocolError(err.description().to_string())
    }
}

impl From<json::DecoderError> for HueError {
    fn from(err: json::DecoderError) -> HueError {
        HueError::ProtocolError(err.description().to_string())
    }
}

impl From<json::ParserError> for HueError {
    fn from(err: json::ParserError) -> HueError {
        HueError::ProtocolError(err.description().to_string())
    }
}

impl From<hyper::HttpError> for HueError {
    fn from(err: hyper::HttpError) -> HueError {
        HueError::ProtocolError(err.description().to_string())
    }
}

impl From<ParseIntError> for HueError {
    fn from(err: ParseIntError) -> HueError {
        HueError::ProtocolError(Error::description(&err).to_string())
    }
}
