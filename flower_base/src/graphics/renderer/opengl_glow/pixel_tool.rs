#[derive(Debug, Copy, Clone)]
pub struct PixelTool {
    pub(crate) screen_width: f32,

    pub(crate) screen_height: f32,
}

impl PixelTool {
    pub fn create(width: u32, height: u32) -> Self {
        Self {
            screen_width: width as f32,
            screen_height: height as f32,
        }
    }

    #[inline]
    pub fn update(&mut self, width: u32, height: u32) {
        self.screen_width = width as f32;
        self.screen_height = height as f32;
    }
}

impl PixelTool {
    #[inline]
    pub fn to_gl_x(&self, x: f32) -> f32 {
        2. * (x / self.screen_width) - 1.
    }

    #[inline]
    pub fn to_gl_y(&self, y: f32) -> f32 {
        -2. * (y / self.screen_height) + 1.
    }
}