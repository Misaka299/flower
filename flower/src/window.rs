use log::debug;
use takeable_option::Takeable;

use flower_base::{ControlType, glutin};
use flower_base::background::{Background, ImageSize};
use flower_base::control::Control;
use flower_base::event::EventMessage;
use flower_base::glutin::ContextWrapper;
use flower_base::glutin::event::MouseButton;
use flower_base::glutin::event_loop::EventLoop;
use flower_base::glutin::PossiblyCurrent;
use flower_base::graphics::Render;
use flower_base::graphics::renderer::Renderer;
use flower_macro::control;

use crate::{WINDOWS, WINDOWS_ID_MAP};

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

#[control]
pub struct Window {
    pub context_wrapper: Takeable<ContextWrapper<PossiblyCurrent, glutin::window::Window>>,
    pub focus_order_id: i32,
    pub active_id: i32,
    pub button_info: ButtonInfo,
    pub mouse_location: MouseLocation,
    pub background: Background,
}

impl Window {
    pub fn create<T>(el: &EventLoop<T>, name: String, title: String) -> &mut Window {
        Self::create_with_control_type(ControlType::Control, el, name, title)
    }

    pub fn create_with_control_type<T>(control_type: ControlType, el: &EventLoop<T>, name: String, title: String) -> &mut Window {
        let window_builder = glutin::window::WindowBuilder::new()
            .with_title(&title)
            .with_inner_size(glutin::dpi::LogicalSize::new(1024, 768));

        let window = glutin::ContextBuilder::new()
            .with_vsync(true)
            .with_multisampling(16)
            .build_windowed(window_builder, el)
            .unwrap();

        let id = window.window().id();

        unsafe {
            let window = window.make_current().unwrap();

            let mut rect = Rect {
                left: 0.0,
                top: 0.0,
                width: window.window().outer_size().width as f32,
                height: window.window().outer_size().height as f32,
            };
            if let Ok(pp) = window.window().outer_position() {
                rect.left = pp.x as f32;
                rect.top = pp.y as f32;
            }

            let mut window = Window::create_control("Window".to_string(),
                                                    rect,
                                                    Takeable::new(window),
                                                    0,
                                                    0,
                                                    Default::default(),
                                                    Default::default(),
                                                    Background::None);
            // 激活id是自身,避免绘制闪烁
            window.active_id = window.id;
            let control_id = window.id();
            WINDOWS.insert(control_id,
                           window);
            WINDOWS_ID_MAP.insert(id, control_id);
            WINDOWS.get_mut(&control_id).unwrap()
        }
    }

    pub fn resize(&mut self, x: f32, y: f32) {
        self.rect.width = x;
        self.rect.height = y;
    }
}

impl Control for Window {
    fn in_scope(&self, x: f32, y: f32) -> bool {
        debug!("x->{} y->{}",x,y);
        return 0. <= x &&
            0. + self.width() >= x &&
            0. <= y &&
            0. + self.height() >= y
        ;
    }

    fn on_draw(&mut self, rdr: &mut Renderer) {
        unsafe {
            match self.background.clone() {
                Background::Image(image, size) => {
                    match size {
                        ImageSize::Size(width, height) => {
                            rdr.draw_image(image, Rect {
                                left: 0.0,
                                top: 0.0,
                                width: width as f32,
                                height: height as f32,
                            });
                        }
                        ImageSize::Cover => {
                            let rect = self.rect.clone_wh();
                            rdr.draw_image(image, rect);
                        }
                    }
                }
                _ => {}
            }
        }
    }

    fn on_event(&mut self, em: EventMessage) -> bool {
        true
    }
}

// Control packaging method
impl Window {
    pub fn request_redraw(&self) {
        self.context_wrapper.window().request_redraw();
    }

    // // 发起绘制
    // pub fn draw(&mut self) {
    //     // debug!("draw all");
    //     // let now = minstant::Instant::now();
    //     self.renderer.begin_paint(&self.context_wrapper);
    //     self.renderer.test();
    //     unsafe { self.on_draw(&mut *null_mut() as &mut Renderer); }
    //     for x in self.child.iter_mut() {
    //         // x.canvas =
    //         self.renderer.new_buffer_canvas(x.id(), x.width() as i32 + 1, x.height() as i32 + 1);
    //         x.draw(&mut self.renderer);
    //         self.renderer.refresh_canvas_to_window(None, x.left() as i32, x.top() as i32);
    //     }
    //     // self.context_wrapper.swap_buffers().unwrap();
    //     self.renderer.end_paint(&self.context_wrapper);
    //     // //println!("draw - {:?}", now.elapsed());
    // }

    pub fn move_focus_to_specify_id_control(&mut self, id: i32) {
        let current_focus_order = self.focus_order_id;
        if let Some(control) = self.search_control_by_id(&id) {
            debug!("success search next_control id is {},set this control focus is true",control.id());
            control.set_focus();
            self.focus_order_id = control.focus_order();
        }
        self.rest_old_control_focus(current_focus_order);
    }

    pub fn move_focus_to_previous_control(&mut self) {
        let current_focus_order = self.focus_order_id;
        if let Some(id) = self.find_previous_focus_control(current_focus_order) {
            let next_control = self.search_control_by_id(&id).unwrap();
            debug!("success search next_control id is {},set this control focus is true",next_control.id());
            next_control.set_focus();
            self.focus_order_id = next_control.focus_order();
        }
        self.rest_old_control_focus(current_focus_order);
    }

    pub fn move_focus_to_next_control(&mut self) {
        let current_focus_order = self.focus_order_id;
        if let Some(id) = self.find_next_focus_control(current_focus_order) {
            let next_control = self.search_control_by_id(&id).unwrap();
            debug!("success search next_control id is {},set this control focus is true",next_control.id());
            next_control.set_focus();
            self.focus_order_id = next_control.focus_order();
        }
        self.rest_old_control_focus(current_focus_order);
    }

    pub fn rest_old_control_focus(&mut self, old_focus_order: i32) {
        debug!("keyboard input rest focus");
        if let Some(old_focus_control) = self.search_control_by_focus_order(old_focus_order) {
            debug!("success search focus_control id is {},set this control focus is false",old_focus_control.id());
            old_focus_control.cancel_focus();
        }
    }
}
