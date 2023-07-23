use gdiplus_sys2::ARGB;

#[derive(Debug, Copy, Clone)]
pub struct Color {
    pub a: u8,
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Into<ARGB> for Color {
    fn into(self) -> ARGB {
        let alpha = self.a as u32;
        let red = self.r as u32;
        let green = self.g as u32;
        let blue = self.b as u32;

        let color = (alpha << 24) | (red << 16) | (green << 8) | blue;
        color as ARGB
    }
}
