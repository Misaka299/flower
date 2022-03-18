use std::ops::{Deref, DerefMut};
use std::ptr::null;

use glow::{Context, HasContext};
use glutin::{ContextWrapper, PossiblyCurrent};
use takeable_option::Takeable;

use crate::{Control, ControlState, ControlType};

pub struct Window {
    title: String,
    control_state: ControlState,
    gl: Context,
    shader_version: String,
    pub(crate) window: Takeable<ContextWrapper<PossiblyCurrent, glutin::window::Window>>,
}

impl Window {
    pub fn create(state: ControlState, title: String, gl: Context, shader_version: String, window: ContextWrapper<PossiblyCurrent, glutin::window::Window>) -> Self {
        Window {
            title,
            control_state: state,
            gl,
            shader_version,
            window: Takeable::new(window),
        }
    }
}


impl Deref for Window {
    type Target = ControlState;

    fn deref(&self) -> &Self::Target {
        &self.control_state
    }
}

impl Window {
    // 发起绘制
    pub fn draw(&mut self) {
        unsafe { self.on_draw(&*null() as &Context); }
        for x in self.control_state.child.iter_mut() {
            x.draw(&self.gl);
        }
    }
}

impl DerefMut for Window {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self
    }
}

impl Control for Window {
    fn get_control_type(&self) -> ControlType {
        ControlType::Window
    }

    fn on_draw(&mut self, gl: &Context) {
        unsafe {
            if !self.window.is_current() {
                let wrapper = Takeable::take(&mut self.window);
                let wrapper = wrapper.make_current().expect("make current error!");
                self.window = Takeable::new(wrapper);
            }
            let gl = &self.gl;
            // println!("draw window_id : {:?} {:?}",self.id(), &gl.version());
            let vertex_array = gl
                .create_vertex_array()
                .expect("Cannot create vertex array");
            gl.bind_vertex_array(Some(vertex_array));

            let program = gl.create_program().expect("Cannot create program");

            let (vertex_shader_source, fragment_shader_source) = (
                r#"const vec2 verts[3] = vec2[3](
                vec2(0.5f, 1.0f),
                vec2(0.0f, 0.0f),
                vec2(1.0f, 0.0f)
            );
            out vec2 vert;
            void main() {
                vert = verts[gl_VertexID];
                gl_Position = vec4(vert - 0.5, 0.0, 1.0);
            }"#,
                r#"precision mediump float;
            in vec2 vert;
            out vec4 color;
            void main() {
                color = vec4(vert, 0.5, 1.0);
            }"#,
            );

            let shader_sources = [
                (glow::VERTEX_SHADER, vertex_shader_source),
                (glow::FRAGMENT_SHADER, fragment_shader_source),
            ];

            let mut shaders = Vec::with_capacity(shader_sources.len());

            for (shader_type, shader_source) in shader_sources.iter() {
                let shader = gl
                    .create_shader(*shader_type)
                    .expect("Cannot create shader");
                gl.shader_source(shader, &format!("{}\n{}", "#version 460", shader_source));
                gl.compile_shader(shader);
                if !gl.get_shader_compile_status(shader) {
                    panic!("{}", gl.get_shader_info_log(shader));
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

            gl.use_program(Some(program));
            gl.clear_color(0.1, 0.2, 0.3, 1.0);

            gl.clear(glow::COLOR_BUFFER_BIT);
            gl.draw_arrays(glow::TRIANGLES, 0, 3);

            // println!("error {}",gl.get_error());
        }
    }
}
