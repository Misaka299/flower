use std::any::Any;
use std::borrow::Borrow;
use std::ops::Deref;

use glow::{Context, HasContext};
use glutin::{ContextCurrentState, ContextWrapper, PossiblyCurrent};
use glutin::event::WindowEvent;
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::WindowId;
use once_cell::sync::Lazy;
use rustc_hash::FxHashMap;

use crate::control::control::{Control, ControlState, ControlType};

pub mod window;
pub mod control;
pub mod event;
pub mod graphics;
mod util;


static mut WINDOW_MAP: Lazy<FxHashMap<WindowId, &mut Window>> = Lazy::new(|| FxHashMap::default());
static mut NAME_MAP: Lazy<FxHashMap<String, i32>> = Lazy::new(|| FxHashMap::default());


pub struct Window {
    control_state: ControlState,
    gl: Context,
    shader_version: String,
    window: Box<ContextWrapper<PossiblyCurrent, glutin::window::Window>>,
}

//
// impl Deref for Window {
//     type Target = ControlState;
//
//     fn deref(&self) -> &Self::Target {
//         &self.control_state
//     }
// }
//
impl Window {
    // pub fn create(self) -> Self {
    //     let state = ControlState::create(vec![], ControlType::Label, 0, 0);
    //     Window {
    //         control_state: state,
    //         gl,
    //         shader_version,
    //         window,
    //     }
    // }
}

// impl Control for Window {
//     fn get_control_type(&self) -> ControlType {
//         todo!()
//     }
//
//     fn on_draw(&self) {}
// }

pub struct Flower {
    el: EventLoop<()>,
    windows: FxHashMap<WindowId, Window>,
}

impl Flower {
    pub fn new() -> Self {
        Flower { el: EventLoop::new(), windows: FxHashMap::default() }
    }
    // pub fn window(mut self, name: String, mut window: Window<T>) -> Self {
    //     unsafe {
    //         NAME_MAP.insert(name, window.id());
    //         WINDOW_MAP.insert(window.window_id.unwrap().clone(), window);
    //     }
    //     self
    // }
    pub fn open(mut self) {
        unsafe {
            self.el.run(move |event, _, control_flow| {
                println!("{:?}", event);
                match event {
                    glutin::event::Event::LoopDestroyed => return,
                    glutin::event::Event::WindowEvent { event, window_id } => match event {
                        WindowEvent::Resized(physical_size) => {
                            // let windowed_context = self.ct.get_current(WINDOW_MAP[&window_id].context_id.unwrap()).unwrap();
                            // let windowed_context = windowed_context.windowed();
                            // windowed_context.resize(physical_size);
                        }
                        WindowEvent::CloseRequested => {
                            self.windows.remove(&window_id);
                            // if let Some(window) = WINDOW_MAP.remove(&window_id) {

                            println!("Window with ID {:?} has been closed", window_id);
                            // }
                        }
                        _ => (),
                    },
                    glutin::event::Event::RedrawRequested(window_id) => {
                        // let window = &WINDOW_MAP[&window_id];
                        //
                        // let mut color = [1.0, 0.5, 0.7, 1.0];
                        // color.swap(0, 1);
                        // println!("{:?}", color);
                        //
                        // let windowed_context = self.ct.get_current(window.context_id.unwrap()).unwrap();
                        //
                        // for id in window.child().iter() {
                        //     &CONTROL_MAP[id].on_draw(window.gl.as_ref().unwrap());
                        // }
                        // window.gl.as_ref().unwrap().draw_frame(color);
                        // windowed_context.windowed().swap_buffers().unwrap();
                    }
                    _ => (),
                }

                if self.windows.is_empty() {
                    *control_flow = ControlFlow::Exit
                } else {
                    *control_flow = ControlFlow::Wait
                }
            });
        }
    }
    pub fn create_window(mut self) -> Self {
        let state = ControlState::create(vec![], ControlType::Label, 0, 0);
        let window_builder = glutin::window::WindowBuilder::new()
            .with_title("Hello triangle!")
            .with_inner_size(glutin::dpi::LogicalSize::new(1024.0, 768.0));
        unsafe {
            let window = glutin::ContextBuilder::new()
                .with_vsync(true)
                .build_windowed(window_builder, &self.el)
                .unwrap()
                .make_current()
                .unwrap();
            let gl =
                glow::Context::from_loader_function(|s| window.get_proc_address(s) as *const _);
            let shader_version = util::find_version(gl.get_parameter_string(glow::VERSION));

            //can i use this version?
            println!("{:?}", &shader_version);


            let id = window.window().id();
            let mut win = Window {
                control_state: state,
                gl,
                shader_version,
                window: Box::new(window),
            };
            self.windows.insert(id, win);
        }
        self
    }

    pub fn get_window(mut self)-> Option<&mut Window>{

        Sone(self.windows.get_mut(s))
    }
}
//
// pub fn get_window(id: String) -> Option<&'static mut Window> {
//     match unsafe { WINDOWS.get(&id) } {
//         None => { None }
//         Some(window_id) => {
//             get_control::<Window>(*window_id)
//         }
//     }
// }
//
// pub fn get_control_type(id: i32) -> Option<ControlType> {
//     match unsafe { CONTROL_MAP.get(&id) } {
//         Some(control) => {
//             Some(control.get_control_type())
//         }
//         None => { None }
//     }
// }
//
// pub fn get_control<T: Any>(id: i32) -> Option<&'static mut T> {
//     match unsafe { CONTROL_MAP.get_mut(&id) } {
//         None => { None }
//         Some(control) => {
//             control.downcast_mut()
//         }
//     }
// }
//
// pub fn get_multiple_control_id<T: Any>(ids: Vec<i32>, handle: fn(&mut T)) {
//     for id in ids.iter() {
//         if let Some(val) = unsafe { CONTROL_MAP.get_mut(&id) } {
//             if let Some(control) = val.downcast_mut::<T>() {
//                 handle(control);
//             }
//         }
//     }
// }
//
// pub fn get_multiple_control_class<T: Any>(class: String, handle: fn(i32)) {
//     for val in unsafe { CONTROL_MAP.values_mut() } {
//         if val.class().contains(&class) {
//             handle(val.id());
//         }
//     }
// }