extern crate nalgebra_glm as glm;

use std::ops::{Deref, DerefMut};

use bytemuck::cast_slice;
use glow::{Context, HasContext, Program};
use log::debug;

use crate::rect::Point;
use crate::render::border::Border;
use crate::render::color::Color;
use crate::render::pixel_tool::PixelTool;
use crate::render::shape::Shape;

pub struct PaintStyle {
    radiu: f32,
    border: Border,
}

pub struct Renderer {
    gl: Context,
    pixel: PixelTool,
    pub(crate) shader: Program,
}

// create
impl Renderer {
    pub fn new(gl: Context, scene_size: Point) -> Self {
        unsafe {
            let program = gl.create_program().expect("Cannot create program");

            let shader_sources = [
                (glow::VERTEX_SHADER, include_str!("shader/shader.vert")),
                (glow::FRAGMENT_SHADER, include_str!("shader/shader.frag")),
            ];

            let mut shaders = Vec::with_capacity(shader_sources.len());

            for (shader_type, shader_source) in shader_sources.iter() {
                let shader = gl
                    .create_shader(*shader_type)
                    .expect("Cannot create shader");
                gl.shader_source(shader, &format!("{}\n{}", "#version 460", shader_source));
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
                pixel: PixelTool::create(scene_size.x as f32, scene_size.y as f32),
                shader: program,
            }
        }
    }
}

// interface
impl Renderer {
    pub fn update_window_size(&mut self, size: Point) {
        self.pixel.update(size.x as f32, size.y as f32);
    }

    ///
    /// use line link point.
    ///
    pub fn line(&mut self, mut shape: Shape) {
        unsafe {
            self.use_def_program();

            self.send_vbo_shape(&mut shape);
            debug!("{}",Color::from_hex_str("00CCFF").unwrap().rgba_gl_vec4());
            self.gl.draw_arrays(glow::LINE_LOOP, 0, 360);
            debug!("ss -> {:?}",self.gl.get_debug_message_log(self.gl.get_error()));
        }
    }

    ///
    /// use line link point.
    /// the last point is linked to the first point.
    ///
    pub fn line_loop(&mut self, mut shape: Shape) {
        unsafe {
            self.use_def_program();

            self.send_vbo_shape(&mut shape);
            debug!("{}",Color::from_hex_str("00CCFF").unwrap().rgba_gl_vec4());
            self.gl.draw_arrays(glow::LINE, 0, 360);
        }
    }

    ///
    /// 填充矩形
    ///
    /// 支持如下属性
    /// radiu
    /// border
    ///
    pub fn fill(&mut self, mut shape: Shape, border: Option<Border>) {
        unsafe {
            self.use_def_program();

            if !shape.is_valid() {
                return;
            }

            self.send_vbo_shape(&mut shape);

            self.gl.draw_arrays(glow::QUADS, 0, shape.len() as i32 / 2);
            debug!("ss -> {:?}",self.gl.get_debug_message_log(self.gl.get_error()));
        }
    }

    /// 绘制文字
    pub fn text(&mut self, text: String, setting: PaintStyle) {}

    ///
    /// 绘制圆
    ///
    pub fn circle(&mut self, shape: Shape) {
        let gl = &self.gl;
        unsafe {
            self.use_def_program();


            let pi = f64::acos(-1.0);
            let radiu = 1.;


            // let mut vec:Vec<u8> = Vec::new();
            let mut vec = Vec::new();
            for i in 0..360 {
                let x = f64::cos(2. * pi / 360. * i as f64) * self.pixel.to_glx(radiu) as f64;
                let y = f64::sin(2. * pi / 360. * i as f64) * self.pixel.to_glx(radiu) as f64;
                // vec.extend_from_slice(bytes_of(&x));
                // vec.extend_from_slice(bytes_of(&y));
                vec.push(x);
                vec.push(y);
            }

            // 右上
            // vec.push(self.pixel.to_glx(400.));
            // vec.push(self.pixel.to_gly(200.));
            //
            // // 右下
            // vec.push(self.pixel.to_glx(400.));
            // vec.push(self.pixel.to_gly(400.));
            //
            // // 左下
            // vec.push(self.pixel.to_glx(200.));
            // vec.push(self.pixel.to_gly(400.));
            //
            // // 左上
            // vec.push(self.pixel.to_glx(200.));
            // vec.push(self.pixel.to_gly(200.));

            println!("{:?}", vec);

            let vao = gl.create_vertex_array();
            let vbo = gl.create_buffer();

            // 绑定vao
            gl.bind_vertex_array(vao.ok());

            // 绑定vbo
            gl.bind_buffer(glow::ARRAY_BUFFER, vbo.ok());
            gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, cast_slice(&vec), glow::STATIC_DRAW);

            // 告诉vao如何解释vbo的数据
            gl.vertex_attrib_pointer_f32(0, 2, glow::DOUBLE, false, 0, 0);
            gl.enable_vertex_attrib_array(0);

            //
            // let x = 100.;
            // let y = 100.;
            // let mut model = glm::Mat4::identity();
            //
            // let mode_size = Point::new(100., 100.);
            //
            //
            // model = glm::translate(&model, &glm::vec3(x, y, 0.0f32));
            // model = glm::translate(&model, &glm::vec3(0.5f32 * (mode_size.x as f32), 0.5f32 * (mode_size.y as f32), 0.0f32));
            // model = glm::rotate(&model, (0 as f32).to_radians(), &glm::vec3(0.0f32, 0.0f32, 1.0f32));
            // model = glm::translate(&model, &glm::vec3(-0.5f32 * (mode_size.x as f32), -0.5f32 * (mode_size.y as f32), 0.0f32));
            // model = glm::scale(&model, &glm::vec3(mode_size.x as f32, mode_size.y as f32, 1.0f32));
            // gl.uniform_matrix_4_f32_slice(gl.get_uniform_location(self.shader, &"model").as_ref(), false, model.as_slice());
            //
            // let projection = glm::ortho(
            //     0.0,
            //     self.pixel.screen_width,
            //     self.pixel.screen_height,
            //     0.0,
            //     -1.0,
            //     1.0);
            //
            // gl.uniform_matrix_4_f32_slice(gl.get_uniform_location(self.shader, &"projection").as_ref(), false, projection.as_slice());
            //
            // gl.draw_elements(glow::LINE_LOOP, 360, glow::UNSIGNED_INT, 0);
            gl.draw_arrays(glow::LINE_LOOP, 0, 360);
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

// tool
impl Renderer {
    pub unsafe fn use_def_program(&self) {
        self.use_program(Some(self.shader));
    }
    pub unsafe fn send_vbo_shape(&mut self, shape: &mut Shape) {
        for i in 0..shape.len() {
            if i % 2 == 0 {
                shape[i] = self.pixel.to_glx(shape[i]);
            } else {
                shape[i] = self.pixel.to_gly(shape[i]);
            }
        }

        let vao = self.gl.create_vertex_array();
        let vbo = self.gl.create_buffer();

        // 绑定vao
        self.gl.bind_vertex_array(vao.ok());

        // 绑定vbo
        self.gl.bind_buffer(glow::ARRAY_BUFFER, vbo.ok());
        self.gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, cast_slice(shape.deref()), glow::STATIC_DRAW);

        // 告诉vao如何解释vbo的数据
        self.gl.vertex_attrib_pointer_f32(0, 2, glow::FLOAT, false, 0, 0);
        self.gl.enable_vertex_attrib_array(0);
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