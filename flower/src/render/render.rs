extern crate nalgebra_glm as glm;

use std::collections::HashMap;
use std::ops::{Add, Deref, DerefMut};

use bytemuck::{bytes_of, cast_slice};
use glm::{acos, acosh, cos, sin};
use glow::{ARRAY_BUFFER, Context, DOUBLE, FALSE, FLOAT, HasContext, HIGH_FLOAT, INT, LINE_LOOP, LINE_STRIP, NativeBuffer, QUADS, STATIC_DRAW, UNSIGNED_INT};
use log::debug;

use crate::AGLTool::AGLTool;
use crate::Px;
use crate::rect::{Point, Rect};
use crate::render::border::Border;
use crate::render::color::Color;
use crate::render::draw::Draw;
use crate::render::shape::ShapeCoord;

pub struct PaintStyle {
    radiu: Px,
    border: Border,
}

pub struct Renderer {
    // gl: Context,
    gl: Draw,
    agl: AGLTool,
    scene_size: Point,
}

impl Renderer {
    pub fn new(gl: Context, scene_size: Point) -> Self {
        Self {
            gl: Draw::new(gl),
            agl: AGLTool::create(scene_size.x as f32, scene_size.y as f32),
            scene_size,
        }
    }
}

impl Renderer {
    pub fn set_canvas_size(&mut self, size: Point) {
        self.scene_size = size;
        self.agl.update(size.x as f32, size.y as f32);
    }

    /// 使用
    ///
    pub fn create_canvas(&self, rect: &Rect) {
        println!("view -> {:?}", rect);
        unsafe {
            debug!("create_canvas -> {} , {} , {} , {}",rect.left as i32, self.scene_size.x as i32 - rect.top as i32 - rect.height as i32, rect.width as i32, rect.height as i32);
            // self.viewport(rect.left as i32, self.size.0 as i32 - rect.top as i32 - rect.height as i32, rect.width as i32, rect.height as i32);
        }
    }

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
            let buffer = gl.create_buffer().unwrap();
            // let loc = gl.get_uniform_location(gl.shader, "viewport");
            // gl.uniform_4_i32(
            //     loc.as_ref(),
            //     rect.left as i32,
            //     rect.top as i32,
            //     rect.width as i32,
            //     rect.height as i32,
            // );
            //
            // let loc = gl.get_uniform_location(gl.shader, "radius");
            // gl.uniform_1_f32(
            //     loc.as_ref(),
            //     1 as f32,
            // );
            //
            // let loc = gl.get_uniform_location(gl.shader, "alpha");
            // gl.uniform_1_f32(
            //     loc.as_ref(),
            //     22 as f32,
            // );

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
            gl.draw_arrays(LINE_LOOP, 0, 360);


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

    ///
    /// 绘制圆
    ///
    pub fn circle(&mut self, shape_coord: ShapeCoord) {
        let gl = &self.gl;
        unsafe {
            gl.use_def_program();


            let pi = f64::acos(-1.0);
            let radiu = 5;


            // let mut vec:Vec<u8> = Vec::new();
            let mut vec = Vec::new();
            // for i in 0..360 {
            //     let x = f64::cos(2. * pi / 360. * i as f64) * 0.5;
            //     let y = f64::sin(2. * pi / 360. * i as f64) * 0.5;
            //     // vec.extend_from_slice(bytes_of(&x));
            //     // vec.extend_from_slice(bytes_of(&y));
            //     vec.push(x);
            //     vec.push(y);
            // }


            vec.push(self.agl.to_glx(200.));
            vec.push(self.agl.to_gly(200.));

            vec.push(self.agl.to_glx(200.) + self.agl.to_glwidth(200.));
            vec.push(self.agl.to_gly(200.));


            vec.push(self.agl.to_glx(200.) + self.agl.to_glwidth(200.));
            vec.push(self.agl.to_gly(200.) + self.agl.to_glheight(200.));

            vec.push(self.agl.to_glx(200.));
            vec.push(self.agl.to_gly(200.) + self.agl.to_glheight(200.));


            println!("{:?}", vec);

            let vao = gl.create_vertex_array();
            let vbo = gl.create_buffer();

            // 绑定vao
            gl.bind_vertex_array(vao.ok());

            // 绑定vbo
            gl.bind_buffer(ARRAY_BUFFER, vbo.ok());
            gl.buffer_data_u8_slice(ARRAY_BUFFER, cast_slice(&vec), STATIC_DRAW);

            // 告诉vao如何解释vbo的数据
            gl.vertex_attrib_pointer_f32(0, 2, FLOAT, false, 0, 0);
            gl.enable_vertex_attrib_array(0);



            let x = 100.;
            let y = 100.;
            let mut model = glm::Mat4::identity();

            let mode_size = Point::new(100., 100.);

            //
            // model = glm::translate(&model, &glm::vec3(x, y, 0.0f32));
            // model = glm::translate(&model, &glm::vec3(0.5f32 * (mode_size.x as f32), 0.5f32 * (mode_size.y as f32), 0.0f32));
            // model = glm::rotate(&model, (0 as f32).to_radians(), &glm::vec3(0.0f32, 0.0f32, 1.0f32));
            // model = glm::translate(&model, &glm::vec3(-0.5f32 * (mode_size.x as f32), -0.5f32 * (mode_size.y as f32), 0.0f32));
            // model = glm::scale(&model, &glm::vec3(mode_size.x as f32, mode_size.y as f32, 1.0f32));
            // gl.uniform_matrix_4_f32_slice(gl.get_uniform_location(gl.shader, &"model").as_ref(), false, model.as_slice());

            // let projection = glm::ortho(
            //     0.0,
            //     self.scene_size.x as f32,
            //     self.scene_size.y as f32,
            //     0.0,
            //     -1.0,
            //     1.0);
            //
            // gl.uniform_matrix_4_f32_slice(gl.get_uniform_location(gl.shader, &"projection").as_ref(), false, projection.as_slice());

            // gl.draw_elements(LINE_LOOP, 360, DOUBLE, 0);
            gl.draw_arrays(QUADS, 0, 4);
            debug!("ss -> {:?}",gl.get_debug_message_log(gl.get_error()));
            // let buffer = gl.create_buffer();
            // gl.bind_buffer(ARRAY_BUFFER, buffer.ok());
            // gl.buffer_data_u8_slice(ARRAY_BUFFER, vec![].as_slice(), STATIC_DRAW);
            // gl.bind_buffer(ARRAY_BUFFER, None);
            // update
            //
            // bind buffer
            // buffer_data_u8_slice
            // bind_buffer None
            //
        }
    }

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