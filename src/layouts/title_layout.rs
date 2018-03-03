use svgdom::{AttributeId as AId, Node};

use super::layout::{Layout, CalcLayout, DrawLayout};
use super::adaptors::Adaptor;
use font::FontMetrics;

pub struct TitleLayout<'a> {
    pub lay: Layout,
    pub title: &'a str,
}

impl<'a> TitleLayout<'a> {
    pub fn new(title: &'a str) -> TitleLayout<'a> {
        TitleLayout {
            lay: Layout::default(),
            title: title,
        }
    }
}

impl<'a> CalcLayout for TitleLayout<'a> {
    fn calc_layout(&mut self, fm: &FontMetrics) {
        let bbox = fm.text_bbox(&self.title);
        self.lay.size.w = bbox.w;
        self.lay.size.h = fm.full_height();
    }
}

impl<'a> DrawLayout for TitleLayout<'a> {
    fn draw_layout(&self, fm: &FontMetrics, x: u32, y: u32, root: &mut Node) {
        let mut text = root.append_text(self.title, x, y + fm.height(), fm);
        // it can make it bigger than bars layout
        text.set_attribute((AId::FontWeight, "bold"));

        if self.lay.debug {
            let mut r = root.append_rect(x, y, self.lay.size.w, self.lay.size.h);
            r.set_attribute((AId::Stroke, "green"));
        }
    }
}
