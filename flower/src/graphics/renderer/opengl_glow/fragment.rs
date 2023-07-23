use glow::Texture;
use image::DynamicImage;

use crate::drawing::graphics::color::Color;
// use crate::render::color::Color;

pub enum FragData<'a> {
    Image(&'a DynamicImage),
    Texture(Texture),
    Color(Color),
    // Text(Canvas),
}

fn test() {
    // Fragment::new().frag(FragType::Image());
    // Fragment {
    //     frag: vec![FragType::Image(image)]
    // }.frag;
}

