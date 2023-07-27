use crate::graphics::color::Color;

pub trait Brush {}

pub struct SolidBrush {
    color: Color,
}

pub struct ImageBrush {
    // image: Image,
}

pub struct DrawOptions{

}