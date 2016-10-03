extern crate serde_codegen;

use std::env;
use std::path::Path;

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();

    serde_codegen::expand("src/hue.in.rs", Path::new(&out_dir).join("hue.rs")).unwrap();
}
