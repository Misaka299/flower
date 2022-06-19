pub struct AGLTool
{
    /**
     * Screen pixel width
     */
    screen_width: f32,

    /**
     * Screen pixel height
     */
    screen_height: f32,
}

impl AGLTool {
    pub fn create(width: f32, height: f32) -> Self {
        Self {
            screen_width: width,
            screen_height: height,
        }
    }

    pub fn update(&mut self, width: f32, height: f32) {
        println!("update -> {} {}", width, height);
        self.screen_width = width;
        self.screen_height = height;
    }
}

impl AGLTool {
    pub fn to_glx(&self) -> f32 {
        1f32
    }
}