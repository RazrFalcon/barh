use std::process::Command;
use std::cmp;

use rusttype;

use super::{Error, BarhResult};
use super::layouts::Rect;
use super::load_file;

pub struct FontMetrics<'a> {
    family: String,
    font: rusttype::Font<'a>,
    size: u8,
    height: u32,
}

impl<'a> FontMetrics<'a> {
    pub fn from_font(family: &'a str, size: u8) -> BarhResult<FontMetrics<'a>> {
        let f = load_font(family)?;

        let scale = rusttype::Scale::uniform(size as f32 * 1.65);
        let vm = f.v_metrics(scale);

        Ok(FontMetrics {
            family: family.to_string(),
            font: f,
            size: size,
            height: (vm.ascent + vm.descent.abs()).round() as u32,
        })
    }

    pub fn text_width(&self, text: &str) -> u32 {
        self.text_bbox(text).w
    }

    pub fn text_bbox(&self, text: &str) -> Rect {
        let scale = rusttype::Scale::uniform(self.height() as f32);
        let start = rusttype::Point { x: 0.0, y: 0.0 };

        let mut w = 0;
        let mut h = 0;
        let mut min_y = 0;
        let mut last_w = 0;
        for pg in self.font.layout(text, scale, start) {
            match pg.pixel_bounding_box() {
                Some(bbox) => {
                    last_w = bbox.width();
                    min_y = cmp::min(min_y, -bbox.max.y);
                    h = cmp::max(h, bbox.max.y + bbox.height());
                    w = bbox.min.x;
                }
                None => {}
            }
        }

        Rect {
            x: 0,
            y: min_y,
            // Make it a bit bigger since it's usualy a bit smaller than actual text width.
            // Probably because of hinting.
            w: (((w + last_w) as f32) * 1.12) as u32,
            h: h as u32,
        }
    }

    pub fn family(&self) -> &str {
        &self.family
    }

    pub fn full_height(&self) -> u32 {
        self.height
    }

    pub fn height(&self) -> u32 {
        // 90 dpi
        (self.size as f32 * 1.3333).round() as u32
    }

    pub fn font_size(&self) -> u8 {
        self.size
    }
}

fn load_font<'a>(family: &str) -> BarhResult<rusttype::Font<'a>> {
    let path = find_font_file(family)?;

    let data = load_file(&path)?;
    let fc = rusttype::FontCollection::from_bytes(data);

    Ok(fc.into_font().unwrap())
}

#[derive(Clone, PartialEq, Debug)]
pub struct FontData {
    pub family: String,
    pub size: u8,
    pub path: String,
}

impl FontData {
    pub fn system_font() -> BarhResult<FontData> {
        let family = find_default_font_family()?;
        Ok(FontData {
            size: find_font_size(&family)?,
            path: find_font_file(&family)?,
            family: family,
        })
    }

    pub fn font_path(family: &str) -> BarhResult<String> {
        find_font_file(family)
    }

    pub fn is_font_exist(family: &str) -> BarhResult<bool> {
        is_font_exist(family)
    }
}

fn find_font_file(family: &str) -> BarhResult<String> {
    run_fc_match(&["--format=%{file}", family])
}

fn find_font_size(family: &str) -> BarhResult<u8> {
    let s = run_fc_match(&["--format=%{size}", family])?;
    Ok(s.parse::<u8>()?)
}

fn find_default_font_family() -> BarhResult<String> {
    run_fc_match(&["--format=%{family}"])
}

fn is_font_exist(family: &str) -> BarhResult<bool> {
    let res = Command::new("fc-list").arg("--format=%{family}\n").output();
    match res {
        Ok(o) => {
            let s = String::from_utf8_lossy(&o.stdout).into_owned();
            Ok(s.lines().any(|x| x == family))
        }
        Err(e) => Err(Error::from(e)),
    }
}

fn run_fc_match(args: &[&str]) -> BarhResult<String> {
    let res = Command::new("fc-match").args(args).output();
    match res {
        Ok(o) => {
            let s = String::from_utf8_lossy(&o.stdout);
            return Ok(s.into_owned());
        }
        Err(e) => return Err(Error::from(e)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn system_font_1() {
        let fd = FontData::system_font().unwrap();
        assert_eq!(fd,
            FontData {
                family: "Verdana".to_string(),
                size: 12, // TODO: actually 11, fc bug
                path: "/usr/share/fonts/corefonts/verdana.ttf".to_string(),
            });
    }
}
