use std::any::Any;
use std::borrow::BorrowMut;
use std::collections::HashMap;
use std::ops::{Deref, DerefMut};
use gleam::gl;

use glutin::{ContextBuilder, WindowedContext};
use glutin::event::VirtualKeyCode::L;
use glutin::event::WindowEvent;
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::{Theme, WindowBuilder, WindowId};
use once_cell::sync::Lazy;
use rustc_hash::FxHashMap;

use crate::ContextWrapper::Windowed;
use crate::control::control::{Control, CONTROL_MAP, ControlState, ControlType};
use crate::support::{ContextCurrentWrapper, ContextId, ContextTracker, ContextWrapper, Gl};

pub mod window;
pub mod control;
pub mod event;
pub mod support;
pub mod graphics;


static mut WINDOW_MAP: Lazy<FxHashMap<WindowId, Window>> = Lazy::new(|| FxHashMap::default());
static mut NAME_MAP: Lazy<FxHashMap<String, WindowId>> = Lazy::new(|| FxHashMap::default());

static mut WINDOWS: Lazy<FxHashMap<String, i32>> = Lazy::new(|| FxHashMap::default());

pub struct Window {
    control_state: ControlState,
    window_id: Option<WindowId>,
    context_id: Option<ContextId>,
    gl: Option<Gl>,
}

impl Deref for Window {
    type Target = ControlState;

    fn deref(&self) -> &Self::Target {
        &self.control_state
    }
}

impl Window {
    pub fn create() -> Self {
        let state = ControlState::create(vec![], ControlType::Label, 0, 0);
        Window {
            control_state: state,
            window_id: None,
            context_id: None,
            gl: None,
        }
    }
}

impl Control for Window {
    fn get_control_type(&self) -> ControlType {
        todo!()
    }

    fn on_draw(&self) {
        todo!()
    }
}

pub struct Flower {
    el: EventLoop<()>,
    ct: ContextTracker,
}

impl Flower {
    pub fn new() -> Self {
        Flower { el: EventLoop::new(), ct: Default::default() }
    }
    pub fn window(mut self, name: String, mut window: Window) -> Self {
        let wb = WindowBuilder::new().with_title("title");
        let windowed_context = unsafe { ContextBuilder::new().build_windowed(wb, &self.el).unwrap() };
        let windowed_context = unsafe { windowed_context.make_current().unwrap() };
        let gl = support::load(&windowed_context.context());
        let window_id = windowed_context.window().id();

        let context_id = unsafe {
            self.ct.insert(ContextCurrentWrapper::PossiblyCurrent(
                ContextWrapper::Windowed(windowed_context),
            ))
        };
        window.window_id = Some(window_id);
        window.gl = Some(gl);
        window.context_id = Some(context_id);

        unsafe {
            NAME_MAP.insert(name, window.window_id.unwrap().clone());
            WINDOW_MAP.insert(window.window_id.unwrap().clone(), window);
        }
        self
    }
    pub fn open(mut self) {
        unsafe {
            self.el.run(move |event, _, control_flow| {
                println!("{:?}", event);
                match event {
                    glutin::event::Event::LoopDestroyed => return,
                    glutin::event::Event::WindowEvent { event, window_id } => match event {
                        WindowEvent::Resized(physical_size) => {
                            let windowed_context = self.ct.get_current(WINDOW_MAP[&window_id].context_id.unwrap()).unwrap();
                            let windowed_context = windowed_context.windowed();
                            windowed_context.resize(physical_size);
                        }
                        WindowEvent::CloseRequested => {
                            if let Some(window) = WINDOW_MAP.remove(&window_id) {
                                self.ct.remove(window.context_id.unwrap());
                                println!("Window with ID {:?} has been closed", window_id);
                            }
                        }
                        _ => (),
                    },
                    glutin::event::Event::RedrawRequested(window_id) => {
                        let window = &WINDOW_MAP[&window_id];

                        let mut color = [1.0, 0.5, 0.7, 1.0];
                        color.swap(0, 1);
                        println!("{:?}", color);

                        let windowed_context = self.ct.get_current(window.context_id.unwrap()).unwrap();

                        window.gl.as_ref().unwrap().draw_frame(color);
                        windowed_context.windowed().swap_buffers().unwrap();
                    }
                    _ => (),
                }

                if WINDOW_MAP.is_empty() {
                    *control_flow = ControlFlow::Exit
                } else {
                    *control_flow = ControlFlow::Wait
                }
            });
        }
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