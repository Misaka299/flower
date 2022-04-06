use std::ops::{Deref, DerefMut};

use glow::{Context, HasContext};

use crate::color::Color;
use crate::Px;
use crate::rect::Rect;

pub struct Draw {
    gl: Context,
    window_height: Px,
}

impl Draw {
    pub fn new(gl: Context,window_height:Px) -> Draw {
        Self {
            gl,
            window_height
        }
    }
}

impl Draw {
    /// 使用
    ///
    pub fn create_canvas(&self, rect: &Rect) {
        println!("view -> {:?}", rect);
        unsafe { self.viewport(rect.left as i32, self.window_height as i32 - rect.top as i32 - rect.height as i32, rect.width as i32, rect.height as i32); }
    }

    pub fn rect(&mut self, rect: &Rect, color: &Color) {
        unsafe {
            let vertex_shader_source = r#"const vec2 verts[4] = vec2[4](
                vec2(-1.0f, 0.9999f),
                vec2(1.0f, 1.0f),
                vec2(1.0f, -0.9999f),
                vec2(-0.99999f, -1.0f)
            );
            out vec2 vert;
            void main() {
                vert = verts[gl_VertexID];
                gl_Position = vec4(vert, 0.0f, 1.0f);
            }"#;
            let fragment_shader_source =
                "".to_string() + r#"precision mediump float;
            in vec2 vert;
            out vec4 color;
            void main() {
                color = "# + &color.rgba_gl_vec4() + &"}";

            self.make_use_program(vertex_shader_source, fragment_shader_source.as_str());

            // self.gl.clear(glow::COLOR_BUFFER_BIT);
            self.gl.draw_arrays(glow::LINE_LOOP, 0, 4);
        }
    }

    pub fn fill(&mut self, rect: &Rect, color: &Color) {
        unsafe {
            let vertex_shader_source = r#"const vec2 verts[4] = vec2[4](
                vec2(-1.0f, 1.0f),
                vec2(1.0f, 1.0f),
                vec2(1.0f, -0.9999f),
                vec2(-0.99999f, -1.0f)
            );
            out vec2 vert;
            void main() {
                vert = verts[gl_VertexID];
                gl_Position = vec4(vert , 0.0, 1.0);
            }"#;
            let fragment_shader_source =
                "".to_string() + r#"precision mediump float;
            in vec2 vert;
            out vec4 color;
            void main() {
                color = "# + &color.rgba_gl_vec4() + &"}";
            //     r#"precision mediump float;
            // in vec2 vert;
            // out vec4 color;
            // void main() {
            //     color = vec4(0.0, 0.74902, 1.0, 1.0);
            // }"#.to_string();

            // println!("-----------{}", fragment_shader_source);

            self.make_use_program(vertex_shader_source, fragment_shader_source.as_str());

            // self.gl.clear(glow::COLOR_BUFFER_BIT);
            self.gl.draw_arrays(glow::QUADS, 0, 4);
        }
    }

    fn make_use_program(&mut self, vertex_shader_source: &str, fragment_shader_source: &str) {
        unsafe {
            let vertex_array = self.gl
                .create_vertex_array()
                .expect("Cannot create vertex array");
            self.gl.bind_vertex_array(Some(vertex_array));

            let program = self.gl.create_program().expect("Cannot create program");

            let shader_sources = [
                (glow::VERTEX_SHADER, vertex_shader_source),
                (glow::FRAGMENT_SHADER, fragment_shader_source),
            ];

            let mut shaders = Vec::with_capacity(shader_sources.len());

            for (shader_type, shader_source) in shader_sources.iter() {
                let shader = self.gl
                    .create_shader(*shader_type)
                    .expect("Cannot create shader");
                self.gl.shader_source(shader, &format!("{}\n{}", "#version 460", shader_source));
                self.gl.compile_shader(shader);
                if !self.gl.get_shader_compile_status(shader) {
                    panic!("{}", self.gl.get_shader_info_log(shader));
                }
                self.gl.attach_shader(program, shader);
                shaders.push(shader);
            }

            self.gl.link_program(program);
            if !self.gl.get_program_link_status(program) {
                panic!("{}", self.gl.get_program_info_log(program));
            }

            for shader in shaders {
                self.gl.detach_shader(program, shader);
                self.gl.delete_shader(shader);
            }
            self.gl.use_program(Some(program));
        }
    }
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