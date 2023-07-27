use std::ops::{Deref, DerefMut};
use glow::{Context, FRAGMENT_SHADER, HasContext, VERTEX_SHADER};

use crate::drawing::graphics::pixel_tool::PixelTool;

#[repr(i32)]
pub enum FragCode {
    Color = 1,
    Texture = 2,
    Text = 3,
}

pub struct Renderer {
    gl: Context,
    pub(crate) pixel: PixelTool,
}

// create
impl Renderer {
    pub fn new(gl: Context, scene_size: (u32, u32)) -> Self {
        unsafe {
            let program = gl.create_program().expect("Cannot create program");

            let shader_sources = [
                (VERTEX_SHADER, include_str!("shader/shader.vert")),
                (FRAGMENT_SHADER, include_str!("shader/shader.frag")),
            ];

            let mut shaders = Vec::with_capacity(shader_sources.len());

            for (shader_type, shader_source) in shader_sources.iter() {
                let shader = gl
                    .create_shader(*shader_type)
                    .expect("Cannot create shader");
                gl.shader_source(shader, &format!("{}\n{}", "#version 450", shader_source));
                gl.compile_shader(shader);
                if !gl.get_shader_compile_status(shader) {
                    panic!("<{}>", gl.get_shader_info_log(shader));
                }
                gl.attach_shader(program, shader);
                shaders.push(shader);
            }

            gl.link_program(program);
            if !gl.get_program_link_status(program) {
                panic!("{}", gl.get_program_info_log(program));
            }

            for shader in shaders {
                gl.detach_shader(program, shader);
                gl.delete_shader(shader);
            }

            Self {
                gl,
                pixel: PixelTool::create(scene_size.0, scene_size.1),
                // shader: program,
                // is_canvas: false,
                // vbo_list: Vec::<NativeBuffer>::new(),
                // texture: None,
            }
        }
    }
}

impl Renderer {
    pub fn update_window_size(&mut self, size: (u32, u32)) {
        self.pixel.update(size.0, size.1);
    }
}

impl Deref for Renderer {
    type Target = Context;

    fn deref(&self) -> &Self::Target {
        &self.gl
    }
}

impl DerefMut for Renderer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.gl
    }
}




// use std::ops::{Deref, DerefMut};
// use std::vec;
//
// use bytemuck::cast_slice;
// use glow::{Buffer, FRAMEBUFFER, HasContext, Texture};
// use glow::{ARRAY_BUFFER, Context, FLOAT, FRAGMENT_SHADER, LINEAR, NativeBuffer, NO_ERROR, Program, RED, REPEAT, RGB, RGBA, STATIC_DRAW, TEXTURE_2D, TEXTURE_MAG_FILTER, TEXTURE_MIN_FILTER, TEXTURE_WRAP_S, TEXTURE_WRAP_T, UNPACK_ALIGNMENT, UNSIGNED_BYTE, VERTEX_SHADER};
// use glutin::Rect;
// use image::EncodableLayout;
// use log::{debug, error};
// use crate::drawing::graphics::enums::DrawMode;
// use crate::drawing::graphics::fragment::FragData;
// use crate::drawing::graphics::pixel_tool::PixelTool;
// use crate::drawing::graphics::shape::Shape;
//
//
// // pub struct PaintStyle {
// //     radiu: f32,
// //     border: Border,
// // }
//
// pub struct Renderer {
//     gl: Context,
//     pub(crate) pixel: PixelTool,
//     pub(crate) shader: Program,
//     is_canvas: bool,
//     vbo_list: Vec<Buffer>,
//     texture: Option<Texture>,
// }
//
// #[repr(i32)]
// pub enum FragCode {
//     Color = 1,
//     Texture = 2,
//     Text = 3,
// }
//
// // create
// impl Renderer {
//     pub fn new(gl: Context, scene_size: (u32,u32)) -> Self {
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
//                 gl.shader_source(shader, &format!("{}\n{}", "#version 200", shader_source));
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
//
//             Self {
//                 gl,
//                 pixel: PixelTool::create(scene_size.0, scene_size.1),
//                 shader: program,
//                 is_canvas: false,
//                 vbo_list: Vec::<NativeBuffer>::new(),
//                 texture: None,
//             }
//         }
//     }
// }
//
//
// // interface
// impl Renderer {
//     pub fn update_window_size(&mut self, size: (u32,u32)) {
//         self.pixel.update(size.0, size.1);
//     }
//
//     // pub fn create_canvas(&mut self, width: i32, height: i32) -> Texture {
//     //     unsafe {
//     //         self.is_canvas = true;
//     //
//     //         // 创建纹理
//     //         let texture = self.gl.create_texture().unwrap();
//     //         self.gl.bind_texture(TEXTURE_2D, Some(texture));
//     //
//     //         self.gl.tex_image_2d(TEXTURE_2D, 0, RGBA as i32, width, height, 0, RGBA, UNSIGNED_BYTE, None);
//     //         self.gl.tex_parameter_i32(TEXTURE_2D, TEXTURE_MIN_FILTER, LINEAR as i32);
//     //         self.gl.tex_parameter_i32(TEXTURE_2D, TEXTURE_MAG_FILTER, LINEAR as i32);
//     //
//     //         // 创建帧缓冲
//     //         let framebuffer = self.gl.create_framebuffer().unwrap();
//     //         self.gl.bind_framebuffer(FRAMEBUFFER, Some(framebuffer));
//     //         self.gl.framebuffer_texture_2d(FRAMEBUFFER, glow::COLOR_ATTACHMENT0, TEXTURE_2D, Some(texture), 0);
//     //
//     //         if self.gl.check_framebuffer_status(FRAMEBUFFER) != glow::FRAMEBUFFER_COMPLETE {
//     //             error!("frame buffer 错误")
//     //         }
//     //
//     //         self.gl.bind_framebuffer(FRAMEBUFFER, Some(framebuffer));
//     //         texture
//     //     }
//     // }
//
//     /// 绘制文字
//     // pub fn text(&mut self, text: String, setting: PaintStyle) {}
//
//     /// 缺失参数,绘制方式
//     pub fn image(&mut self, image: Vec<u8>) {}
//
//     pub fn use_def_program(&self) {
//         unsafe {
//             self.use_program(Some(self.shader));
//         }
//     }
//
//     pub fn draw_shape(&mut self, shape: &Shape, draw_mode: DrawMode, frag_data_list: Vec<FragData>) {
//         unsafe {
//             let vao = self.gl.create_vertex_array().unwrap();
//             self.gl.bind_vertex_array(Some(vao));
//
//             let mut vertex = shape.to_vertex(&self.pixel);
//             self.send_vertex(&vertex);
//
//             let mut frag_code = 0;
//             for frag_data in frag_data_list {
//                 match frag_data {
//                     FragData::Image(img) => unsafe {
//                         // if self.is_canvas {
//                         //     let format = if img.color().has_alpha() { RGBA } else { RGB };
//                         //     if let Shape::Rect { left, top, width, height } = shape {
//                         //         let img = img.resize(*width as u32, *height as u32, FilterType::Nearest).fliph();
//                         //         let left = 1080 - width - left;
//                         //         self.gl.tex_sub_image_2d(TEXTURE_2D, 0, left as i32, *top as i32, *width as i32, *height as i32, format, UNSIGNED_BYTE, PixelUnpackData::Slice(img.as_bytes()));
//                         //     }
//                         // } else {
//                         frag_code = frag_code | FragCode::Texture as i32;
//                         let texture = self.gl.create_texture().unwrap();
//                         self.gl.bind_texture(TEXTURE_2D, Some(texture));
//                         self.texture = Some(texture);
//
//                         self.send_coord(&shape.to_texture_coord(None));
//
//                         self.gl.tex_parameter_i32(TEXTURE_2D, TEXTURE_WRAP_S, REPEAT as i32);
//                         self.gl.tex_parameter_i32(TEXTURE_2D, TEXTURE_WRAP_T, REPEAT as i32);
//                         self.gl.tex_parameter_i32(TEXTURE_2D, TEXTURE_MIN_FILTER, LINEAR as i32);
//                         self.gl.tex_parameter_i32(TEXTURE_2D, TEXTURE_MAG_FILTER, LINEAR as i32);
//
//                         let format = if img.color().has_alpha() { RGBA } else { RGB };
//                         self.gl.tex_image_2d(TEXTURE_2D, 0, format as i32, img.width() as i32, img.height() as i32, 0, format, UNSIGNED_BYTE, Some(img.as_bytes()));
//
//                         self.gl.generate_mipmap(TEXTURE_2D);
//                         // }
//                     }
//                     FragData::Texture(texture) => unsafe {
//                         frag_code = frag_code | FragCode::Texture as i32;
//                         self.gl.bind_texture(TEXTURE_2D, Some(texture));
//                         self.texture = Some(texture);
//                         self.send_coord(&shape.to_texture_coord(None));
//                     }
//                     FragData::Color(color) => unsafe {
//                         frag_code = frag_code | FragCode::Color as i32;
//                         let mut frag_coord: Vec<f32> = Vec::new();
//                         for i in 0..vertex.len() / 2 {
//                             frag_coord.append(&mut color.rgba_gl_vec());
//                         }
//                         let vbo = self.gl.create_buffer().unwrap();
//                         self.gl.bind_buffer(ARRAY_BUFFER, Some(vbo));
//                         self.gl.buffer_data_u8_slice(ARRAY_BUFFER, cast_slice(&frag_coord), STATIC_DRAW);
//                         // 告诉vao如何解释vbo的数据
//                         self.gl.vertex_attrib_pointer_f32(1, 4, FLOAT, false, 0, 0);
//                         self.gl.enable_vertex_attrib_array(1);
//
//                         self.gl.bind_buffer(ARRAY_BUFFER, None);
//                         self.vbo_list.push(vbo);
//                     }
//                     FragData::Text(canvas) => {
//                         frag_code = frag_code | FragCode::Text as i32;
//                         if self.is_canvas {
//                             self.gl.tex_image_2d(TEXTURE_2D, 0, RED as i32, canvas.size.x() as i32, canvas.size.y() as i32, 0, RED, UNSIGNED_BYTE, Some(&canvas.pixels));
//                         } else {
//                             let texture = self.gl.create_texture().unwrap();
//                             self.gl.bind_texture(TEXTURE_2D, Some(texture));
//                             self.texture = Some(texture);
//                             self.send_coord(&shape.to_texture_coord(None));
//
//                             self.pixel_store_i32(UNPACK_ALIGNMENT, 1);
//
//                             self.gl.tex_parameter_i32(TEXTURE_2D, TEXTURE_WRAP_S, REPEAT as i32);
//                             self.gl.tex_parameter_i32(TEXTURE_2D, TEXTURE_WRAP_T, REPEAT as i32);
//                             self.gl.tex_parameter_i32(TEXTURE_2D, TEXTURE_MIN_FILTER, LINEAR as i32);
//                             self.gl.tex_parameter_i32(TEXTURE_2D, TEXTURE_MAG_FILTER, LINEAR as i32);
//                             self.gl.tex_image_2d(TEXTURE_2D, 0, RED as i32, canvas.size.x() as i32, canvas.size.y() as i32, 0, RED, UNSIGNED_BYTE, Some(&canvas.pixels));
//
//                             self.gl.generate_mipmap(TEXTURE_2D);
//                         }
//                     }
//                 }
//             }
//             unsafe {
//                 debug!("type = {}", frag_code);
//                 self.gl.uniform_1_i32(self.gl.get_uniform_location(self.shader, "type").as_ref(), frag_code);
//             }
//
//             self.gl.draw_arrays(draw_mode.get_gl_mode(), 0, vertex.len() as i32 / 2);
//
//             if self.texture.is_some() {
//                 self.gl.delete_texture(self.texture.take().unwrap());
//             }
//             while !self.vbo_list.is_empty() {
//                 self.gl.delete_buffer(self.vbo_list.remove(0));
//             }
//
//             self.gl.delete_vertex_array(vao);
//
//             self.gl.line_width(1f32);
//
//             let error = self.gl.get_error();
//             if error != NO_ERROR {
//                 debug!("draw debug message -> {:?}",self.gl.get_debug_message_log(error));
//             }
//         }
//     }
//
//     pub fn send_vertex(&mut self, vertex: &Vec<f32>) {
//         unsafe {
//             let vbo = self.gl.create_buffer().unwrap();
//
//             // 绑定vbo
//             self.gl.bind_buffer(ARRAY_BUFFER, Some(vbo));
//             self.gl.buffer_data_u8_slice(ARRAY_BUFFER, cast_slice(&vertex), STATIC_DRAW);
//
//             // 告诉vao如何解释vbo的数据
//             self.gl.vertex_attrib_pointer_f32(0, 2, FLOAT, false, 0, 0);
//             self.gl.enable_vertex_attrib_array(0);
//             self.gl.bind_buffer(ARRAY_BUFFER, None);
//             self.vbo_list.push(vbo);
//         }
//     }
//
//     pub fn send_coord(&mut self, coord: &Vec<f32>) {
//         unsafe {
//             let vbo = self.gl.create_buffer().unwrap();
//             self.gl.bind_buffer(ARRAY_BUFFER, Some(vbo));
//             self.gl.buffer_data_u8_slice(ARRAY_BUFFER, cast_slice(&coord), STATIC_DRAW);
//
//             // 告诉vao如何解释vbo的数据
//             self.gl.vertex_attrib_pointer_f32(2, 2, FLOAT, false, 0, 0);
//             self.gl.enable_vertex_attrib_array(2);
//             self.vbo_list.push(vbo);
//         }
//     }
//
//     // pub fn draw_rect(&mut self, shape: &Shape, draw_mode: &DrawMode, frag_data_list: &Vec<FragData>) {
//     //     unsafe {
//     //         let vao = self.gl.create_vertex_array().unwrap();
//     //         self.gl.bind_vertex_array(Some(vao));
//     //         let vertex = shape.to_vertex(&self.pixel);
//     //         let count = (vertex.len() / 2) as i32;
//     //
//     //         self.send_vertex(&vertex);
//     //         self.send_frag_data(shape, count, frag_data_list);
//     //         self.draw(draw_mode.get_gl_mode(), count);
//     //         self.gl.delete_vertex_array(vao);
//     //     }
//     // }
//
//     // pub fn send_frag_data(&mut self, shape: &Shape, count: i32, frag_data_list: &Vec<FragData>) {
//     //     let mut frag_code = 0;
//     //     for frag_data in frag_data_list {
//     //         match frag_data {
//     //             FragData::Image(img) => unsafe {
//     //                 if self.is_canvas {
//     //                     let format = if img.color().has_alpha() { RGBA } else { RGB };
//     //                     self.gl.tex_sub_image_2d(TEXTURE_2D, 0, 0, 0, img.width() as i32, img.height() as i32, format, UNSIGNED_BYTE, PixelUnpackData::Slice(img.as_bytes()));
//     //                 } else {
//     //                     frag_code = frag_code | FragCode::Texture as i32;
//     //                     let texture = self.gl.create_texture().unwrap();
//     //                     self.gl.bind_texture(TEXTURE_2D, Some(texture));
//     //                     self.texture = Some(texture);
//     //
//     //                     self.send_coord(&shape.to_texture_coord(Some((img.width(), img.height()))));
//     //
//     //                     self.gl.tex_parameter_i32(TEXTURE_2D, TEXTURE_WRAP_S, REPEAT as i32);
//     //                     self.gl.tex_parameter_i32(TEXTURE_2D, TEXTURE_WRAP_T, REPEAT as i32);
//     //                     self.gl.tex_parameter_i32(TEXTURE_2D, TEXTURE_MIN_FILTER, LINEAR as i32);
//     //                     self.gl.tex_parameter_i32(TEXTURE_2D, TEXTURE_MAG_FILTER, LINEAR as i32);
//     //
//     //                     let format = if img.color().has_alpha() { RGBA } else { RGB };
//     //                     self.gl.tex_image_2d(TEXTURE_2D, 0, format as i32, img.width() as i32, img.height() as i32, 0, format, UNSIGNED_BYTE, Some(img.as_bytes()));
//     //
//     //                     self.gl.generate_mipmap(TEXTURE_2D);
//     //                 }
//     //             }
//     //             FragData::Texture(texture) => unsafe {
//     //                 if let rect = shape {}
//     //                 frag_code = frag_code | FragCode::Texture as i32;
//     //                 self.gl.bind_texture(TEXTURE_2D, Some(*texture));
//     //                 self.texture = Some(*texture);
//     //                 self.send_coord(&Shape::rect_framebuffer_texture_coord());
//     //             }
//     //             FragData::Color(color) => unsafe {
//     //                 frag_code = frag_code | FragCode::Color as i32;
//     //                 let mut frag_coord: Vec<f32> = Vec::new();
//     //                 for i in 0..count {
//     //                     frag_coord.append(&mut color.rgba_gl_vec());
//     //                 }
//     //                 let vbo = self.gl.create_buffer().unwrap();
//     //                 self.gl.bind_buffer(ARRAY_BUFFER, Some(vbo));
//     //                 self.gl.buffer_data_u8_slice(ARRAY_BUFFER, cast_slice(&frag_coord), STATIC_DRAW);
//     //                 // 告诉vao如何解释vbo的数据
//     //                 self.gl.vertex_attrib_pointer_f32(1, 4, FLOAT, false, 0, 0);
//     //                 self.gl.enable_vertex_attrib_array(1);
//     //
//     //                 self.gl.bind_buffer(ARRAY_BUFFER, None);
//     //                 self.vbo_list.push(vbo);
//     //             }
//     //             FragData::Text => {}
//     //         }
//     //     }
//     //     unsafe {
//     //         debug!("type = {}", frag_code);
//     //         self.gl.uniform_1_i32(self.gl.get_uniform_location(self.shader, "type").as_ref(), frag_code);
//     //     }
//     // }
//
//     // pub fn draw(&mut self, mode: u32, count: i32) {
//     //     unsafe {
//     //         self.gl.draw_arrays(mode, 0, count);
//     //         // self.gl.draw_buffer(COLOR_BUFFER_BIT);
//     //
//     //         if self.texture.is_some() {
//     //             self.gl.delete_texture(self.texture.take().unwrap());
//     //         }
//     //         while !self.vbo_list.is_empty() {
//     //             self.gl.delete_buffer(self.vbo_list.remove(0));
//     //         }
//     //
//     //         self.gl.line_width(1f32);
//     //
//     //         let error = self.gl.get_error();
//     //         if error != NO_ERROR {
//     //             debug!("draw debug message -> {:?}",self.gl.get_debug_message_log(error));
//     //         }
//     //     }
//     // }
//
//     // pub fn draw_text(&mut self, font: &mut FontLoader, size: f32, rect: Rect, text: &impl AsRef<str>) {
//         // let canvas = font.get_font_canvas_new(size, text.as_ref(), HintingOptions::None, RasterizationOptions::GrayscaleAa).unwrap();
//         // self.draw_shape(&Shape::rect(rect.left, rect.top, canvas.size.x(), canvas.size.y()), DrawMode::Fill, vec![
//         //     FragData::Text(canvas)
//         // ]);
//         // let text: String = text.as_ref().to_owned();
//         // let mut left = rect.left;
//         // let mut top = rect.top;
//         // let mut max = 0;
//         // 'end_char: for c in text.chars() {
//         //     println!("for - {}", c);
//         //     let canvas = font.get_font_canvas(c, size, HintingOptions::None, RasterizationOptions::Bilevel).unwrap();
//         //     let w = canvas.2.size.x();
//         //     let mut h = canvas.2.size.y();
//         //     if top + h > rect.height as i32 {
//         //         // 如果h小于等于0
//         //         'next_char: loop {
//         //             // 可以容纳的高度
//         //             h = rect.height - top;
//         //             if h > 0 {
//         //                 break 'next_char;
//         //             }
//         //             // 如果超出行宽度，那就这行字没有可以部分显示的，结束剩下的字的处理
//         //             if left + w > rect.width {
//         //                 break 'end_char;
//         //             } else {
//         //                 left = left + w+ canvas.0;
//         //             }
//         //         }
//         //     }
//         //     self.draw_shape(&Shape::rect(left + canvas.0, top + canvas.1, w, h), DrawMode::Fill, vec![
//         //         FragData::Text(canvas.2)
//         //     ]);
//         //     if w > max {
//         //         max = w + canvas.1;
//         //     }
//         //     // 如果超出行宽度，那就换行
//         //     if left + w > rect.width {
//         //         left = rect.left;
//         //         top = top + max;
//         //     } else {
//         //         left = left + w + canvas.0;
//         //     }
//         // }
//         // font.clear();
//     // }
//
//     pub fn end_canvas(&mut self) {
//         unsafe {
//             self.is_canvas = false;
//             self.gl.bind_framebuffer(FRAMEBUFFER, None);
//             self.gl.bind_texture(TEXTURE_2D, None);
//         }
//     }
//
//     pub fn delete_canvas(&mut self, mut canvas: Option<Texture>) {
//         unsafe {
//             if canvas.is_some() {
//                 self.gl.delete_texture(canvas.take().unwrap());
//             }
//         }
//     }
// }
//
