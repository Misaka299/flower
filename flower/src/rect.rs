#[derive(Debug, Copy, Clone)]
pub struct Rect {
    pub left: u32,
    pub top: u32,
    pub width: u32,
    pub height: u32,
}

impl Rect {
    pub fn new(left: u32, top: u32, width: u32, height: u32) -> Rect {
        Self {
            left,
            top,
            width,
            height,
        }
    }
}
#[derive(Copy, Clone)]
pub struct Point {
    pub x: u32,
    pub y: u32,
}

impl Point {
    pub fn new(x: u32, y: u32) -> Self {
        Self {
            x,
            y,
        }
    }
}