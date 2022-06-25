use std::ops::{Deref, DerefMut};

#[derive(Debug, Clone)]
pub enum Shape {
    Sector { origin_x: f32, origin_y: f32, radiu_x: f32, radiu_y: f32, start_angle: f32, end_angle: f32 },
    Rect {
        left: f32,
        top: f32,
        width: f32,
        height: f32,
        radiu_left_top: f32,
        radiu_left_bottom: f32,
        radiu_right_top: f32,
        radiu_right_bottom: f32,
    },
    Line { start_x: f32, start_y: f32, end_x: f32, end_y: f32 },
    Other { vertex: Vec<f32> },
}

impl Shape {
    ///
    /// create a custom shape.
    ///
    pub fn new_custom(vertex: Vec<f32>) -> Self {
        Self::Other {
            vertex,
        }
    }

    ///
    /// create a line.
    ///
    pub fn line(start_x: f32, start_y: f32, end_x: f32, end_y: f32) -> Self {
        Self::Line {
            start_x,
            start_y,
            end_x,
            end_y,
        }
    }


    ///
    /// create a rect shape.
    ///
    pub fn rect(left: f32, top: f32, width: f32, height: f32) -> Self {
        Self::Rect {
            left,
            top,
            width,
            height,
            radiu_left_top: 0.0,
            radiu_left_bottom: 0.0,
            radiu_right_top: 0.0,
            radiu_right_bottom: 0.0,
        }
    }

    ///
    /// create a rounded rectangle shape.
    ///
    pub fn rect_radiu(left: f32, top: f32, width: f32, height: f32, radiu: f32) -> Self {
        Self::Rect {
            left,
            top,
            width,
            height,
            radiu_left_top: radiu,
            radiu_left_bottom: radiu,
            radiu_right_top: radiu,
            radiu_right_bottom: radiu,
        }
    }

    ///
    /// create a circle.
    ///
    pub fn circle(origin_x: f32, origin_y: f32, radiu: f32) -> Self {
        Self::sector(origin_x, origin_y, radiu, 0., 360.)
    }

    ///
    /// create a circle use rectangle format param.
    /// If you pass a parameter of rectangular data, it will become an ellipse.
    ///
    pub fn circle_from_rect(left: f32, top: f32, width: f32, height: f32) -> Self {
        Self::sector_from_rect(left, top, width, height, 0., 360.)
    }

    ///
    /// create a sector.
    ///
    pub fn sector(origin_x: f32, origin_y: f32, radiu: f32, start_angle: f32, end_angle: f32) -> Self {
        Self::Sector {
            origin_x,
            origin_y,
            radiu_x: radiu,
            radiu_y: radiu,
            start_angle,
            end_angle,
        }
    }

    ///
    /// create a sector use rectangle format param.
    /// if you pass a parameter of rectangular data, it will become an ellipse.
    ///
    pub fn sector_from_rect(left: f32, top: f32, width: f32, height: f32, start_angle: f32, end_angle: f32) -> Self {
        let radiu_x = width / 2.;
        let radiu_y = height / 2.;
        Self::Sector {
            origin_x: left + radiu_x,
            origin_y: top + radiu_y,
            radiu_x,
            radiu_y,
            start_angle,
            end_angle,
        }
    }
}