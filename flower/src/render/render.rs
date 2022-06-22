extern crate nalgebra_glm as glm;

use std::ops::{Deref, DerefMut};

use bytemuck::cast_slice;
use glow::{Context, HasContext, NativeVertexArray, Program};
use log::debug;

use crate::rect::Point;
use crate::render::border::Border;
use crate::render::color::Color;
use crate::render::pixel_tool::PixelTool;
use crate::render::shape::{Shape, ShapeType};

pub struct PaintStyle {
    radiu: f32,
    border: Border,
}

pub struct Renderer {
    gl: Context,
    pixel: PixelTool,
    // vao: Result<NativeVertexArray, String>,
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

            let vao = gl.create_vertex_array();
            // 绑定vao
            gl.bind_vertex_array(vao.ok());

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

            let count = self.send_vbo_shape(&mut shape);

            self.gl.draw_arrays(glow::LINE, 0, count);
            debug!("draw debug message -> {:?}",self.gl.get_debug_message_log(self.gl.get_error()));
        }
    }

    ///
    /// use line link point.
    /// the last point is linked to the first point.
    ///
    pub fn line_loop(&mut self, mut shape: Shape) {
        unsafe {
            self.use_def_program();

            let count = self.send_vbo_shape(&mut shape);
            debug!("{}",Color::from_hex_str("00CCFF").unwrap().rgba_gl_vec4());
            self.gl.draw_arrays(glow::LINE_LOOP, 0, count);
            debug!("draw debug message -> {:?}",self.gl.get_debug_message_log(self.gl.get_error()));
        }
    }

    ///
    /// fill shape.
    ///
    pub fn fill(&mut self, mut shape: Shape, border: Option<Border>) {
        unsafe {
            self.use_def_program();

            let count = self.send_vbo_shape(&mut shape);

            self.gl.draw_arrays(glow::QUADS, 0, count);
            debug!("draw debug message -> {:?}",self.gl.get_debug_message_log(self.gl.get_error()));
        }
    }

    /// 绘制文字
    pub fn text(&mut self, text: String, setting: PaintStyle) {}

    /// 缺失参数,绘制方式
    pub fn image(&mut self, image: Vec<u8>) {}
}

// tool
impl Renderer {
    pub unsafe fn use_def_program(&self) {
        self.use_program(Some(self.shader));
    }
    pub unsafe fn send_vbo_shape(&mut self, shape: &mut Shape) -> i32 {
        let mut vec: Vec<f32>;
        match shape.shape_type {
            ShapeType::Sector => {
                let glx = self.pixel.to_glx(shape.vertex[0]);
                let gly = self.pixel.to_gly(shape.vertex[1]);

                let radiu_x = (1. + self.pixel.to_glx(shape.vertex[2]));
                let radiu_y = (1. - self.pixel.to_gly(shape.vertex[3]));

                vec = Vec::new();
                let mut angle_start = (shape.vertex[4] * 100.) as i32;
                let mut angle_end = (shape.vertex[5] * 100.) as i32;

                let angle_count = 36000;
                if angle_end - angle_start < angle_count {
                    vec.push(glx);
                    vec.push(gly);
                }
                println!("{} {} {}", angle_start, angle_end, angle_count);
                for i in angle_start..angle_end {
                    use std::f32::consts;
                    let x = f32::cos(consts::PI / angle_count as f32 * (2 * i) as f32) * radiu_x;
                    let y = f32::sin(consts::PI / angle_count as f32 * (2 * i) as f32) * radiu_y;

                    vec.push(x + glx);
                    vec.push(y + gly);
                }
            }
            ShapeType::Other => {
                for i in 0..shape.len() {
                    if i % 2 == 0 {
                        shape[i] = self.pixel.to_glx(shape[i]);
                    } else {
                        shape[i] = self.pixel.to_gly(shape[i]);
                    }
                }
                vec = shape.vertex.clone();
            }
        }


        let vbo = self.gl.create_buffer();


        // 绑定vbo
        self.gl.bind_buffer(glow::ARRAY_BUFFER, vbo.ok());
        self.gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, cast_slice(&vec), glow::STATIC_DRAW);

        // 告诉vao如何解释vbo的数据
        self.gl.vertex_attrib_pointer_f32(0, 2, glow::FLOAT, false, 0, 0);
        self.gl.enable_vertex_attrib_array(0);

        // 解绑vbo
        self.gl.bind_buffer(glow::ARRAY_BUFFER, None);
        vec.len() as i32 / 2
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