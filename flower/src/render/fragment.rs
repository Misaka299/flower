use std::ops::{Deref, DerefMut};

use image::DynamicImage;

use crate::render::color::Color;
use crate::render::image::Image;

pub enum FragData {
    Image(Image),
    Color(Color),
}

fn test() {
    // Fragment::new().frag(FragType::Image());
    // Fragment {
    //     frag: vec![FragType::Image(image)]
    // }.frag;
}

