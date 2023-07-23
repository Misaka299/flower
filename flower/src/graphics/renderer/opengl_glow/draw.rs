// use glow::{Context, FRAGMENT_SHADER, HasContext, VERTEX_SHADER};
// use image::DynamicImage;
//
// use crate::color::Color;
// use crate::shape::Shape;
// use crate::text::Text;
//
// #[derive(Debug)]
// pub struct Radio {
//     left_top: i32,
//     right_top: i32,
//     left_bottom: i32,
//     right_bottom: i32,
// }
//
// impl Radio {
//     pub fn radio_all(r: i32) -> Self {
//         Self {
//             left_top: r,
//             right_top: r,
//             left_bottom: r,
//             right_bottom: r,
//         }
//     }
// }
//
// #[derive(Debug)]
// pub struct Border {
//     left: i32,
//     top: i32,
//     width: i32,
//     height: i32,
// }
//
// impl Border {
//     pub fn border_all(l: i32) -> Self {
//         Self {
//             left: l,
//             top: l,
//             width: l,
//             height: l,
//         }
//     }
// }
//
// #[derive(Debug)]
// pub struct Image {
//     src: DynamicImage,
//     alpha: Option<i32>,
// }
//
// #[derive(Debug)]
// pub struct BoxShadow {
//     h_shadow: i32,
//     v_shadow: i32,
//     blur: Option<u32>,
//     spread: Option<u32>,
//     color: Option<Color>,
//     inset: Option<u32>,
// }
//
// #[derive(Debug)]
// pub enum Fill {
//     /// 纯色
//     Color(Color)
//     // todo 渐变
// }
//
// #[derive(Debug)]
// pub struct Draw<'a> {
//     gl: &'a Context,
//     shape: &'a Shape,
//     radio: Option<&'a Radio>,
//     border: Option<&'a Border>,
//     image: Option<&'a Image>,
//     // todo 颜色或者渐变
//     fill: Option<&'a Fill>,
//     text: Option<&'a Text>,
//     box_shadow: Option<Vec<&'a BoxShadow>>,
//     // fixme This feature is currently not in the implementation plan
//     // transform: Transform,
// }
//
// impl<'a> Draw<'a> {
//     pub fn new(gl: &'a Context, shape: &'a Shape) -> Self {
//         unsafe {
//             let program = gl.create_program().expect("Cannot create program");
//
//             let shader_sources = [
//                 (VERTEX_SHADER, include_str!("shader/shader.vert")),
//                 (FRAGMENT_SHADER, include_str!("shader/shader.frag")),
//             ];
//
//             let mut shaders = Vec::with_capacity(shader_sources.len());
//
//             for (shader_type, shader_source) in shader_sources.iter() {
//                 let shader = gl
//                     .create_shader(*shader_type)
//                     .expect("Cannot create shader");
//                 gl.shader_source(shader, &format!("{}\n{}", "#version 400", shader_source));
//                 gl.compile_shader(shader);
//                 if !gl.get_shader_compile_status(shader) {
//                     panic!("<{}>", gl.get_shader_info_log(shader));
//                 }
//                 gl.attach_shader(program, shader);
//                 shaders.push(shader);
//             }
//
//             gl.link_program(program);
//             if !gl.get_program_link_status(program) {
//                 panic!("{}", gl.get_program_info_log(program));
//             }
//
//             for shader in shaders {
//                 gl.detach_shader(program, shader);
//                 gl.delete_shader(shader);
//             }
//         }
//         Self {
//             gl,
//             shape,
//             radio: None,
//             border: None,
//             image: None,
//             fill: None,
//             text: None,
//             box_shadow: None,
//         }
//     }
//
//     pub fn new_image(gl: &'a Context, shape: &'a Shape, image: &'a Image) -> Self {
//         Self {
//             gl,
//             shape,
//             radio: None,
//             border: None,
//             image: Some(image),
//             fill: None,
//             text: None,
//             box_shadow: None,
//         }
//     }
//     // pub fn new_color(gl: &Context,shape: Shape) -> Self {}
//     pub fn new_text(gl: &'a Context, shape: &'a Shape) -> Self {
//         Self {
//             gl,
//             shape,
//             radio: None,
//             border: None,
//             image: None,
//             fill: None,
//             text: None,
//             box_shadow: None,
//         }
//     }
//
//     pub fn radio(mut self, radio: &'a Radio) -> Self {
//         self.radio = Some(radio);
//         self
//     }
//     pub fn border(mut self, border: &'a Border) -> Self {
//         self.border = Some(border);
//         self
//     }
//     pub fn image(mut self, image: &'a Image) -> Self {
//         self.image = Some(image);
//         self
//     }
//     pub fn fill(mut self, fill: &'a Fill) -> Self {
//         self.fill = Some(fill);
//         self
//     }
//     pub fn text(mut self, text: &'a Text) -> Self {
//         self.text = Some(text);
//         self
//     }
//     pub fn box_shadow(mut self, box_shadow: Vec<&'a BoxShadow>) -> Self {
//         self.box_shadow = Some(box_shadow);
//         self
//     }
//     pub fn box_shadow_one(mut self, box_shadow: &'a BoxShadow) -> Self {
//         self.box_shadow = Some(vec![box_shadow]);
//         self
//     }
//
//     pub fn draw(&self) {
//         if let Some(text) = self.text {
//
//         }
//     }
// }