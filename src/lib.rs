#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]
#![feature(core)]
#![feature(custom_attribute)]

extern crate serde;
extern crate hyper;
extern crate regex;

mod disco;
pub mod errors;
pub mod bridge;
