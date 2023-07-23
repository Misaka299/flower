use crate::graphics::color::Color;

#[derive(Debug, Copy, Clone)]
pub struct Pen {
    pub width: f32,
    pub color: Color,
}