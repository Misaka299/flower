use gdiplus_sys2::RectF;

#[derive(Debug, Copy, Clone)]
pub struct Rect {
    pub left: f32,
    pub top: f32,
    pub width: f32,
    pub height: f32,
}

impl Rect {
    pub fn clone_wh(&self) -> Self {
        Self{
            left: 0.0,
            top: 0.0,
            width: self.width,
            height: self.height,
        }
    }
}

impl Rect {
    pub fn move_to_target_center(&mut self, rect: &Rect) -> &mut Rect {
        // self.left = rect.left + rect.width / 2. - self.width / 2.;
        // self.top = rect.top + rect.height / 2. - self.height / 2.;
        self.left = rect.width / 2. - self.width / 2.;
        self.top = rect.height / 2. - self.height / 2.;
        self
    }
}

impl Into<RectF> for Rect {
    fn into(self) -> RectF {
        RectF {
            X: self.left,
            Y: self.top,
            Width: self.width,
            Height: self.height,
        }
    }
}

impl From<RectF> for Rect {
    fn from(value: RectF) -> Self {
        Rect {
            left: value.X,
            top: value.Y,
            width: value.Width,
            height: value.Height,
        }
    }
}

// impl<T> From<T> for Rect where T: ControlProperty {
//     fn from(value: T) -> Self {
//         Rect {
//             left: value.left(),
//             top: value.top(),
//             width: value.width(),
//             height: value.height(),
//         }
//     }
// }