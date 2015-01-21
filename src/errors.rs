use hyper;
use std::error;
use rustc_serialize::json;
use rustc_serialize::{Encoder, Encodable, Decoder, Decodable};

#[derive(Show)]
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

#[derive(Show)]
pub enum HueError {
    JsonParserError(json::ParserError),
    JsonDecoderError(json::DecoderError),
    HttpError(hyper::HttpError),
    BridgeError(AppError),
    Error(String)
}

impl HueError {
    pub fn wrap<O> (a:&str) -> ::std::result::Result<O, HueError> {
        Err(HueError::Error(a.to_string()))
    }
}

impl error::FromError<json::DecoderError> for HueError {
    fn from_error(err: json::DecoderError) -> HueError {
        HueError::JsonDecoderError(err)
    }
}

impl error::FromError<json::ParserError> for HueError {
    fn from_error(err: json::ParserError) -> HueError {
        HueError::JsonParserError(err)
    }
}

impl error::FromError<hyper::HttpError> for HueError {
    fn from_error(err: hyper::HttpError) -> HueError {
        HueError::HttpError(err)
    }
}

impl error::FromError<::core::fmt::Error> for HueError {
    fn from_error(err: ::core::fmt::Error) -> HueError {
        HueError::Error("core format error".to_string())
    }
}

