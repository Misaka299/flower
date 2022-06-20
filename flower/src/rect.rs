#[derive(Debug, Copy, Clone)]
pub struct Rect {
    pub left: f32,
    pub top: f32,
    pub width: f32,
    pub height: f32,
}

impl Rect {
    pub fn new(left: f32, top: f32, width: f32, height: f32) -> Rect {
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
    pub x: f32,
    pub y: f32,
}

impl Point {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
        }
    }
}