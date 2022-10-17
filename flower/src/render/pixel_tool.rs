#[derive(Debug, Copy, Clone)]
pub struct PixelTool {
    pub(crate) screen_width: f32,

    pub(crate) screen_height: f32,

    screen_width_half: f32,

    screen_height_half: f32,

    gl_pixel_width: f32,

    gl_pixel_height: f32,
}

impl PixelTool {
    pub fn create(width: f32, height: f32) -> Self {
        let screen_width_half = width / 2f32;
        let screen_height_half = height / 2f32;
        Self {
            screen_width: width,
            screen_height: height,
            screen_width_half,
            screen_height_half,
            gl_pixel_width: 1f32 / screen_width_half,
            gl_pixel_height: 1f32 / screen_height_half,
        }
    }

    #[inline]
    pub fn update(&mut self, width: f32, height: f32) {
        self.screen_width = width;
        self.screen_height = height;
        self.screen_width_half = width / 2f32;
        self.screen_height_half = height / 2f32;
        self.gl_pixel_width = 1f32 / self.screen_width_half;
        self.gl_pixel_height = 1f32 / self.screen_height_half;
        println!("pixel -> {:?}", self);
    }
}

impl PixelTool {
    #[inline]
    pub fn to_gl_x(&self, x: f32) -> f32 {
        // if x in left return negative number
        if self.screen_width_half > x {
            return 0f32 - (1f32 - x * self.gl_pixel_width);
        }
        // x in right, need to subtract half the pixel length,then calc gl position
        return (x - self.screen_width_half) * self.gl_pixel_width;
    }

    #[inline]
    pub fn to_gl_y(&self, y: f32) -> f32 {
        // if y in bottom return negative number
        if self.screen_height_half > y {
            return 0f32 - (y - self.screen_height_half) * self.gl_pixel_height;
        }
        // y in top, need to subtract half the pixel length,then calc gl position
        return 1f32 - y * self.gl_pixel_height;
    }

    #[inline]
    pub fn get_gl_pixel_width(&self) -> f32 {
        return self.gl_pixel_width;
    }

    #[inline]
    pub fn get_gl_pixel_height(&self) -> f32 {
        return self.gl_pixel_height;
    }
}