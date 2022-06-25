use std::ops::{Deref, DerefMut};

use bytemuck::cast_slice;
use glow::{Context, HasContext, Program};
use log::debug;

use crate::rect::Point;
use crate::render::border::Border;
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
    const ROUND_VERTEX_MAX_COUNT: i32 = 36000;
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

    /// 绘制文字
    pub fn text(&mut self, text: String, setting: PaintStyle) {}

    /// 缺失参数,绘制方式
    pub fn image(&mut self, image: Vec<u8>) {}

    pub unsafe fn use_def_program(&self) {
        self.use_program(Some(self.shader));
    }
    ///
    /// mode:
    /// 不知道为什么线段不能用LINE_LOOP LINE 必须得用LINES
    /// GL_LINE_STRIP 多点链接，第一个点会和最后一个点不会链接起来
    /// LINE_LOOP 多点链接，第一个点会和最后一个点链接起来
    /// LINES 绘制线时, 会将从 glBegin 到 glEnd 之间的所有的点都绘制出来 ;注意必须成对设置 , 如果设置 奇数个点 , 最后一个点会被丢弃;
    ///
    /// 下面是各个图形推荐的一些mode配置
    /// TRIANGLE_FAN 填充圆、扇形
    /// LINE_LOOP 空心圆，空心扇形
    /// LINES 线段
    ///
    pub fn draw_shape(&self, shape: Shape, mode: u32) {
        unsafe {
            self.use_def_program();
            let mut vec: Vec<f32>;
            match shape {
                Shape::Sector { origin_x, origin_y, radiu_x, radiu_y, start_angle, end_angle } => {
                    vec = Vec::new();
                    self.calc_round_edge(&mut vec, origin_x, origin_y, radiu_x, radiu_y, start_angle, end_angle, true);
                }
                Shape::Other { mut vertex } => {
                    for i in 0..vertex.len() {
                        if i % 2 == 0 {
                            vertex[i] = self.pixel.to_glx(vertex[i]);
                        } else {
                            vertex[i] = self.pixel.to_gly(vertex[i]);
                        }
                    }
                    vec = vertex.clone();
                }
                Shape::Line { start_x, start_y, end_x, end_y } => {
                    vec = Vec::new();
                    println!("{}-{}-{}-{}", start_x, start_y, end_x, end_y);
                    vec.push(self.pixel.to_glx(start_x));
                    vec.push(self.pixel.to_gly(start_y));
                    vec.push(self.pixel.to_glx(end_x));
                    vec.push(self.pixel.to_gly(end_y));
                }
                Shape::Rect { left, top, width, height, radiu_left_top, radiu_left_bottom, radiu_right_top, radiu_right_bottom } => {
                    vec = Vec::new();
                    let right_angle = (360 / 4) as f32;
                    if radiu_left_top == 0. {
                        vec.push(self.pixel.to_glx(left));
                        vec.push(self.pixel.to_gly(top));
                    } else {
                        self.calc_round_edge(&mut vec, left + radiu_left_top, top + radiu_left_top, radiu_left_top, radiu_left_top, right_angle * 1., right_angle * 2., false);
                    }
                    if radiu_left_bottom == 0. {
                    vec.push(self.pixel.to_glx(left));
                    vec.push(self.pixel.to_gly(top + height));
                    } else {
                        self.calc_round_edge(&mut vec, left + radiu_left_bottom, top + height - radiu_left_bottom, radiu_left_bottom, radiu_left_bottom, right_angle * 2., right_angle * 3.,false);
                    }
                    if radiu_right_bottom == 0. {
                    vec.push(self.pixel.to_glx(left + width));
                    vec.push(self.pixel.to_gly(top + height));
                    } else {
                        self.calc_round_edge(&mut vec, left + width - radiu_left_bottom, top + height - radiu_left_bottom, radiu_right_bottom, radiu_right_bottom,right_angle * 3., right_angle * 4.,false);
                    }
                    if radiu_right_top == 0. {
                    vec.push(self.pixel.to_glx(left + width));
                    vec.push(self.pixel.to_gly(top));
                    } else {
                        self.calc_round_edge(&mut vec, left + width - radiu_left_bottom, top + radiu_left_bottom, radiu_right_top, radiu_right_top, right_angle * 0., right_angle * 1.,false);
                    }
                }
            }

            let vbo = self.gl.create_buffer().unwrap();

            // 绑定vbo
            self.gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));
            self.gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, cast_slice(&vec), glow::STATIC_DRAW);

            // 告诉vao如何解释vbo的数据
            self.gl.vertex_attrib_pointer_f32(0, 2, glow::FLOAT, false, 0, 0);
            self.gl.enable_vertex_attrib_array(0);

            // 解绑vbo
            self.gl.bind_buffer(glow::ARRAY_BUFFER, None);
            self.gl.draw_arrays(mode, 0, vec.len() as i32 / 2);
            self.gl.delete_buffer(vbo);
            debug!("draw debug message -> {:?}",self.gl.get_debug_message_log(self.gl.get_error()));
        }
    }

    unsafe fn calc_round_edge(&self, vec: &mut Vec<f32>, origin_x: f32, origin_y: f32, radiu_x: f32, radiu_y: f32, start_angle: f32, end_angle: f32, need_center: bool) {
        let glx = self.pixel.to_glx(origin_x);
        let gly = self.pixel.to_gly(origin_y);

        let radiu_x = 1. + self.pixel.to_glx(radiu_x);
        let radiu_y = 1. - self.pixel.to_gly(radiu_y);

        let mut angle_start = (start_angle * 100.) as i32;
        let mut angle_end = (end_angle * 100.) as i32;

        if need_center && angle_end - angle_start < Renderer::ROUND_VERTEX_MAX_COUNT {
            vec.push(glx);
            vec.push(gly);
        }
        for i in angle_start..angle_end {
            use std::f32::consts;
            let x = f32::cos(consts::PI / Renderer::ROUND_VERTEX_MAX_COUNT as f32 * (2 * i) as f32) * radiu_x;
            let y = f32::sin(consts::PI / Renderer::ROUND_VERTEX_MAX_COUNT as f32 * (2 * i) as f32) * radiu_y;

            vec.push(x + glx);
            vec.push(y + gly);
        }
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