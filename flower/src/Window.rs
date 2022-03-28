use std::ops::{Deref, DerefMut};
use std::ptr::null;

use glow::{Context, HasContext};
use glutin::{ContextWrapper, PossiblyCurrent};
use glutin::event_loop::EventLoop;
use takeable_option::Takeable;
use crate::control::{Control, ControlState, ControlType};
use crate::{util, WINDOW_ID_MAP, WINDOW_NAME_MAP, WINDOWS};


pub struct Window {
    title: String,
    control_state: ControlState,
    gl: Context,
    shader_version: String,
    pub(crate) window: Takeable<ContextWrapper<PossiblyCurrent, glutin::window::Window>>,
}

impl Window {
    pub fn create<T>(el: &EventLoop<T>, name: String, title: String) -> &mut Window {
        Self::create_with_control_type(ControlType::Control, el, name, title)
    }

    pub fn create_with_control_type<T>(control_type: ControlType, el: &EventLoop<T>, name: String, title: String) -> &mut Window {
        let state = ControlState::create(name.clone(), vec![], control_type, 0, 0);
        let window_builder = glutin::window::WindowBuilder::new()
            .with_title(&title)
            .with_inner_size(glutin::dpi::LogicalSize::new(1024.0, 768.0));
        unsafe {
            let window = glutin::ContextBuilder::new()
                .with_vsync(true)
                .build_windowed(window_builder, el)
                .unwrap()
                .make_current()
                .unwrap();
            let gl = glow::Context::from_loader_function(|s| window.get_proc_address(s) as *const _);
            let shader_version = util::find_version(gl.get_parameter_string(glow::VERSION));
            //can i use this version?
            println!("{:?}", &shader_version);
            let id = window.window().id();
            let state_id = state.id;
            WINDOWS.push((state_id.clone(), Window {
                title,
                control_state: state,
                gl,
                shader_version,
                window: Takeable::new(window),
            }));
            WINDOW_ID_MAP.insert(id, state_id);
            WINDOW_NAME_MAP.insert(name.clone(),state_id);
            // get_window_by_id(state_id)
            let this_index = WINDOWS.binary_search_by(|(sid, _)| sid.cmp(&state_id)).unwrap();
            &mut WINDOWS[this_index].1
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
        &mut self.control_state
    }
}

impl Control for Window {
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
