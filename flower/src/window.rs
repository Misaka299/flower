use std::ops::{Deref, DerefMut};
use std::ptr::null_mut;

use glutin::{ContextWrapper, PossiblyCurrent};
use glutin::event::MouseButton;
use glutin::event_loop::EventLoop;
use log::debug;
use takeable_option::Takeable;

use crate::{WINDOWS, WINDOWS_ID_MAP};
use crate::background::{Background, ImageSize};
use crate::control::{Control, ControlState, ControlType};
use crate::event::EventMessage;
use crate::graphics::rect::Rect;
use crate::graphics::Render;
use crate::graphics::renderer::default::Renderer;

#[derive(Debug, Clone, Copy)]
pub struct ButtonInfo {
    pub mouse_button: MouseButton,
    pub press_id: i32,
}

impl Default for ButtonInfo {
    fn default() -> Self {
        Self {
            mouse_button: MouseButton::Other(u16::MAX),
            press_id: 0,
        }
    }
}

#[derive(Default, Debug, Clone, Copy)]
pub struct MouseLocation {
    pub x: i32,
    pub y: i32,
}

pub struct Window {
    pub control_state: ControlState,
    renderer: Renderer,
    pub context_wrapper: Takeable<ContextWrapper<PossiblyCurrent, glutin::window::Window>>,
    pub focus_order_id: i32,
    pub active_id: i32,
    pub button_info: ButtonInfo,
    pub mouse_location: MouseLocation,
    pub background: Background,
}

impl Window {
    pub fn print(&self) {
        self.renderer.test();
        println!("{:?}", &self.context_wrapper);
    }
    pub fn create<T>(el: &EventLoop<T>, name: String, title: String) -> &mut Window {
        Self::create_with_control_type(ControlType::Control, el, name, title)
    }
    pub fn create_with_control_type<T>(control_type: ControlType, el: &EventLoop<T>, name: String, title: String) -> &mut Window {
        let mut state = ControlState::create(name.clone(), Rect {
            left: 0.0,
            top: 0.0,
            width: 1024.0,
            height: 768.0,
        }, false, control_type);
        // state.width = 1024;
        // state.height = 768;
        state.set_focus();
        let window_builder = glutin::window::WindowBuilder::new()
            .with_title(&title)
            .with_inner_size(glutin::dpi::LogicalSize::new(1024, 768));

        let window = glutin::ContextBuilder::new()
            .with_vsync(true)
            .with_multisampling(16)
            .build_windowed(window_builder, el)
            .unwrap();

        let id = window.window().id();
        let state_id = state.id;

        unsafe {
            let window = window.make_current().unwrap();

            WINDOWS.insert(state_id, Self {
                control_state: state,
                renderer: Renderer::create(),
                // px: PixelTool::create(1024, 768),
                context_wrapper: Takeable::new(window.make_current().unwrap()),
                focus_order_id: state_id,
                active_id: state_id,
                button_info: ButtonInfo::default(),
                mouse_location: MouseLocation::default(),
                background: Background::None,
            });
            WINDOWS_ID_MAP.insert(id, state_id);
            WINDOWS.get_mut(&state_id).unwrap()
        }
    }
}

impl Deref for Window {
    type Target = ControlState;

    fn deref(&self) -> &Self::Target {
        &self.control_state
    }
}

impl DerefMut for Window {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.control_state
    }
}

impl Control for Window {
    fn on_draw(&mut self, rdr: &mut Renderer) {
        unsafe {
            match self.background.clone() {
                Background::Image(image, size) => {
                    match size {
                        ImageSize::Size(width, height) => {
                            self.renderer.draw_image(image, Rect{
                                left: 0.0,
                                top: 0.0,
                                width: width as f32,
                                height: height as f32,
                            });
                        }
                        ImageSize::Cover => {
                            let rect = self.rect.clone();
                            self.renderer.draw_image(image, rect);
                        }
                    }
                }
                Background::None => {}
            }

            // if !self.context_wrapper.is_current() {
            //     let wrapper = Takeable::take(&mut self.context_wrapper);
            //     self.context_wrapper = Takeable::new(wrapper.make_current().expect("make current error!"));
            // }
            // let gl = &self.draw;
            // let px = &self.px;
            // gl.clear_color(0.1, 0.2, 0.3, 1.0);
            // gl.clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);
            // // let rect = &self.abs_rect();
            // // gl.viewport(rect.left as i32, self.height as i32 - rect.top as i32 - rect.height as i32, rect.width as i32, rect.height as i32);
            // let size = self.context_wrapper.window().inner_size();
            // gl.viewport(0, 0, size.width as i32, size.height as i32);
        }
    }

    fn on_event(&mut self, em: EventMessage) -> bool {
        todo!()
    }
}

// Control packaging method
impl Window {
    pub fn request_redraw(&self) {
        self.context_wrapper.window().request_redraw();
    }

    // 发起绘制
    pub fn draw(&mut self) {
        // debug!("draw all");
        // let now = minstant::Instant::now();
        self.renderer.begin_paint(&self.context_wrapper);
        self.renderer.test();
        unsafe { self.on_draw(&mut *null_mut() as &mut Renderer); }
        for x in self.control_state.child.iter_mut() {
            // x.canvas =
            self.renderer.new_buffer_canvas(x.id, x.rect.width as i32 + 1, x.rect.height as i32 + 1);
            x.draw(&mut self.renderer);
            self.renderer.refresh_canvas_to_window(None, x.rect.left as i32, x.rect.top as i32);
        }
        // self.context_wrapper.swap_buffers().unwrap();
        self.renderer.end_paint(&self.context_wrapper);
        // //println!("draw - {:?}", now.elapsed());
    }

    pub fn move_focus_to_specify_id_control(&mut self, id: i32) {
        let current_focus_order = self.focus_order_id;
        if let Some(control) = self.search_control_by_id(&id) {
            debug!("success search next_control id is {},set this control focus is true",control.id);
            control.set_focus();
            self.focus_order_id = control.focus_order;
        }
        self.rest_old_control_focus(current_focus_order);
    }

    pub fn move_focus_to_previous_control(&mut self) {
        let current_focus_order = self.focus_order_id;
        if let Some(id) = self.find_previous_focus_control(current_focus_order) {
            let next_control = self.search_control_by_id(&id).unwrap();
            debug!("success search next_control id is {},set this control focus is true",next_control.id);
            next_control.set_focus();
            self.focus_order_id = next_control.focus_order;
        }
        self.rest_old_control_focus(current_focus_order);
    }

    pub fn move_focus_to_next_control(&mut self) {
        let current_focus_order = self.focus_order_id;
        if let Some(id) = self.find_next_focus_control(current_focus_order) {
            let next_control = self.search_control_by_id(&id).unwrap();
            debug!("success search next_control id is {},set this control focus is true",next_control.id);
            next_control.set_focus();
            self.focus_order_id = next_control.focus_order;
        }
        self.rest_old_control_focus(current_focus_order);
    }

    pub fn rest_old_control_focus(&mut self, old_focus_order: i32) {
        debug!("keyboard input rest focus");
        if let Some(old_focus_control) = self.search_control_by_focus_order(old_focus_order) {
            debug!("success search focus_control id is {},set this control focus is false",old_focus_control.id);
            old_focus_control.cancel_focus();
        }
    }

    ///
    /// Search Control includes Windows
    ///
    pub fn search_control_by_id(&mut self, id: &i32) -> Option<&mut Box<dyn Control<Target=ControlState>>> {
        if &self.id == id {
            return None;
        }
        self.control_state.search_control_by_id(id)
    }

    pub fn search_control_by_focus_order(&mut self, order: i32) -> Option<&mut Box<dyn Control<Target=ControlState>>> {
        self.control_state.search_control_by_focus_order(order)
    }
}
