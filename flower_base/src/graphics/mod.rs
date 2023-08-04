use glutin::{ContextWrapper, PossiblyCurrent};
use glutin::window::Window;
use once_cell::sync::Lazy;
use rustc_hash::FxHashMap;

use crate::graphics::color::Color;
use crate::graphics::font::Font;
use crate::graphics::pen::Pen;
use crate::graphics::rect::Rect;
use crate::graphics::renderer::Renderer;

pub mod renderer;
pub mod rect;
pub mod brush;
pub mod font;
pub mod pen;
pub mod color;

pub static mut RENDERERS: Lazy<FxHashMap<i32, Renderer>> = Lazy::new(|| FxHashMap::default());

#[macro_export]
macro_rules! rdr {
    ($id:expr) => {
        unsafe { crate::RENDERERS.get_mut($id) }
    };
}

pub trait Render {
    fn create(window_id: i32) -> Self;

    fn get_window_id(&self) -> i32;
    /// 开始绘制
    fn init(&mut self, window_context: &ContextWrapper<PossiblyCurrent, Window>);

    fn new_canvas_buffer(&mut self, id: i32, width: i32, height: i32);

    fn refresh_to_buffer(&mut self, source_id: i32, target_id: i32, x: i32, y: i32);

    fn refresh_canvas_to_window(&mut self);

    fn delete_canvas_buffer(&mut self, id: i32);

    fn store(&mut self, rect: &Rect, pen: &Pen);

    //fn fill(&self, rect: &Rect, brush: &Brush);

    fn draw_line(&self, pt1: Point, pt2: Point, color: &Color, stroke_width: f32, stroke_style: DashStyle);

    fn draw_image(&mut self, image: Vec<u8>, rect_location: Rect);

    fn measure_text(&mut self, font: &Font, str: &impl AsRef<str>) -> Rect;

    fn draw_text_rect(&mut self, rect: &Rect, font: &Font, color: &Color, str: &impl AsRef<str>);

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