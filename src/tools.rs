/// Decodes a json value from an `&mut io::Reader`
// borrowed from rustc json lib
use rustc_serialize::json;
use std::old_io as io;

fn io_error_to_error(io: io::IoError) -> json::ParserError {
    json::ParserError::IoError(io.kind, io.desc)
}

pub fn from_reader(rdr: &mut io::Reader) -> Result<json::Json, json::BuilderError> {
    let contents = match rdr.read_to_end() {
        Ok(c)  => c,
        Err(e) => return Err(io_error_to_error(e))
    };
    let s = match ::core::str::from_utf8(contents.as_slice()).ok() {
        Some(s) => s,
        _       => return Err(json::ParserError::SyntaxError(json::ErrorCode::NotUtf8, 0, 0))
    };
    let mut builder = json::Builder::new(s.chars());
    builder.build()
}
