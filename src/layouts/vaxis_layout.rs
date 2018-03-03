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

use std::cmp;

use svgdom::{AttributeId as AId, Node};

use super::layout::{Layout, CalcLayout, DrawLayout};
use super::adaptors::Adaptor;
use font::FontMetrics;
use config;

pub struct VAxisLayout<'a> {
    pub lay: Layout,
    config: &'a config::Config<'a>,
    pub ticks: Vec<u32>,
}

impl<'a> VAxisLayout<'a> {
    pub fn new(config: &'a config::Config<'a>) -> VAxisLayout<'a> {
        VAxisLayout {
            lay: Layout::default(),
            config: config,
            ticks: Vec::new(),
        }
    }
}

impl<'a> CalcLayout for VAxisLayout<'a> {
    fn calc_layout(&mut self, fm: &FontMetrics) {
        let mut max_w = 0;
        for item in &self.config.items {
            let bbox = fm.text_bbox(item.name);
            max_w = cmp::max(bbox.w, max_w);
        }

        self.lay.size.w = max_w + 4;

        // height will be set by MainLayout
    }
}

impl<'a> DrawLayout for VAxisLayout<'a> {
    fn draw_layout(&self, fm: &FontMetrics, x: u32, y: u32, root: &mut Node) {
        debug_assert!(self.lay.size.h > 0);
        // ticks list should be set by MainLayout
        debug_assert!(!self.ticks.is_empty());

        for (item, tick) in self.config.items.iter().zip(self.ticks.iter()) {
            let bbox = fm.text_bbox(item.name);
            let tx = x + self.lay.size.w - bbox.w;
            root.append_text(item.name, tx, y + *tick, fm);

            if self.lay.debug {
                let dy = y + *tick - fm.height();
                let mut r = root.append_rect(tx, dy, bbox.w, fm.full_height());
                r.set_attribute((AId::Stroke, "red"));
            }
        }

        if self.lay.debug {
            let mut r = root.append_rect(x, y, self.lay.size.w, self.lay.size.h);
            r.set_attribute((AId::Stroke, "blue"));
        }
    }
}
