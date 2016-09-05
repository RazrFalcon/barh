extern crate svgdom;
extern crate serde_json;
extern crate rusttype;

pub use error::{Error, BarhResult};

pub mod font;
pub mod layouts;
pub mod config;
pub mod error;

use std::fs::File;
use std::io::Read;

pub fn load_file(path: &str) -> BarhResult<Vec<u8>> {
    let mut file = try!(File::open(path));

    let length = try!(file.metadata()).len() as usize;

    let mut v = Vec::with_capacity(length + 1);
    try!(file.read_to_end(&mut v));

    Ok(v)
}
