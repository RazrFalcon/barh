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

use serde_json;
use serde_json::Value;

type JSMap = serde_json::Map<String, Value>;

use font::FontData;

static DEFAULT_BAR_COLOR: &'static str = "#3260cd";

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Item<'a> {
    pub name: &'a str,
    pub value: f64,
    pub color: &'a str,
}

#[derive(PartialEq, Debug)]
pub struct HorAxis<'a> {
    pub title: Option<&'a str>,
    pub suffix: Option<&'a str>,
    pub max_value: Option<f64>,
    pub ticks: Option<Vec<f64>>,
    pub width: Option<u32>,
}

#[cfg(test)]
impl<'a> Default for HorAxis<'a> {
    fn default() -> HorAxis<'a> {
        HorAxis {
            title: None,
            suffix: None,
            max_value: None,
            ticks: None,
            width: None,
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct Config<'a> {
    pub title: Option<&'a str>,
    pub items: Vec<Item<'a>>,
    pub debug: bool,
    pub items_font: FontData,
    pub hor_axis: Option<HorAxis<'a>>,
}

#[derive(PartialEq, Debug)]
pub enum Error {
    ItemsIsNotSet,
    ItemNameIsNotSet,
    ItemValueIsNotSet,
    VelueMustBePositive,
    CouldNotResolveFontPath,
    InvalidValueType(&'static str, &'static str), // value path, expected type
}

// TODO: Error Display

macro_rules! value_option {
    ($obj:expr, $name:expr, $value_type:ident) => ({
        match $obj.get($name) {
            Some(v) => {
                match *v {
                    Value::$value_type(ref t) => Some(t),
                    _ => None,
                }
            }
            None => None,
        }
    })
}

macro_rules! get_str {
    ($obj:expr, $name:expr) => ({
        match $obj.get($name) {
            Some(v) => v.as_str(),
            None => None,
        }
    })
}

macro_rules! get_num {
    ($obj:expr, $name:expr) => ({
        match $obj.get($name) {
            Some(v) => v.as_f64(),
            None => None,
        }
    })
}

impl<'a> Config<'a> {
    pub fn from_value(value: &'a Value, system_font: &FontData) -> Result<Config<'a>, Error> {
        let conf = match value.as_object() {
            Some(c) => c,
            None => return Err(Error::ItemsIsNotSet),
        };

        let items_obj = match conf.get("items") {
            Some(v) => {
                match *v {
                    Value::Array(ref t) => t,
                    _ => return Err(Error::InvalidValueType("/items", "Array")),
                }
            }
            None => return Err(Error::ItemsIsNotSet),
        };

        if items_obj.len() == 0 {
            return Err(Error::ItemsIsNotSet);
        }

        let items = try!(parse_items(items_obj));

        let items_font = try!(parse_font(conf, "items_font", system_font));

        let hor_axis = match conf.get("hor_axis") {
            Some(h) => Some(try!(parse_hor_axis(h))),
            None => None,
        };

        Ok(Config {
            title: get_str!(conf, "title"),
            items: items,
            debug: *value_option!(conf, "debug", Bool).unwrap_or(&false),
            items_font: items_font,
            hor_axis: hor_axis,
        })
    }
}

#[cfg(test)]
impl<'a> Default for Config<'a> {
    fn default() -> Config<'a> {
        Config {
            title: None,
            items: Vec::new(),
            debug: false,
            items_font: FontData::system_font().unwrap(),
            hor_axis: None,
        }
    }
}

fn parse_items(items: &Vec<Value>) -> Result<Vec<Item>, Error> {
    let mut v = Vec::new();

    for item in items {
        let obj = match item {
            &Value::Object(ref t) => t,
            _ => return Err(Error::InvalidValueType("/items/n", "Object")),
        };

        let name = match obj.get("name") {
            Some(v) => {
                match v.as_str() {
                    Some(s) => s,
                    None => return Err(Error::InvalidValueType("/items/n/name", "String")),
                }
            }
            None => return Err(Error::ItemNameIsNotSet),
        };

        let value = match obj.get("value") {
            Some(v) => {
                match *v {
                    Value::Number(ref t) => t.as_f64().unwrap(),
                    _ => return Err(Error::InvalidValueType("/items/n/value", "Number")),
                }
            }
            None => return Err(Error::ItemValueIsNotSet),
        };

        if value.is_sign_negative() {
            return Err(Error::VelueMustBePositive);
        }

        v.push(Item {
            name: name,
            value: value,
            color: get_str!(obj, "color").unwrap_or(DEFAULT_BAR_COLOR),
        });
    }

    Ok(v)
}

fn parse_font(value: &JSMap, name: &str, system_font: &FontData) -> Result<FontData, Error> {
    match value_option!(value, name, Object) {
        Some(font_obj) => {
            let family = match font_obj.get("family") {
                Some(v) => {
                    match v.as_str() {
                        Some(s) => {
                            if FontData::is_font_exist(s).unwrap() {
                                s
                            } else {
                                return Err(Error::CouldNotResolveFontPath);
                            }
                        }
                        None => return Err(Error::InvalidValueType("/*_font/family", "String")),
                    }
                },
                None => &system_font.family,
            };

            let size = get_num!(font_obj, "size").unwrap_or(system_font.size as f64);

            let path = match FontData::font_path(family) {
                Ok(p) => p,
                Err(_) => return Err(Error::CouldNotResolveFontPath),
            };

            Ok(FontData {
                family: family.to_string(),
                size: size as u8,
                path: path,
            })
        }
        None => {
            Ok(system_font.clone())
        }
    }
}

fn parse_hor_axis(value: &Value) -> Result<HorAxis, Error> {
    let ha = match value.as_object() {
        Some(c) => c,
        None => return Err(Error::InvalidValueType("/hor_axis", "Map")),
    };

    let ticks = match ha.get("ticks") {
        Some(v) => {
            match v.as_array() {
                Some(a) => {
                    let mut list: Vec<f64> = Vec::new();
                    for n in a {
                        list.push(n.as_f64().unwrap());
                    }
                    Some(list)
                },
                None => None,
            }
        }
        None => None,
    };

    let width = match ha.get("width") {
        Some(w) => {
            match w.as_u64() {
                Some(v) => Some(v as u32),
                None => None,
            }
        }
        None => None,
    };

    // TODO: this macros didn't check value types, which is bad
    Ok(HorAxis {
        title: get_str!(ha, "title"),
        suffix: get_str!(ha, "suffix"),
        max_value: get_num!(ha, "max_value"),
        ticks: ticks,
        width: width,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::DEFAULT_BAR_COLOR;
    use font::FontData;
    use serde_json;

    static DEFAULT_ITEM: &'static Item<'static> = &Item {
        name: "some name",
        value: 42.0,
        color: "#3260cd", // we can't use DEFAULT_BAR_COLOR here
    };

    macro_rules! test {
        ($name:ident, $data:expr, $result:expr) => (
            #[test]
            fn $name() {
                let value: serde_json::Value = serde_json::from_slice($data).unwrap();
                let sf = FontData::system_font().unwrap();
                assert_eq!(Config::from_value(&value, &sf).unwrap(), $result);
            }
        )
    }

    macro_rules! test_err {
        ($name:ident, $data:expr, $err:expr) => (
            #[test]
            fn $name() {
                let value: serde_json::Value = serde_json::from_slice($data).unwrap();
                let sf = FontData::system_font().unwrap();
                assert_eq!(Config::from_value(&value, &sf).err().unwrap(), $err);
            }
        )
    }

    test_err!(empty_1, b"{}", Error::ItemsIsNotSet);
    test_err!(empty_2, b"{ \"items\": [] }", Error::ItemsIsNotSet);
    test_err!(invalid_item_1, b"{ \"items\": [ {} ] }", Error::ItemNameIsNotSet);

    test_err!(invalid_item_2,
        b"{
            \"items\": [
                {
                    \"name\": \"some name\"
                }
            ]
        }",
        Error::ItemValueIsNotSet);

    test_err!(invalid_item_3,
        b"{
            \"items\": [
                {
                    \"name\": \"some name\",
                    \"value\": \"0\"
                }
            ]
        }",
        Error::InvalidValueType("/items/n/value", "Number"));

    test_err!(invalid_item_4,
        b"{
            \"items\": [
                {
                    \"name\": \"some name\",
                    \"value\": -1
                }
            ]
        }",
        Error::VelueMustBePositive);

    test!(minimal_1,
        b"{
            \"items\": [
                {
                    \"name\": \"some name\",
                    \"value\": 42
                }
            ]
        }",
        Config {
            items: vec![*DEFAULT_ITEM],
            ..Config::default()
        });

    test!(minimal_2,
        b"{
            \"items\": [
                {
                    \"name\": \"some name\",
                    \"value\": 42
                },
                {
                    \"name\": \"other name\",
                    \"value\": 142.5
                }
            ]
        }",
        Config {
            items: vec![
                *DEFAULT_ITEM,
                Item {
                    name: "other name",
                    value: 142.5,
                    color: DEFAULT_BAR_COLOR,
                }
            ],
            ..Config::default()
        });

    test!(custom_item_color_1,
        b"{
            \"items\": [
                {
                    \"name\": \"some name\",
                    \"value\": 42,
                    \"color\": \"red\"
                },
                {
                    \"name\": \"other name\",
                    \"value\": 142.5
                }
            ]
        }",
        Config {
            items: vec![
                Item {
                    name: "some name",
                    value: 42.0,
                    color: "red",
                },
                Item {
                    name: "other name",
                    value: 142.5,
                    color: DEFAULT_BAR_COLOR,
                }
            ],
            ..Config::default()
        });

    test!(title_1,
        b"{
            \"title\": \"hi!\",
            \"items\": [{ \"name\": \"some name\",\"value\": 42}]
        }",
        Config {
            title: Some("hi!"),
            items: vec![*DEFAULT_ITEM],
            ..Config::default()
        });

    test!(debug_1,
        b"{
            \"debug\": true,
            \"items\": [{ \"name\": \"some name\",\"value\": 42}]
        }",
        Config {
            items: vec![*DEFAULT_ITEM],
            debug: true,
            ..Config::default()
        });

    // it's pointless, since debug is 'false' by default,
    // but we have to check that it processed correctly
    test!(debug_2,
        b"{
            \"debug\": false,
            \"items\": [{ \"name\": \"some name\",\"value\": 42}]
        }",
        Config {
            items: vec![*DEFAULT_ITEM],
            debug: false,
            ..Config::default()
        });

    test!(items_font_1,
        b"{
            \"items\": [{ \"name\": \"some name\",\"value\": 42}],
            \"items_font\": {
                \"family\": \"Arial\",
                \"size\": 16
            }
        }",
        Config {
            items: vec![*DEFAULT_ITEM],
            items_font: FontData {
                family: "Arial".to_string(),
                size: 16,
                path: FontData::font_path("Arial").unwrap(),
            },
            ..Config::default()
        });

    test_err!(invalid_font_1,
        b"{
            \"items\": [{ \"name\": \"some name\",\"value\": 42}],
            \"items_font\": {
                \"family\": \"Ari2132al\",
                \"size\": 16
            }
        }", Error::CouldNotResolveFontPath);

    test!(haxis_title_1,
        b"{
            \"items\": [{ \"name\": \"some name\",\"value\": 42}],
            \"hor_axis\": {
                \"title\": \"Title\"
            }
        }",
        Config {
            items: vec![*DEFAULT_ITEM],
            hor_axis: Some(HorAxis {
                title: Some("Title"),
                ..HorAxis::default()
            }),
            ..Config::default()
        });

    test!(haxis_suffix_1,
        b"{
            \"items\": [{ \"name\": \"some name\",\"value\": 42}],
            \"hor_axis\": {
                \"suffix\": \"ms\"
            }
        }",
        Config {
            items: vec![*DEFAULT_ITEM],
            hor_axis: Some(HorAxis {
                suffix: Some("ms"),
                ..HorAxis::default()
            }),
            ..Config::default()
        });

    test!(haxis_max_value_1,
        b"{
            \"items\": [{ \"name\": \"some name\",\"value\": 42}],
            \"hor_axis\": {
                \"max_value\": 100
            }
        }",
        Config {
            items: vec![*DEFAULT_ITEM],
            hor_axis: Some(HorAxis {
                max_value: Some(100.0),
                ..HorAxis::default()
            }),
            ..Config::default()
        });

    test!(haxis_ticks_1,
        b"{
            \"items\": [{ \"name\": \"some name\",\"value\": 42}],
            \"hor_axis\": {
                \"ticks\": [10, 20, 30]
            }
        }",
        Config {
            items: vec![*DEFAULT_ITEM],
            hor_axis: Some(HorAxis {
                ticks: Some(vec![10.0, 20.0, 30.0]),
                ..HorAxis::default()
            }),
            ..Config::default()
        });

    test!(haxis_width_1,
        b"{
            \"items\": [{ \"name\": \"some name\",\"value\": 42}],
            \"hor_axis\": {
                \"width\": 400
            }
        }",
        Config {
            items: vec![*DEFAULT_ITEM],
            hor_axis: Some(HorAxis {
                width: Some(400),
                ..HorAxis::default()
            }),
            ..Config::default()
        });
}
