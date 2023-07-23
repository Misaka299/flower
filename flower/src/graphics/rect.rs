use gdiplus_sys2::RectF;

#[derive(Debug, Copy, Clone)]
pub struct Rect {
    pub left: f32,
    pub top: f32,
    pub width: f32,
    pub height: f32,
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