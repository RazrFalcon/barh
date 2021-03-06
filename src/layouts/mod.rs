pub use self::layout::{CalcLayout, DrawLayout};
pub use self::main_layout::MainLayout;

mod bars_layout;
mod main_layout;
mod title_layout;
mod haxis_layout;
mod vaxis_layout;
mod layout;
mod adaptors;

#[derive(Debug)]
pub struct Size {
    pub w: u32,
    pub h: u32,
}

impl Default for Size {
    fn default() -> Size {
        Size {
            w: 0,
            h: 0,
        }
    }
}

impl Size {
    fn into_rect(&self, x: i32, y: i32) -> Rect {
        Rect {
            x: x,
            y: y,
            w: self.w,
            h: self.h,
        }
    }
}

#[derive(Debug)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub w: u32,
    pub h: u32,
}

impl Default for Rect {
    fn default() -> Rect {
        Rect {
            x: 0,
            y: 0,
            w: 0,
            h: 0,
        }
    }
}

impl Rect {
    fn new(x: i32, y: i32, w: u32, h: u32) -> Rect {
        Rect {
            x: x,
            y: y,
            w: w,
            h: h,
        }
    }

    fn adjusted(&self, margins: &Margins) -> Rect {
        Rect {
            x: self.x + margins.left as i32,
            y: self.y + margins.top as i32,
            w: self.w - margins.left - margins.right,
            h: self.h - margins.top - margins.bottom,
        }
    }

    fn right(&self) -> u32 {
        self.x.abs() as u32 + self.w
    }
}

#[derive(Debug)]
pub struct Margins {
    pub left: u32,
    pub top: u32,
    pub right: u32,
    pub bottom: u32,
}

impl Default for Margins {
    fn default() -> Margins {
        Margins {
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
        }
    }
}
