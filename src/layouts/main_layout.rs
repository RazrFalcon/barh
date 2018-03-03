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

use svgdom::Node;

use super::bars_layout::BarsLayout;
use super::title_layout::TitleLayout;
use super::haxis_layout::HAxisLayout;
use super::vaxis_layout::VAxisLayout;
use super::layout::{CalcLayout, DrawLayout};
use super::Size;
use font::FontMetrics;
use config;

pub struct MainLayout<'a> {
    size: Size,
    bars_lay: BarsLayout<'a>,
    title_lay: Option<TitleLayout<'a>>,
    haxis_lay: Option<HAxisLayout<'a>>,
    vaxis_lay: VAxisLayout<'a>,
}

impl<'a> MainLayout<'a> {
    pub fn new(config: &'a config::Config<'a>) -> MainLayout<'a> {
        let tl = match config.title {
            Some(title) => Some(TitleLayout::new(title)),
            None => None,
        };

        let hal = match &config.hor_axis {
            &Some(ref axis) => {
                match axis.title {
                    Some(title) => Some(HAxisLayout::new(title)),
                    None => None,
                }
            }
            &None => None,
        };

        MainLayout {
            size: Size::default(),
            bars_lay: BarsLayout::new(&config),
            title_lay: tl,
            haxis_lay: hal,
            vaxis_lay: VAxisLayout::new(&config),
        }
    }

    pub fn set_enable_debug(&mut self, flag: bool) {
        self.bars_lay.lay.debug = flag;
        self.vaxis_lay.lay.debug = flag;

        match &mut self.title_lay {
            &mut Some(ref mut l) => l.lay.debug = flag,
            &mut None => {}
        }

        match &mut self.haxis_lay {
            &mut Some(ref mut l) => l.lay.debug = flag,
            &mut None => {}
        }
    }

    pub fn width(&self) -> u32 {
        self.size.w
    }

    pub fn height(&self) -> u32 {
        self.size.h
    }
}

impl<'a> CalcLayout for MainLayout<'a> {
    fn calc_layout(&mut self, fm: &FontMetrics) {
        self.bars_lay.calc_layout(fm);

        let mut h = 0;

        match &mut self.title_lay {
            &mut Some(ref mut l) => {
                l.calc_layout(fm);
                h += l.lay.size.h;
            }
            &mut None => {}
        }

        // TODO: maybe stretch bars to title width
        h += self.bars_lay.lay.size.h;

        match &mut self.haxis_lay {
            &mut Some(ref mut l) => {
                l.calc_layout(fm);
                l.lay.size.w = self.bars_lay.lay.size.w;
                h += l.lay.size.h;
            }
            &mut None => {}
        }

        for bar in self.bars_lay.bars.iter() {
            self.vaxis_lay.ticks.push(bar.annotation_bbox.y as u32);
        }

        self.vaxis_lay.calc_layout(fm);
        self.vaxis_lay.lay.size.h = self.bars_lay.lay.size.h;

        self.size.w = self.bars_lay.lay.size.w + self.vaxis_lay.lay.size.w;
        self.size.h = h;
    }
}

impl<'a> DrawLayout for MainLayout<'a> {
    fn draw_layout(&self, fm: &FontMetrics, x: u32, y: u32, root: &mut Node) {
        let tx = x + self.vaxis_lay.lay.size.w;
        let mut ty = y;

        match &self.title_lay {
            &Some(ref l) => {
                l.draw_layout(fm, tx + self.bars_lay.lay.margins.left, ty, root);
                ty += l.lay.size.h;
            }
            &None => {}
        }

        // draw after title
        self.vaxis_lay.draw_layout(fm, x, ty, root);

        self.bars_lay.draw_layout(fm, tx, ty, root);
        ty += self.bars_lay.lay.size.h;

        match &self.haxis_lay {
            &Some(ref l) => l.draw_layout(fm, tx, ty, root),
            &None => {}
        }
    }
}
