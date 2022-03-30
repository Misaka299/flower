pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl Color {
    pub fn rgba(r: u8, g: u8, b: u8, a: u8) -> Color {
        Self {
            r,
            g,
            b,
            a,
        }
    }
    pub fn rgb(r: u8, g: u8, b: u8) -> Color {
        Self {
            r,
            g,
            b,
            a: 1,
        }
    }

    pub fn rgba_gl_vec4(&self) -> String {
        format!("vec4({},{},{},{});", self.r as f32 / 255., self.g as f32 / 255., self.b as f32 / 255., self.a)
    }
}