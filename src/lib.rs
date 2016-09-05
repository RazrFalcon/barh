/****************************************************************************
**
** SVG Cleaner could help you to clean up your SVG files
** from unnecessary data.
** Copyright (C) 2012-2016 Evgeniy Reizner
**
** This program is free software; you can redistribute it and/or modify
** it under the terms of the GNU General Public License as published by
** the Free Software Foundation; either version 2 of the License, or
** (at your option) any later version.
**
** This program is distributed in the hope that it will be useful,
** but WITHOUT ANY WARRANTY; without even the implied warranty of
** MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
** GNU General Public License for more details.
**
** You should have received a copy of the GNU General Public License along
** with this program; if not, write to the Free Software Foundation, Inc.,
** 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA.
**
****************************************************************************/

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
