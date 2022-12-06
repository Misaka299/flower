use std::ops::{Deref, DerefMut};

use image::DynamicImage;

use crate::rect;
use crate::render::color::Color;
use crate::render::pixel_tool::PixelTool;
use crate::render::shape;
use crate::render::shape::Shape;

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

    pub fn from_src(src: DynamicImage) -> Self {
        Self {
            src,
            filter_type: glow::NEAREST,
            align: Align::Center,
            zoom_type: ZoomType::Zoom,
        }
    }
}

impl Image {
    pub fn texture_coord(&self, pixel: &PixelTool, vertex: &Vec<f32>) -> Vec<f32> {
        let mut coord = Vec::<f32>::new();
        match self.zoom_type {
            ZoomType::Tile(width, height) => {
                //画布像素大小  图片像素大小
                //画布gl坐标   图片纹理坐标
                // 图片纹理坐标 在 gl 坐标
                // 图片纹理坐标 -> gl坐标长度
                // let shape = Shape::rect(50., 50., 50., 50.);
                // if let Some(rect::Rect{ left, top, width, height }) = shape {
                //     PixelTool::create(width, height);
                // }
                //
                //
                // // 图片像素宽度
                // pixel.to_gl_x(width);
                //
                //
                // self.height();
                // self.width();
            }
            ZoomType::Zoom => {
                //缩放可以这样
                //画布像素大小  图片像素大小
                //画布gl坐标   图片纹理坐标
                for x in vertex {
                    coord.push((1. - x) / 2.);
                }

                //[-1.0, -1.0, 1.0, -1.0, 1.0, 1.0, -1.0, 1.0]


                // coord.push(1.);// 1
                // coord.push(1.);//
                // coord.push(0.);//-1
                // coord.push(1.);//1
                //
                // coord.push(0.);// -1
                // coord.push(0.);// -1
                // coord.push(1.);//
                // coord.push(0.);// -1
            }
            ZoomType::Repeat(width, height) => {}
            ZoomType::ClampToBorder(width, height, color) => {}
        }
        return coord;
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