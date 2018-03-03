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
use super::Rect;
use font::FontMetrics;
use config;

static FIRST_TICK_COLOR: &'static str = "#333";
static OTHER_TICKS_COLOR: &'static str = "#ccc";
static TICKS_TEXT_COLOR: &'static str = "#000";
static TICKS_COUNT: u32 = 5;
static ANNOTATION_BORDER_FACTOR: f32 = 0.2;
static ANNOTATION_TEXT_COLOR: &'static str = "#fff";
static ANNOTATION_TEXT_COLOR_ALT: &'static str = "#000";
static ANNOTATION_HANDLE_COLOR: &'static str = "#999";

pub struct Bar {
    r: Rect,
    annotation: String,
    pub annotation_bbox: Rect,
}

pub struct Tick {
    pos: u32,
    value: f64,
    title: String,
    bbox: Rect,
}

pub struct BarsLayout<'a> {
    pub lay: Layout,
    config: &'a config::Config<'a>,
    max_value: f64,
    item_height: u32,
    pub bars: Vec<Bar>,
    ticks: Vec<Tick>,
}

impl<'a> BarsLayout<'a> {
    pub fn new(config: &'a config::Config<'a>) -> BarsLayout<'a> {
        BarsLayout {
            lay: Layout::default(),
            config: config,
            max_value: 0.0,
            item_height: 0,
            bars: Vec::new(),
            ticks: Vec::new(),
        }
    }
}

impl<'a> CalcLayout for BarsLayout<'a> {
    fn calc_layout(&mut self, fm: &FontMetrics) {
        let values = self.config.items.iter().map(|x| x.value).collect::<Vec<f64>>();
        let max = values.iter().cloned().fold(0./0., f64::max);

        self.max_value = match &self.config.hor_axis {
            &Some(ref ha) => {
                match ha.max_value {
                    Some(mv) => mv,
                    None => calc_max_value(max),
                }
            }
            &None => calc_max_value(max),
        };

        self.item_height = (fm.height() as f32 * (1.0 + ANNOTATION_BORDER_FACTOR * 2.0)) as u32;

        self.lay.size.h =   (self.config.items.len() as u32) * self.item_height
                          + (self.config.items.len() as u32 + 1) * (self.item_height / 2);

        // get hor axis suffix
        let suffix = match &self.config.hor_axis {
            &Some(ref axis) => {
                match axis.suffix {
                    Some(s) => s,
                    None => "",
                }
            }
            &None => "",
        };

        fn gen_ticks_list(max_value: f64, count: u32) -> Vec<f64> {
            let mut v = Vec::new();
            let num_step = max_value / (count - 1) as f64;
            let mut n = 0.0;
            for _ in 0..count {
                v.push(n);
                n += num_step;
            }

            v
        }

        // calc ticks text
        let ticks_value = match &self.config.hor_axis {
            &Some(ref axis) => {
                match axis.ticks {
                    Some(ref l) => l.clone(),
                    None => gen_ticks_list(self.max_value, TICKS_COUNT),
                }
            }
            &None => gen_ticks_list(self.max_value, TICKS_COUNT),
        };

        {
            for n in ticks_value {
                let text1 = n.to_string() + suffix;
                let bbox1 = fm.text_bbox(&text1);

                self.ticks.push(Tick {
                    pos: 0, // will be calculated later
                    value: n,
                    title: text1,
                    bbox: bbox1,
                });
            }
        }

        // first tick is always '0'
        let min_v_bbox = fm.text_bbox(&("0".to_string() + suffix));
        let min_text_w = min_v_bbox.w;

        // find widest tick text
        let mut max_text_w = 0;
        for tick in &self.ticks {
            let max_v_bbox = fm.text_bbox(&tick.title);
            max_text_w = cmp::max(max_v_bbox.w, max_text_w);
        }
        // and max value
        {
            let max_v_bbox = fm.text_bbox(&(self.max_value.to_string() + suffix));
            max_text_w = cmp::max(max_v_bbox.w, max_text_w);
        }
        let max_text_h = fm.full_height();

        // TODO: move spacing to options
        self.lay.size.w = match &self.config.hor_axis {
            &Some(ref axis) => {
                match axis.width {
                    Some(ref w) => *w,
                    None => max_text_w * (self.ticks.len() as u32 * 3),
                }
            }
            &None => max_text_w * (self.ticks.len() as u32 * 3),
        };

        let text_factor = 2.0;
        self.lay.margins.left = (min_text_w as f32 / text_factor) as u32;
        self.lay.margins.right = (max_text_w as f32 / text_factor) as u32;
        self.lay.margins.top = max_text_h / 2;
        self.lay.margins.bottom = max_text_h as u32;

        self.lay.size.h += self.lay.margins.top + self.lay.margins.bottom;

        let r2 = self.lay.size.into_rect(0, 0).adjusted(&self.lay.margins);
        let scale_factor = r2.w as f64 / self.max_value as f64;

        // calc ticks pos
        {
            for tick in self.ticks.iter_mut() {
                let x = (tick.value * scale_factor) as u32;
                tick.pos = r2.x as u32 + x;
            }
        }

        // calc bars
        let mut y = (self.item_height as i32 / 2) + r2.y;
        for item in &self.config.items {
            let w = (item.value * scale_factor) as u32;

            let annotation = format!("{}", item.value);
            let text_bbox = fm.text_bbox(&annotation);
            let ann_border = (self.item_height as f32 * ANNOTATION_BORDER_FACTOR) as u32;
            let ann_handle_w = (fm.height() as f32 * 0.75) as u32;

            let ann_bbox;
            if text_bbox.w + ann_border * 2 < w {
                // can be written inside the bar
                let tx = r2.x as u32 + w - text_bbox.w - ann_border;
                let ty = y as u32 + text_bbox.h + ann_border;

                ann_bbox = Rect::new(tx as i32, ty as i32, text_bbox.w, text_bbox.h);
            } else {
                // should be written outside
                let x = r2.x as u32 + w;
                let tx = x + ann_handle_w + 2;
                let ty = y as u32 + text_bbox.h + ann_border;

                ann_bbox = Rect::new(tx as i32, ty as i32, text_bbox.w, text_bbox.h);
            }

            self.bars.push(Bar {
                r: Rect::new(r2.x as i32, y as i32, w, self.item_height),
                annotation: annotation,
                annotation_bbox: ann_bbox,
            });

            y += (self.item_height as f32 * 1.5) as i32;
        }
    }
}

impl<'a> DrawLayout for BarsLayout<'a> {
    fn draw_layout(&self, fm: &FontMetrics, x: u32, y: u32, root: &mut Node) {
        if self.lay.debug {
            let mut lay_rect = root.append_rect(x, y, self.lay.size.w, self.lay.size.h);
            lay_rect.set_attribute((AId::Stroke, "red"));
        }

        let r2 = self.lay.size.into_rect(x as i32, y as i32).adjusted(&self.lay.margins);
        if self.lay.debug {
            let mut lay_rect = root.append_rect(r2.x as u32, r2.y as u32, r2.w, r2.h);
            lay_rect.set_attribute((AId::Stroke, "green"));
        }

        let tx = fm.height();

        for tick in self.ticks.iter() {
            // draw tick line
            let mut tick_vl = root.append_vline(x + tick.pos, r2.y as u32, r2.h);
            tick_vl.set_attribute((AId::Fill, OTHER_TICKS_COLOR));

            let ty = r2.y as u32 + r2.h + tx - 2;
            let tx = (x + tick.pos - tick.bbox.w / 2) as u32;
            let mut text_node1 = root.append_text(&tick.title, tx, ty, fm);
            text_node1.set_attribute((AId::Fill, TICKS_TEXT_COLOR));

            if self.lay.debug {
                let mut rect = root.append_rect(tx, ty - tick.bbox.h, tick.bbox.w, tick.bbox.h);
                rect.set_attribute((AId::Stroke, "red"));
            }
        }

        // draw bars
        for (item, bar) in self.config.items.iter().zip(self.bars.iter()) {
            let mut rect = root.append_rect(bar.r.x as u32 + x, bar.r.y as u32 + y, bar.r.w, bar.r.h);
            rect.set_attribute((AId::Fill, item.color));

            let ann_color;
            if bar.r.right() > bar.annotation_bbox.right() {
                ann_color = ANNOTATION_TEXT_COLOR;
            } else {
                ann_color = ANNOTATION_TEXT_COLOR_ALT;

                // draw handle
                let hx = x + bar.r.right();
                let hy = y + bar.r.y as u32 + self.item_height / 2;
                let hw = bar.annotation_bbox.x as u32 - bar.r.right() - 2;
                let mut hline = root.append_hline(hx, hy, hw);
                hline.set_attribute((AId::Fill, ANNOTATION_HANDLE_COLOR));
            }

            let mut text_node = root.append_text(&bar.annotation, x + bar.annotation_bbox.x as u32,
                                             y + bar.annotation_bbox.y as u32, fm);
            text_node.set_attribute((AId::Fill, ann_color));
        }

        // first tick should be drawn last, so it will be above bars
        let mut first_tick_vl = root.append_vline(r2.x as u32, r2.y as u32, r2.h);
        first_tick_vl.set_attribute((AId::Fill, FIRST_TICK_COLOR));
    }
}

fn calc_max_value(value: f64) -> f64 {
    // this function trying to mimic 'google charts' algorithm

    debug_assert!(value.is_sign_positive());
    debug_assert!(value != 0.0);

    let mut step = 1.0;
    let mut v = value;

    while v <= 100.0 {
        step *= 10.0;
        v *= 10.0;
    }

    while v >= 1000.0 {
        step *= 0.1;
        v *= 0.1;
    }

    let v1 = v.round() as u32;

    let v2 = match v1 {
        100 => 100,
        101...119 => 120,
        120...149 => 160,
        150...200 => 200,
        201...240 => 240,
        241...299 => 300,
        300...400 => 400,
        401...449 => 500,
        450...599 => 600,
        600...749 => 800,
        750...999 => 1000,
        _ => unreachable!(),
    };

    let v3 = v2 as f64 / step;
    if v3 > 1.0 {
        v3.round()
    } else {
        v3
    }
}

#[cfg(test)]
mod tests {
    use super::calc_max_value;

    macro_rules! test {
        ($name:ident, $value:expr, $result:expr) => (
            #[test]
            fn $name() {
                assert_eq!(calc_max_value($value), $result);
            }
        )
    }

    test!(max_v_02, 0.2, 0.2);
    test!(max_v_1, 1.0, 1.0);
    test!(max_v_2, 2.0, 2.0);
    test!(max_v_3, 3.0, 4.0);
    test!(max_v_4, 4.0, 4.0);
    test!(max_v_5, 5.0, 6.0);
    test!(max_v_6, 6.0, 8.0);
    test!(max_v_7, 7.0, 8.0);
    test!(max_v_8, 8.0, 10.0);
    test!(max_v_9, 9.0, 10.0);
    test!(max_v_10, 10.0, 10.0);
    test!(max_v_11, 11.0, 12.0);
    test!(max_v_12, 12.0, 16.0);
    test!(max_v_14, 14.0, 16.0);
    test!(max_v_15, 15.0, 20.0);
    test!(max_v_19, 19.0, 20.0);
    test!(max_v_20, 20.0, 20.0);
    test!(max_v_21, 21.0, 24.0);
    test!(max_v_22, 22.0, 24.0);
    test!(max_v_25, 25.0, 30.0);
    test!(max_v_29, 29.0, 30.0);
    test!(max_v_30, 30.0, 40.0);
    test!(max_v_35, 35.0, 40.0);
    test!(max_v_40, 40.0, 40.0);
    test!(max_v_42, 42.0, 50.0);
    test!(max_v_50, 50.0, 60.0);
    test!(max_v_57, 57.0, 60.0);
    test!(max_v_60, 60.0, 80.0);
    test!(max_v_70, 70.0, 80.0);
    test!(max_v_74, 74.0, 80.0);
    test!(max_v_75, 75.0, 100.0);
    test!(max_v_95, 95.0, 100.0);
    test!(max_v_100, 100.0, 100.0);
    test!(max_v_101, 101.0, 120.0);
    test!(max_v_129, 129.0, 160.0);
    test!(max_v_1120, 1120.0, 1200.0);
    test!(max_v_19684, 19684.0, 20000.0);

    #[test]
    fn test_max_num_match() {
        // must not fail
        for i in 10..1000 {
            calc_max_value(i as f64 / 10.0);
        }
    }
}
