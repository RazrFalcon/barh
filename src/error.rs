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

use std::io;
use std::num;

use serde_json;
use config;

pub type BarhResult<T> = Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    IoError(io::Error),
    NumParseError(num::ParseIntError),
    JsonError(serde_json::Error),
    ConfigError(config::Error),
}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Error {
        Error::IoError(value)
    }
}

impl From<num::ParseIntError> for Error {
    fn from(value: num::ParseIntError) -> Error {
        Error::NumParseError(value)
    }
}

impl From<config::Error> for Error {
    fn from(value: config::Error) -> Error {
        Error::ConfigError(value)
    }
}

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Error {
        Error::JsonError(value)
    }
}
