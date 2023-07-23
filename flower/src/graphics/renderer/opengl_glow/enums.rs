// use image::DynamicImage;
// use crate::color::Color;
// use crate::enums::DrawOption::*;
// use crate::shape::Shape;
// use crate::text;
// use crate::draw::BoxShadow;
// #[test]
// fn test() {
//     draw(Shape::Rect { left: 0, top: 0, width: 0, height: 0 },
//          vec![
//              RadioAll(5),
//              Alpha(150),
//              BorderAll(3),
//          ]);
//     draw(Shape::Rect { left: 0, top: 0, width: 0, height: 0 },
//          vec![
//              Radio { left_top: 0, right_top: 0, left_bottom: 0, right_bottom: 0 },
//              Alpha(150),
//              BorderAll(3),
//              Border { left: 0, top: 0, width: 0, height: 0 },
//          ]);
// }
//
// #[derive(Debug)]
// pub enum DrawOption {
//     RadioAll(i32),
//     Radio { left_top: i32, right_top: i32, left_bottom: i32, right_bottom: i32 },
//     Alpha(i32),
//     BorderAll(i32),
//     Border { left: i32, top: i32, width: i32, height: i32 },
//     Image(DynamicImage),
//     Color(Color),
//     Text(text::Text),
//     BoxShadow(Vec<BoxShadow>),
//     BoxShadowOne(BoxShadow),
//     // fixme This feature is currently not in the implementation plan
//     Transform,
// }
//
//
// fn draw(shape: Shape, draw_option: Vec<DrawOption>) {}
//
// // todo 这个不是渲染层，而是组件层的
// enum TextOption {
//     /// 占位符
//     Placeholder(char),
//     /// 遮掩符
//     Mask(char),
// }


use glow::{LINE_LOOP, TRIANGLE_FAN};

#[derive(Debug, Clone)]
pub enum DrawMode {
    Fill,
    // color and line_width
    Line,
}

impl DrawMode {
    pub fn get_gl_mode(&self) -> u32 {
        match self {
            DrawMode::Fill { .. } => { TRIANGLE_FAN }
            DrawMode::Line { .. } => { LINE_LOOP }
        }
    }
}