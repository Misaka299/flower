use std::ops::{Deref, DerefMut};

use glow::{Context, HasContext, INT, LINE_LOOP, QUADS};
use log::debug;
use nalgebra_glm::cos;

use crate::Px;
use crate::rect::Rect;
use crate::render::border::Border;
use crate::render::color::Color;
use crate::render::draw::Draw;

pub trait Render {}

pub struct PaintStyle {
    radiu: Px,
    border: Border,
}

pub struct Renderer {
    // gl: Context,
    gl: Draw,
}

impl Renderer {
    pub(crate) fn set_window_height(&mut self, window_height: Px) {
        self.gl.window_height = window_height;
    }
}

impl Render for Renderer {}

impl Renderer {
    pub fn new(gl: Context, height: Px) -> Self {
        Self {
            gl: Draw::new(gl, height)
        }
    }
}

impl Renderer {
    ///
    /// 绘制矩形
    ///
    /// 支持如下属性
    /// radiu
    /// border
    ///
    pub fn rect(&mut self, rect: &Rect, border: Option<Border>) {
        let gl = &self.gl;
        unsafe {
            gl.use_def_program();
            let loc = gl.get_uniform_location(gl.shader, "viewport");
            gl.uniform_4_i32(
                loc.as_ref(),
                rect.left as i32,
                rect.top as i32,
                rect.width as i32,
                rect.height as i32,
            );

            let loc = gl.get_uniform_location(gl.shader, "radius");
            gl.uniform_1_f32(
                loc.as_ref(),
                1 as f32,
            );

            let loc = gl.get_uniform_location(gl.shader, "alpha");
            gl.uniform_1_f32(
                loc.as_ref(),
                22 as f32,
            );

            // let buffer = gl.create_buffer().unwrap();
            // let len = 2;
            // gl.bind_buffer(glow::ARRAY_BUFFER, Some(buffer));
            // gl.vertex_attrib_pointer_i32(len, 0 as i32, INT, 0, 0);
            // gl.enable_vertex_attrib_array(len);
            //
            // let buffer = gl.create_buffer().unwrap();
            // let len = 2;
            // gl.bind_buffer(glow::ARRAY_BUFFER, Some(buffer));
            // gl.vertex_attrib_pointer_i32(len, 0 as i32, INT, 0, 0);
            // gl.enable_vertex_attrib_array(len);
            //
            // let buffer = gl.create_buffer().unwrap();
            // let len = 3;
            // gl.bind_buffer(glow::ARRAY_BUFFER, Some(buffer));
            // gl.vertex_attrib_pointer_i32(len, 0 as i32, INT, 0, 0);
            // gl.enable_vertex_attrib_array(len);
            // let data: [u8; 8] = [110, 10, 240, 10, 110, 140, 240, 140];
            // let buffer = gl.create_buffer().unwrap();
            // gl.bind_buffer(glow::ARRAY_BUFFER, Some(buffer));
            // gl.buffer_data_u8_slice(
            //     glow::ARRAY_BUFFER,
            //     data.as_slice(),
            //     STATIC_DRAW,
            // );
            // gl.bind_buffer(glow::ARRAY_BUFFER, None);

            debug!("{}",Color::from_hex_str("00CCFF").unwrap().rgba_gl_vec4());
            gl.draw_arrays(LINE_LOOP,0, 4);


            debug!("ss -> {:?}",gl.get_debug_message_log(gl.get_error()));
        }
    }

    ///
    /// 填充矩形
    ///
    /// 支持如下属性
    /// radiu
    /// border
    ///
    pub fn fill(&mut self, rect: &Rect, border: Option<Border>) {}

    /// 绘制文字
    pub fn text(&mut self, text: String, setting: PaintStyle) {}

    /// 绘制线
    pub fn line(&mut self, line: (Px, Px)) {}

    // 贝塞尔曲线
    pub fn bezier(&mut self, line_start: (Px, Px), line_end: (Px, Px)) {}

    /// todo 参数未定义
    pub fn circle(&mut self) {}

    /// 缺失参数,绘制方式
    pub fn image(&mut self, image: Vec<u8>) {}
}

impl Deref for Renderer {
    type Target = Draw;

    fn deref(&self) -> &Self::Target {
        &self.gl
    }
}

impl DerefMut for Renderer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.gl
    }
}