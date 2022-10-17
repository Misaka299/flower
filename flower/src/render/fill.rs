use std::ops::{Deref, DerefMut};

use image::DynamicImage;

use crate::render::color::Color;

// pub struct Fill(dyn TFill);

pub enum Fill {
    Image(Image),
    Color(Color),
}

pub enum Align {
    Center,
    Left,
    Top,
    Right,
    Bottom,
    LeftTop,
    RightTop,
    LeftBottom,
    RightBottom,
}

pub enum ZoomType {
    // 平铺
    // 计算像素坐标，转换纹理坐标
    /// image (width, height)
    Tile(f32, f32),
    // 缩放
    // 写死最大
    Zoom,
    // 重复
    // 计算像素坐标，转换纹理坐标
    /// image (width, height)
    Repeat(f32, f32),
    // 多余的地方用边框填充
    // 计算像素坐标，转换纹理坐标
    /// image (width, height, _)
    /// color (_, _, color)
    ClampToBorder(f32, f32, Color),
}

pub struct Image {
    pub src: DynamicImage,
    pub filter_type: u32,
    pub align: Align,
    pub zoom_type: ZoomType,
}

impl Image {
    ///
    /// filter_type
    ///     glow::GL_NEAREST
    ///     glow::LINEAR
    ///
    pub fn new(src: DynamicImage, align: Align, zoom_type: ZoomType, filter_type: u32) -> Self {
        Self {
            src,
            filter_type,
            align,
            zoom_type,
        }
    }

    pub fn def_from_src(src: DynamicImage) -> Self {
        Self {
            src,
            filter_type: glow::NEAREST,
            align: Align::Center,
            zoom_type: ZoomType::Zoom,
        }
    }
}

impl Image {
    pub fn texture_coord(&self) -> Vec<f32> {
        match self.zoom_type {
            ZoomType::Tile(width, height) => {}
            ZoomType::Zoom => {}
            ZoomType::Repeat(width, height) => {}
            ZoomType::ClampToBorder(width, height, color) => {}
        }
        return Vec::<f32>::new();
    }
}

impl Deref for Image {
    type Target = DynamicImage;

    fn deref(&self) -> &Self::Target {
        &self.src
    }
}

impl DerefMut for Image {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.src
    }
}


// pub struct Gradient {
//     pub i: i32,
// }
//
// impl Fill for Gradient{
//
// }