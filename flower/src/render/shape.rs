use std::ops::{Deref, DerefMut};

///
/// Shape.it record is shapes vertex.
///
#[derive(Debug, Clone)]
pub struct Shape {
    vertex: Vec<f32>,
}

impl Shape {
    ///
    /// create a empty shape,then you can diy setting it.
    ///
    pub fn new() -> Self {
        Self {
            vertex: vec![]
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
        }
    }

    ///
    /// create a circle.
    ///
    pub fn circle(){}

    ///
    /// create a circle use rectangle format param.
    ///
    pub fn circle_from_rect(){}

    ///
    /// create a line.
    ///
    pub fn line(start_x: f32, start_y: f32, end_x: f32, end_y: f32) -> Shape {
        Self { vertex: vec![start_x, start_y, end_x, end_y] }
    }
}

impl Shape {
    /// whether vertex data is valid.
    pub fn is_valid(&self) -> bool {
        self.vertex.len() != 0 && self.vertex.len() % 2 == 0
    }
}


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