use std::ops::{Deref, DerefMut};

#[derive(Debug, Copy,Clone)]
pub enum ShapeType {
    Sector,
    Other,
}

///
/// Shape.it record is shapes vertex.
///
#[derive(Debug, Clone)]
pub struct Shape {
    pub vertex: Vec<f32>,
    pub shape_type: ShapeType,
}

impl Shape {
    ///
    /// create a empty shape,then you can diy setting it.
    ///
    pub fn new() -> Self {
        Self {
            vertex: vec![],
            shape_type: ShapeType::Other,
        }
    }

    ///
    /// create a rect shape.
    ///
    pub fn rect(left: f32, top: f32, width: f32, height: f32) -> Self {
        let mut vec = Vec::new();
        // left-top
        vec.push(left);
        vec.push(top);

        // right-top
        vec.push(left + width);
        vec.push(top);

        // right-bottom
        vec.push(left + width);
        vec.push(top + height);

        // left-bottom
        vec.push(left);
        vec.push(top + height);
        Self {
            vertex: vec,
            shape_type: ShapeType::Other,
        }
    }

    ///
    /// create a rounded rectangle shape.
    ///
    pub fn rect_radiu(left: f32, top: f32, width: f32, height: f32,radiu:f32) -> Self {
        let mut vec = Vec::new();
        // left-top
        vec.push(left);
        vec.push(top);

        // right-top
        vec.push(left + width);
        vec.push(top);

        // right-bottom
        vec.push(left + width);
        vec.push(top + height);

        // left-bottom
        vec.push(left);
        vec.push(top + height);
        Self {
            vertex: vec,
            shape_type: ShapeType::Other,
        }
    }

    ///
    /// create a circle.
    ///
    pub fn circle(origin_x: f32, origin_y: f32, radiu: f32) -> Self {
        Self::sector(origin_y, origin_y, radiu, 0., 360.)
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
        Self { vertex: vec![origin_x, origin_y, radiu, radiu, start_angle, end_angle], shape_type: ShapeType::Sector }
    }

    ///
    /// create a sector use rectangle format param.
    /// if you pass a parameter of rectangular data, it will become an ellipse.
    ///
    pub fn sector_from_rect(left: f32, top: f32, width: f32, height: f32, start_angle: f32, end_angle: f32) -> Self {
        let radiu_x = width / 2.;
        let radiu_y = height / 2.;
        Self { vertex: vec![left + radiu_x, top + radiu_y, radiu_x, radiu_y, start_angle, end_angle], shape_type: ShapeType::Sector }
    }

    ///
    /// create a line.
    ///
    pub fn line(start_x: f32, start_y: f32, end_x: f32, end_y: f32) -> Shape {
        Self { vertex: vec![start_x, start_y, end_x, end_y], shape_type: ShapeType::Other }
    }
}

impl Shape {}


impl Deref for Shape {
    type Target = Vec<f32>;

    fn deref(&self) -> &Self::Target {
        &self.vertex
    }
}

impl DerefMut for Shape {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.vertex
    }
}