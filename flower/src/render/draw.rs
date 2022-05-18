use std::ops::{Deref, DerefMut};

use glow::{Context, HasContext, MAX_VIEWPORTS, Program};
use log::debug;

use crate::Px;
use crate::rect::Rect;

// pub static mut SHADER: Option<Program> = None;

pub struct Draw {
    gl: Context,
    pub(crate) window_height: Px,
    pub(crate) shader: Option<Program>,
}

impl Draw {
    pub fn new(gl: Context, window_height: Px) -> Draw {
        unsafe {
            // if SHADER.is_none() {
            let vertex_array = gl
                .create_vertex_array()
                .expect("Cannot create vertex array");
            gl.bind_vertex_array(Some(vertex_array));

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

            // println!("max viewports {}",gl.get_parameter_i32(MAX_VIEWPORTS));

            Self {
                gl,
                window_height,
                shader: Some(program),
            }
        }
    }
}

impl Draw {
    /// 使用
    ///
    pub fn create_canvas(&self, rect: &Rect) {
        println!("view -> {:?}", rect);
        unsafe {
            debug!("create_canvas -> {} , {} , {} , {}",rect.left as i32, self.window_height as i32 - rect.top as i32 - rect.height as i32, rect.width as i32, rect.height as i32);
            self.viewport(rect.left as i32, self.window_height as i32 - rect.top as i32 - rect.height as i32, rect.width as i32, rect.height as i32);
        }
    }

    pub fn use_def_program(&self) {
        unsafe {
            self.use_program(self.shader);
        }
    }

    //
    // pub fn rect(&mut self, rect: &Rect, color: &Color) {
    //     unsafe {
    //         let vertex_shader_source = r#"const vec2 verts[4] = vec2[4](
    //             vec2(-1.0f, 0.9999f),
    //             vec2(1.0f, 1.0f),
    //             vec2(1.0f, -0.9999f),
    //             vec2(-0.99999f, -1.0f)
    //         );
    //         out vec2 vert;
    //         void main() {
    //             vert = verts[gl_VertexID];
    //             gl_Position = vec4(vert, 0.0f, 1.0f);
    //         }"#;
    //         let fragment_shader_source =
    //             "".to_string() + r#"precision mediump float;
    //         in vec2 vert;
    //         out vec4 color;
    //         void main() {
    //             color = "# + &color.rgba_gl_vec4() + &"}";
    //
    //         let option = self.create_link_program(vertex_shader_source, fragment_shader_source.as_str());
    //         self.gl.use_program(option);
    //         // self.gl.clear(glow::COLOR_BUFFER_BIT);
    //         self.gl.draw_arrays(glow::LINE_LOOP, 0, 4);
    //     }
    // }
    //
    // pub fn fill(&mut self, rect: &Rect, color: &Color) {
    //     unsafe {
    //         let vertex_shader_source = r#"const vec2 verts[4] = vec2[4](
    //             vec2(-1.0f, 1.0f),
    //             vec2(1.0f, 1.0f),
    //             vec2(1.0f, -0.9999f),
    //             vec2(-0.99999f, -1.0f)
    //         );
    //         out vec2 vert;
    //         void main() {
    //             vert = verts[gl_VertexID];
    //             gl_Position = vec4(vert , 0.0, 1.0);
    //         }"#;
    //         let fragment_shader_source =
    //             "".to_string() + r#"precision mediump float;
    //         in vec2 vert;
    //         out vec4 color;
    //         void main() {
    //             color = "# + &color.rgba_gl_vec4() + &"}";
    //         //     r#"precision mediump float;
    //         // in vec2 vert;
    //         // out vec4 color;
    //         // void main() {
    //         //     color = vec4(0.0, 0.74902, 1.0, 1.0);
    //         // }"#.to_string();
    //
    //         // println!("-----------{}", fragment_shader_source);
    //
    //         let program = self.create_link_program(vertex_shader_source, fragment_shader_source.as_str());
    //
    //         self.gl.use_program(program);
    //         // self.gl.clear(glow::COLOR_BUFFER_BIT);
    //         self.gl.draw_arrays(glow::QUADS, 0, 4);
    //     }
    // }
}

impl Deref for Draw {
    type Target = Context;

    fn deref(&self) -> &Self::Target {
        &self.gl
    }
}

impl DerefMut for Draw {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.gl
    }
}