use svgdom::Node;

use font::FontMetrics;
use super::{Size, Margins};

pub trait CalcLayout {
    fn calc_layout(&mut self, font: &FontMetrics);
}

pub trait DrawLayout {
    fn draw_layout(&self, font: &FontMetrics, x: u32, y: u32, root: &mut Node);
}

#[derive(Debug)]
pub struct Layout {
    pub size: Size,
    pub margins: Margins,
    pub debug: bool,
}

impl Default for Layout {
    fn default() -> Layout {
        Layout {
            size: Size::default(),
            margins: Margins::default(),
            debug: false,
        }
    }
}
