use std::any::Any;
use std::borrow::Borrow;
use std::ops::Deref;

use glow::HasContext;
use glutin::ContextCurrentState;
use glutin::event::WindowEvent;
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::WindowId;
use rustc_hash::FxHashMap;
use takeable_option::Takeable;

use crate::control::control::{Control, ControlState, ControlType};
use crate::window::Window;

pub mod window;
pub mod control;
pub mod event;
mod util;

pub struct Flower {
    el: EventLoop<()>,
    windows: Vec<(i32, Window)>,
    window_id_map: FxHashMap<WindowId, i32>,
    window_name_map: FxHashMap<String, i32>,
}

impl Flower {
    pub fn new() -> Self {
        Flower { el: EventLoop::new(), windows: Vec::new(), window_id_map: FxHashMap::default(), window_name_map: FxHashMap::default() }
    }
    pub fn open(mut self) {
        unsafe {
            self.el.run(move |event, _, control_flow| {
                // println!("{:?}", event);
                match event {
                    glutin::event::Event::LoopDestroyed => return,
                    glutin::event::Event::WindowEvent { event, window_id } => match event {
                        WindowEvent::Resized(physical_size) => {
                            // let windowed_context = self.ct.get_current(WINDOW_MAP[&window_id].context_id.unwrap()).unwrap();
                            // let windowed_context = windowed_context.windowed();
                            // windowed_context.resize(physical_size);
                        }
                        WindowEvent::CloseRequested => {
                            let id = self.window_id_map.remove(&window_id).unwrap();
                            let this_index = self.windows.binary_search_by(|(sid, _)| sid.cmp(&id)).unwrap();
                            self.windows.remove(this_index);

                            // Takeable::take(&mut self.windows.remove(this_index).1);

                            // if let Some(window) = WINDOW_MAP.remove(&window_id) {

                            println!("Window with ID {:?} has been closed", window_id);
                            // }
                        }
                        _ => (),
                    },
                    glutin::event::Event::RedrawRequested(window_id) => {
                        let win_id = self.window_id_map.get(&window_id).unwrap();
                        let window_index = (*win_id as usize) - 1;
                        // println!("window_index : {}", window_index);
                        let x = &mut self.windows[window_index].1;
                        x.draw();
                        // println!("x.id : {:?}", x.id());
                        x.window.swap_buffers().unwrap();
                        // if let Some(id) = self.window_id_map.get(&window_id) {
                        //     let x = &mut self.windows[*id as usize].1;
                        //     println!("{}", x.id());
                        // }


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
    pub fn create_window(mut self, name: String, title: String) -> Self {
        let state = ControlState::create(vec![], ControlType::Label, 0, 0);
        let window_builder = glutin::window::WindowBuilder::new()
            .with_title(&title)
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
            let state_id = state.id();
            self.windows.push((state_id.clone(), Window::create(state, title, gl, shader_version, window)));
            self.window_id_map.insert(id, state_id);
        }
        self
    }

    pub fn get_window(&mut self, id: i32) -> &mut Window {
        let this_index = self.windows.binary_search_by(|(sid, _)| sid.cmp(&id)).unwrap();
        &mut self.windows[this_index].1
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