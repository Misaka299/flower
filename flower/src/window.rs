use std::ops::{Deref, DerefMut};
use std::ptr::null_mut;

use glow::{ HasContext};
use glutin::{ContextWrapper, PossiblyCurrent};
use glutin::event_loop::EventLoop;
use log::debug;
use takeable_option::Takeable;

use crate::{get_window_control_by_id, util, WINDOW_ID_MAP, WINDOW_NAME_MAP, WINDOWS};
use crate::control::{Control, ControlState, ControlType};
use crate::rect::Point;
use crate::render::render::Renderer;

pub struct Window {
    title: String,
    control_state: ControlState,
    gl: Renderer,
    shader_version: String,
    pub context_wrapper: Takeable<ContextWrapper<PossiblyCurrent, glutin::window::Window>>,
    pub(crate) focus_order_id: i32,
    pub(crate) active_id: i32,
}

impl Window {
    pub fn create<T>(el: &EventLoop<T>, name: String, title: String) -> &mut Window {
        Self::create_with_control_type(ControlType::Control, el, name, title)
    }

    pub fn create_with_control_type<T>(control_type: ControlType, el: &EventLoop<T>, name: String, title: String) -> &mut Window {
        let mut state = ControlState::create(name.clone(), false, control_type);
        state.width = 1024 as f32;
        state.height = 768 as f32;
        state.focus = true;
        let window_builder = glutin::window::WindowBuilder::new()
            .with_title(&title)
            .with_inner_size(glutin::dpi::LogicalSize::new(1024, 768));
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
            let size = Point::new(state.height, state.width);
            WINDOWS.push((state_id, Box::new(Window {
                title,
                control_state: state,
                gl: Renderer::new(gl, size),
                shader_version,
                context_wrapper: Takeable::new(window),
                focus_order_id: state_id,
                active_id: state_id,
            })));
            WINDOW_ID_MAP.insert(id, state_id);
            WINDOW_NAME_MAP.insert(name.clone(), state_id);
            let this_index = WINDOWS.binary_search_by(|(sid, _)| sid.cmp(&state_id)).unwrap();
            WINDOWS[this_index].1.downcast_mut::<Window>().unwrap()
        }
    }

    pub fn title(&self) -> &str {
        &self.title
    }
    pub fn control_state(&self) -> &ControlState {
        &self.control_state
    }
    pub fn gl(&self) -> &Renderer {
        &self.gl
    }
    pub fn shader_version(&self) -> &str {
        &self.shader_version
    }
    pub fn window(&self) -> &Takeable<ContextWrapper<PossiblyCurrent, glutin::window::Window>> {
        &self.context_wrapper
    }
}

/// get set
impl Window {
    pub(crate) fn set_height(&mut self, height: f32) {
        self.height = height;
        self.gl.update_window_size(Point::new(self.width, self.height));
    }
    pub(crate) fn set_width(&mut self, width: f32) {
        self.width = width;
        self.gl.update_window_size(Point::new(self.width, self.height));
    }
}

impl Deref for Window {
    type Target = ControlState;

    fn deref(&self) -> &Self::Target {
        &self.control_state
    }
}

// Control packaging method
impl Window {
    // 发起绘制
    pub fn draw(&mut self) {
        debug!("draw all");
        unsafe { self.on_draw(&mut *null_mut() as &mut Renderer); }
        for x in self.control_state.child.iter_mut() {
            x.draw(&mut self.gl);
        }
        self.context_wrapper.swap_buffers().unwrap();
    }

    pub fn move_focus_to_specify_id_control(&mut self, id: i32) {
        let current_focus_order = self.focus_order_id;
        if let Some(control) = self.search_control_by_id(&id) {
            debug!("success search next_control id is {},set this control focus is true",control.id());
            control.focus = true;
            self.focus_order_id = control.focus_order;
        }
        self.rest_old_control_focus(current_focus_order);
    }

    pub fn move_focus_to_previous_control(&mut self) {
        let current_focus_order = self.focus_order_id;
        if let Some(id) = self.find_previous_focus_control(current_focus_order) {
            let next_control = self.search_control_by_id(&id).unwrap();
            debug!("success search next_control id is {},set this control focus is true",next_control.id());
            next_control.focus = true;
            self.focus_order_id = next_control.focus_order;
        }
        self.rest_old_control_focus(current_focus_order);
    }

    pub fn move_focus_to_next_control(&mut self) {
        let current_focus_order = self.focus_order_id;
        if let Some(id) = self.find_next_focus_control(current_focus_order) {
            let next_control = self.search_control_by_id(&id).unwrap();
            debug!("success search next_control id is {},set this control focus is true",next_control.id());
            next_control.focus = true;
            self.focus_order_id = next_control.focus_order;
        }
        self.rest_old_control_focus(current_focus_order);
    }

    pub fn rest_old_control_focus(&mut self, old_focus_order: i32) {
        debug!("keyboard input rest focus");
        if let Some(old_focus_control) = self.search_control_by_focus_order(old_focus_order) {
            debug!("success search focus_control id is {},set this control focus is false",old_focus_control.id());
            old_focus_control.focus = false;
        }
    }

    ///
    /// Search Control includes Windows
    ///
    pub fn search_control_by_id(&mut self, id: &i32) -> Option<&mut Box<dyn Control<Target=ControlState>>> {
        if self.id == *id {
            return Some(get_window_control_by_id!(&id));
        }
        self.control_state.search_control_by_id(id)
    }

    pub fn search_control_by_focus_order(&mut self, order: i32) -> Option<&mut Box<dyn Control<Target=ControlState>>> {
        if self.focus_order == order {
            return Some(get_window_control_by_id!(&self.id));
        }
        self.control_state.search_control_by_focus_order(order)
    }
}

impl DerefMut for Window {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.control_state
    }
}

impl Control for Window {
    fn on_draw(&mut self, _: &mut Renderer) {
        unsafe {
            if !self.context_wrapper.is_current() {
                let wrapper = Takeable::take(&mut self.context_wrapper);
                let wrapper = wrapper.make_current().expect("make current error!");
                self.context_wrapper = Takeable::new(wrapper);
            }
            let gl = &self.gl;
            // gl.use_def_program();


            // println!("draw window_id : {:?} {:?}",self.id(), &gl.version());
            // let vertex_array = gl
            //     .create_vertex_array()
            //     .expect("Cannot create vertex array");
            // gl.bind_vertex_array(Some(vertex_array));
            //
            // let program = gl.create_program().expect("Cannot create program");
            //
            // let (vertex_shader_source, fragment_shader_source) = (
            //     r#"const vec2 verts[3] = vec2[3](
            //     vec2(0.5f, 1.0f),
            //     vec2(0.0f, 0.0f),
            //     vec2(1.0f, 0.0f)
            // );
            // out vec2 vert;
            // void main() {
            //     vert = verts[gl_VertexID];
            //     gl_Position = vec4(vert - 0.5, 0.0, 1.0);
            // }"#,
            //     r#"precision mediump float;
            // in vec2 vert;
            // out vec4 color;
            // void main() {
            //     color = vec4(vert, 0.5, 1.0);
            // }"#,
            // );
            //
            // let shader_sources = [
            //     (glow::VERTEX_SHADER, vertex_shader_source),
            //     (glow::FRAGMENT_SHADER, fragment_shader_source),
            // ];
            //
            // let mut shaders = Vec::with_capacity(shader_sources.len());
            //
            // for (shader_type, shader_source) in shader_sources.iter() {
            //     let shader = gl
            //         .create_shader(*shader_type)
            //         .expect("Cannot create shader");
            //     gl.shader_source(shader, &format!("{}\n{}", "#version 460", shader_source));
            //     gl.compile_shader(shader);
            //     if !gl.get_shader_compile_status(shader) {
            //         panic!("{}", gl.get_shader_info_log(shader));
            //     }
            //     gl.attach_shader(program, shader);
            //     shaders.push(shader);
            // }
            //
            // gl.link_program(program);
            // if !gl.get_program_link_status(program) {
            //     panic!("{}", gl.get_program_info_log(program));
            // }
            //
            // for shader in shaders {
            //     gl.detach_shader(program, shader);
            //     gl.delete_shader(shader);
            // }
            //
            // gl.use_program(Some(program));
            gl.clear_color(0.1, 0.2, 0.3, 1.0);

            gl.clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);
            // gl.clear_buffer_f32_slice(0, 0, 0);
            // gl.draw_arrays(glow::TRIANGLES, 0, 3);
            // gl.use_def_program();
            debug!("window[{}] draw",self.id());
            let rect = &self.abs_rect();
            debug!("create_canvas -> {} , {} , {} , {}",rect.left as i32, self.height as i32 - rect.top as i32 - rect.height as i32, rect.width as i32, rect.height as i32);
            gl.viewport(rect.left as i32, self.height as i32 - rect.top as i32 - rect.height as i32, rect.width as i32, rect.height as i32);
            // println!("error {}",gl.get_error());
        }
    }
}
// https://blog.csdn.net/tom_221x/article/details/51248832
