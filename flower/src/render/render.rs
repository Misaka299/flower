use std::ops::{Deref, DerefMut};
use std::vec;

use bytemuck::cast_slice;
use glow::{Context, HasContext, NativeBuffer, Program};
use log::debug;

use crate::rect::Point;
use crate::render::border::Border;
use crate::render::fill::{Align, Fill, Image, ZoomType};
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

#[repr(i32)]
pub enum FragType {
    Color = 1,
    Texture = 2,
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
    /// 下面是各个图形的一些本人已知mode配置
    /// TRIANGLE_FAN 填充圆、扇形
    /// LINE_LOOP 空心圆，空心扇形
    /// LINES 线段
    ///
    pub fn draw_shape(&self, shape: Shape, mode: u32, fill: Fill) {
        unsafe {
            self.use_def_program();
            let mut vertex: Vec<f32>;
            let mut fragcoord: Vec<f32> = Vec::new();
            match shape {
                Shape::Sector { origin_x, origin_y, radiu_x, radiu_y, start_angle, end_angle } => {
                    vertex = Vec::new();
                    self.calc_round_edge(&mut vertex, origin_x, origin_y, radiu_x, radiu_y, start_angle, end_angle, true);
                }
                Shape::Other { vertex: mut vec } => {
                    for i in 0..vec.len() {
                        if i % 2 == 0 {
                            vec[i] = self.pixel.to_gl_x(vec[i]);
                        } else {
                            vec[i] = self.pixel.to_gl_y(vec[i]);
                        }
                    }
                    vertex = vec.clone();
                }
                Shape::Line { start_x, start_y, end_x, end_y } => {
                    vertex = Vec::new();
                    vertex.push(self.pixel.to_gl_x(start_x));
                    vertex.push(self.pixel.to_gl_y(start_y));
                    vertex.push(self.pixel.to_gl_x(end_x));
                    vertex.push(self.pixel.to_gl_y(end_y));
                }
                Shape::Rect { left, top, width, height, radiu_left_top, radiu_left_bottom, radiu_right_top, radiu_right_bottom } => {
                    vertex = Vec::new();
                    let right_angle = (360 / 4) as f32;
                    if radiu_left_top == 0. {
                        vertex.push(self.pixel.to_gl_x(left));
                        vertex.push(self.pixel.to_gl_y(top));
                    } else {
                        self.calc_round_edge(&mut vertex, left + radiu_left_top, top + radiu_left_top, radiu_left_top, radiu_left_top, right_angle * 1., right_angle * 2., false);
                    }
                    if radiu_left_bottom == 0. {
                        vertex.push(self.pixel.to_gl_x(left));
                        vertex.push(self.pixel.to_gl_y(top + height));
                    } else {
                        self.calc_round_edge(&mut vertex, left + radiu_left_bottom, top + height - radiu_left_bottom, radiu_left_bottom, radiu_left_bottom, right_angle * 2., right_angle * 3., false);
                    }
                    if radiu_right_bottom == 0. {
                        vertex.push(self.pixel.to_gl_x(left + width));
                        vertex.push(self.pixel.to_gl_y(top + height));
                    } else {
                        self.calc_round_edge(&mut vertex, left + width - radiu_left_bottom, top + height - radiu_left_bottom, radiu_right_bottom, radiu_right_bottom, right_angle * 3., right_angle * 4., false);
                    }
                    if radiu_right_top == 0. {
                        vertex.push(self.pixel.to_gl_x(left + width));
                        vertex.push(self.pixel.to_gl_y(top));
                    } else {
                        self.calc_round_edge(&mut vertex, left + width - radiu_left_bottom, top + radiu_left_bottom, radiu_right_top, radiu_right_top, right_angle * 0., right_angle * 1., false);
                    }
                    println!("left {:?} ", vertex);
                    // match &fill {
                    //     Fill::Image(img) => {
                    //
                    //         match img.zoom_type {
                    //             ZoomType::Tile(width, height) => {
                    //
                    //             }
                    //             ZoomType::Zoom => {}
                    //             ZoomType::Repeat(width, height) => {}
                    //             ZoomType::ClampToBorder(width, height, color) => {}
                    //         }
                    //         // match &img.align {
                    //         //     Align::Center => {}
                    //         //     Align::Left => {}
                    //         //     Align::Top => {}
                    //         //     Align::Right => {}
                    //         //     Align::Bottom => {}
                    //         //     Align::LeftTop => {}
                    //         //     Align::RightTop => {}
                    //         //     Align::LeftBottom => {
                    //         //
                    //         //         // fragcoord = Vec::new();
                    //         //         // 左下
                    //         //         fragcoord.push(0.);
                    //         //         fragcoord.push(0.);
                    //         //
                    //         //         // 左上
                    //         //         fragcoord.push(0.);
                    //         //         fragcoord.push(1.);
                    //         //
                    //         //         // 右上
                    //         //         fragcoord.push(1.);
                    //         //         fragcoord.push(1.);
                    //         //
                    //         //         // 右下
                    //         //         fragcoord.push(1.);
                    //         //         fragcoord.push(0.);
                    //         //     }
                    //         //     Align::RightBottom => {}
                    //         // }
                    //     }
                    //     _ => {}
                    // }
                }
            }

            // let vao = self.gl.create_vertex_array().unwrap();
            // self.gl.bind_vertex_array(Some(vao));

            let mut vbo_list = Vec::<Option<NativeBuffer>>::new();

            let vbo = self.gl.create_buffer().unwrap();

            // 绑定vbo
            self.gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));
            self.gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, cast_slice(&vertex), glow::STATIC_DRAW);

            // 告诉vao如何解释vbo的数据
            self.gl.vertex_attrib_pointer_f32(0, 2, glow::FLOAT, false, 0, 0);
            self.gl.enable_vertex_attrib_array(0);
            self.gl.bind_buffer(glow::ARRAY_BUFFER, None);
            vbo_list.push(Some(vbo));
            // self.gl.delete_buffer(vbo);

            let mut opt_texture = None;
            let mut frag_type = FragType::Color as i32;
            match fill {
                Fill::Image(img) => {
                    frag_type = frag_type | FragType::Texture as i32;
                    opt_texture = self.gl.create_texture().ok();
                    self.gl.bind_texture(glow::TEXTURE_2D, opt_texture);

                    let frag_coord = img.texture_coord();

                    let vbo1 = self.gl.create_buffer().unwrap();
                    self.gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo1));

                    // 为当前绑定的纹理对象设置环绕、过滤方式
                    // self.gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_S, glow::MIRRORED_REPEAT as i32);
                    // self.gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_T, glow::MIRRORED_REPEAT as i32);
                    // //
                    // self.gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_MIN_FILTER, glow::LINEAR as i32);
                    // self.gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_MAG_FILTER, glow::LINEAR as i32);
                    self.gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_S, glow::REPEAT as i32);
                    self.gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_T, glow::REPEAT as i32);
                    //
                    self.gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_MIN_FILTER, glow::LINEAR as i32);
                    self.gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_MAG_FILTER, glow::LINEAR as i32);
                    println!("fragcoord -> {:?}", frag_coord);
                    self.gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, cast_slice(&frag_coord), glow::STATIC_DRAW);

                    // 告诉vao如何解释vbo的数据
                    self.gl.vertex_attrib_pointer_f32(1, 2, glow::FLOAT, false, 0, 0);
                    self.gl.enable_vertex_attrib_array(1);

                    self.gl.bind_buffer(glow::ARRAY_BUFFER, None);
                    // self.gl.delete_buffer(vbo);
                    vbo_list.push(Some(vbo));

                    // 加载并生成纹理
                    let image = img.src;

                    let data = image.as_bytes();
                    self.gl.tex_image_2d(glow::TEXTURE_2D, 0, glow::RGBA as i32, image.width() as i32, image.height() as i32, 0, glow::RGBA, glow::UNSIGNED_BYTE, Some(data));

                    self.gl.generate_mipmap(glow::TEXTURE_2D);
                }
                Fill::Color(color) => {
                    let mut frag_coord: Vec<f32> = Vec::new();
                    for i in 0..(vertex.len() as i32 / 2) {
                        frag_coord.append(&mut color.rgba_gl_vec());
                    }
                    let vbo1 = self.gl.create_buffer().unwrap();
                    self.gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo1));
                    self.gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, cast_slice(&frag_coord), glow::STATIC_DRAW);
                    // 告诉vao如何解释vbo的数据
                    self.gl.vertex_attrib_pointer_f32(1, 4, glow::FLOAT, false, 0, 0);
                    self.gl.enable_vertex_attrib_array(1);

                    self.gl.bind_buffer(glow::ARRAY_BUFFER, None);
                    vbo_list.push(Some(vbo1));
                }
            }
            self.gl.uniform_1_i32(self.gl.get_uniform_location(self.shader, "type").as_ref(), frag_type);
            // gl.draw_arrays(glow::QUADS, 0, 4);
            println!("frag_type {:?} ", frag_type);
            self.gl.draw_arrays(mode, 0, vertex.len() as i32 / 2);
            if let Some(texture) = opt_texture {
                self.gl.delete_texture(texture);
                opt_texture = None;
            }
            for mut x in vbo_list {
                if let Some(vbo) = x {
                    self.gl.delete_buffer(vbo);
                    x = None;
                }
            }

            // self.gl.enable_vertex_attrib_array(0);
            // self.gl.delete_buffer(vbo);
            // self.gl.delete_buffer(vbo1);


            debug!("draw debug message -> {:?}",self.gl.get_debug_message_log(self.gl.get_error()));
        }
    }

    unsafe fn calc_round_edge(&self, vec: &mut Vec<f32>, origin_x: f32, origin_y: f32, radiu_x: f32, radiu_y: f32, start_angle: f32, end_angle: f32, need_center: bool) {
        let gl_x = self.pixel.to_gl_x(origin_x);
        let gl_y = self.pixel.to_gl_y(origin_y);

        let radiu_x = 1. + self.pixel.to_gl_x(radiu_x);
        let radiu_y = 1. - self.pixel.to_gl_y(radiu_y);

        let mut angle_start = (start_angle * 100.) as i32;
        let mut angle_end = (end_angle * 100.) as i32;

        if need_center && angle_end - angle_start < Renderer::ROUND_VERTEX_MAX_COUNT {
            vec.push(gl_x);
            vec.push(gl_y);
        }
        for i in angle_start..angle_end {
            use std::f32::consts;
            let x = f32::cos(consts::PI / Renderer::ROUND_VERTEX_MAX_COUNT as f32 * (2 * i) as f32) * radiu_x;
            let y = f32::sin(consts::PI / Renderer::ROUND_VERTEX_MAX_COUNT as f32 * (2 * i) as f32) * radiu_y;

            vec.push(x + gl_x);
            vec.push(y + gl_y);
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