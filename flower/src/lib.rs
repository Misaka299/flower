use glutin::event::ElementState::Pressed;
use glutin::event::VirtualKeyCode::Tab;
use glutin::event::WindowEvent;
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::WindowId;
use log::debug;
use once_cell::sync::Lazy;
use rustc_hash::FxHashMap;

use crate::control::{Control, ControlState, InteractiveState};
use crate::window::Window;

pub mod window;
pub mod event;
pub(crate) mod util;
pub mod controls;
pub mod control;
pub mod draw;
pub mod rect;
pub mod color;

pub type Px = f64;

pub static mut WINDOWS: Lazy<Vec<(i32, Box<dyn Control<Target=ControlState>>)>> = Lazy::new(|| Vec::new());
pub static mut WINDOW_ID_MAP: Lazy<FxHashMap<WindowId, i32>> = Lazy::new(|| FxHashMap::default());
pub static mut WINDOW_NAME_MAP: Lazy<FxHashMap<String, i32>> = Lazy::new(|| FxHashMap::default());

pub struct Flower<T: 'static> {
    el: EventLoop<T>,
}

impl Flower<()> {
    pub fn new() -> Flower<()> {
        Self { el: EventLoop::<()>::new() }
    }
}

impl<T> Flower<T> {
    pub fn with_user_event() -> Flower<T> {
        Self { el: EventLoop::<T>::with_user_event() }
    }

    pub fn open(mut self) {
        self.el.run(move |event, event_loop, control_flow| {
            // println!("{:?}", event);
            match event {
                glutin::event::Event::LoopDestroyed => return,
                glutin::event::Event::WindowEvent { event, window_id } => match event {
                    WindowEvent::Resized(physical_size) => {
                        let window = get_window_by_window_id(&window_id);
                        window.context_wrapper.resize(physical_size);
                    }
                    WindowEvent::CloseRequested => {
                        remove_window_by_window_id(&window_id);
                    }
                    WindowEvent::CursorMoved { device_id, position, modifiers } => {
                        debug!("cursor moved");
                        let window = get_window_by_window_id(&window_id);
                        if let Some(option) = window.find_event_control_id(0, position.x as i32, position.y as i32) {
                            debug!("cursor moved - find result {:?}",option);
                            let active_id = if option.1 == window.active_id {return;}else{window.active_id};
                            // if option.1 == window.active_id {
                            //     return;
                            // }
                            // let active_id = window.active_id;
                            debug!("update active");
                            if let Some(control) = window.search_control_by_id(&option.1) {
                                debug!("success search control id is {},set this control focus is true",control.id());
                                if let InteractiveState::Ordinary = control.interactive_state {
                                    control.interactive_state = InteractiveState::Active;
                                }
                                window.active_id = control.id;
                            }
                            if let Some(old_control) = window.search_control_by_id(&active_id) {
                                debug!("success search control id is {},set this control focus is false",old_control.id());
                                if let InteractiveState::Active = old_control.interactive_state { old_control.interactive_state = InteractiveState::Ordinary; }
                            }
                            debug!("re draw");
                            window.draw();
                        }
                    }
                    WindowEvent::Focused(f) => {}
                    WindowEvent::KeyboardInput { device_id, input, is_synthetic } => {
                        if input.state != Pressed || input.virtual_keycode != Some(Tab) {
                            debug!("return");
                            return;
                        }
                        let mut window = get_window_by_window_id(&window_id);
                        if input.modifiers.shift() {
                            debug!("keyboard input to change focus to previous");
                            window.move_focus_to_previous_control();
                        } else {
                            debug!("keyboard input to change focus to next");
                            window.move_focus_to_next_control();
                        }
                        window.draw();
                    }
                    _ => (),
                },
                glutin::event::Event::RedrawRequested(window_id) => {
                    let window = get_window_by_window_id(&window_id);
                    window.draw();
                }

                _ => (),
            }

            if unsafe { WINDOWS.is_empty() } {
                *control_flow = ControlFlow::Exit
            } else {
                *control_flow = ControlFlow::Wait
            }
        });
    }
    pub fn el(&self) -> &EventLoop<T> {
        &self.el
    }
}

pub fn get_id_by_window_id(window_id: &WindowId) -> i32 {
    unsafe {
        let id = WINDOW_ID_MAP.get(&window_id).unwrap();
        (WINDOWS.binary_search_by(|(sid, _)| sid.cmp(&id)).unwrap() + 1) as i32
    }
}


pub fn get_window_by_window_id(window_id: &WindowId) -> &mut Window {
    unsafe {
        let id = WINDOW_ID_MAP.get(window_id).unwrap();
        get_window_by_id(id)
    }
}

/// 加上 & 就可以编译了
pub fn get_window_by_id(id: &i32) -> &mut Window {
    unsafe {
        let this_index = WINDOWS.binary_search_by(|(sid, _)| sid.cmp(&id)).unwrap();
        WINDOWS[this_index].1.downcast_mut::<Window>().unwrap()
    }
}

/// 加上 & 就可以编译了
pub fn get_window_control_by_id(id: &i32) -> &mut Box<dyn Control<Target=ControlState>> {
    get_window_control_by_id!(id)
}

#[macro_export]
macro_rules! get_window_control_by_id {
    ($id:expr) => {
       unsafe {
        let this_index = WINDOWS.binary_search_by(|(sid, _)| sid.cmp($id)).unwrap();
        &mut WINDOWS[this_index].1
       }
    };
}

//加上 & 就可以编译了
pub fn remove_window_by_id(id: &i32) -> String {
    unsafe {
        let win = get_window_by_id(id);
        // 删除window_id map数据
        WINDOW_ID_MAP.remove(&win.context_wrapper.window().id());
        WINDOW_NAME_MAP.remove(win.name());
        let vec_index = WINDOWS.binary_search_by(|(sid, _)| sid.cmp(&win.id())).unwrap();
        WINDOWS.remove(vec_index);
        println!("Window with ID {:?} has been closed", id);
        win.name().to_string()
    }
}

pub fn remove_window_by_window_id(id: &WindowId) -> String {
    unsafe {
        let win = get_window_by_window_id(id);
        WINDOW_NAME_MAP.remove(win.name());
        let vec_index = WINDOWS.binary_search_by(|(sid, _)| sid.cmp(&win.id())).unwrap();
        WINDOWS.remove(vec_index);
        // 删除window_id map数据
        WINDOW_ID_MAP.remove(&id);
        println!("Window with ID {:?} has been closed", id);
        win.name().to_string()
    }
}

pub fn get_window_by_name(name: &String) -> &mut Window {
    unsafe {
        let id = WINDOW_NAME_MAP.get(name).unwrap();
        get_window_by_id(id)
    }
}
