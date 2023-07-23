use std::fs::File;
use std::ops::Add;
use std::path::PathBuf;

use glutin::{ContextWrapper, PossiblyCurrent};
use glutin::window::Window;
use image::DynamicImage;

use crate::graphics::brush::Brush;
use crate::graphics::color::Color;
use crate::graphics::font::Font;
use crate::graphics::pen::Pen;
use crate::graphics::rect::Rect;

pub mod renderer;
pub mod rect;
pub mod brush;
pub mod font;
pub mod pen;
pub mod color;

///
/// 画笔画刷 对象缓冲池，。end销毁。
///
///
///
///
pub trait Render {
    fn create() -> Self;

    /// 开始绘制
    fn begin_paint(&mut self, window_context: &ContextWrapper<PossiblyCurrent, Window>);

    /// 结束绘制
    fn end_paint(&mut self, window_context: &ContextWrapper<PossiblyCurrent, Window>);

    fn new_buffer_canvas(&mut self, width: i32, height: i32, id: i32);

    fn refresh_canvas_to_window(&mut self, id: Option<i32>, x: i32, y: i32);

    fn store(&mut self, rect: &Rect, pen: &Pen);

    //fn fill(&self, rect: &Rect, brush: &Brush);

    fn draw_line(&self, pt1: Point, pt2: Point, color: &Color, stroke_width: f32, stroke_style: DashStyle);

    fn draw_image(&mut self, image: Vec<u8>, rect_location: Rect);

    fn measure_text(&self, font: &Font, str: &impl AsRef<str>) -> Rect;

    fn draw_text_rect(&self, rect: &Rect, font: &Font, color: &Color, str: &impl AsRef<str>);

    fn update_window_size();
}

/// tip: gdi plus does not support floating-point parameters, and the framework will truncate the decimal point and convert it to an integer.
pub struct Point {
    pub x: f32,
    pub y: f32,
}


pub enum DashStyle {
    /// 实线
    Solid,
    /// 虚线
    Dash,
    /// 点线
    Dot,
    /// 虚线和点线交替
    DashDot,
    /// 虚线和两个点线交替
    DashDotDot,
}