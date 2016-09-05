use svgdom::{AttributeId as AId, Node};

use super::layout::{Layout, CalcLayout, DrawLayout};
use super::adaptors::Adaptor;
use font::FontMetrics;

pub struct HAxisLayout<'a> {
    pub lay: Layout,
    title: &'a str,
    title_width: u32,
}

impl<'a> HAxisLayout<'a> {
    pub fn new(title: &'a str) -> HAxisLayout<'a> {
        HAxisLayout {
            lay: Layout::default(),
            title: title,
            title_width: 0,
        }
    }
}

impl<'a> CalcLayout for HAxisLayout<'a> {
    fn calc_layout(&mut self, fm: &FontMetrics) {
        let bbox = fm.text_bbox(&self.title);
        self.lay.size.h = fm.full_height();
        self.title_width = bbox.w;
    }
}

impl<'a> DrawLayout for HAxisLayout<'a> {
    fn draw_layout(&self, fm: &FontMetrics, x: u32, y: u32, root: &Node) {
        // should be set by MainLayout
        debug_assert!(self.lay.size.w > 0);

        let tx = x + (self.lay.size.w - self.title_width) / 2;
        let text = root.append_text(self.title, tx, y + fm.height(), fm);
        text.set_attribute(AId::FontStyle, "italic");

        if self.lay.debug {
            let r = root.append_rect(x, y, self.lay.size.w, self.lay.size.h);
            r.set_attribute(AId::Stroke, "green");
        }
    }
}
