use glutin::event::WindowEvent;
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::WindowId;
use once_cell::sync::Lazy;
use rustc_hash::FxHashMap;

use crate::control::control::{Control, ControlState, ControlType};
use crate::window::Window;

pub mod window;
pub mod control;
pub mod event;
pub(crate) mod util;

pub static mut WINDOWS: Lazy<Vec<(i32, Window)>> = Lazy::new(|| Vec::new());
pub static mut WINDOW_ID_MAP: Lazy<FxHashMap<WindowId, i32>> = Lazy::new(|| FxHashMap::default());
pub static mut WINDOW_NAME_MAP: Lazy<FxHashMap<String, i32>> = Lazy::new(|| FxHashMap::default());

pub trait ExtendLoopEvent {}

impl ExtendLoopEvent for () {}

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

    pub fn open(self) {
        unsafe {
            self.el.run(move |event, event_loop, control_flow| {
                // println!("{:?}", event);
                match event {
                    glutin::event::Event::LoopDestroyed => return,
                    glutin::event::Event::WindowEvent { event, window_id } => match event {
                        WindowEvent::Resized(physical_size) => {
                            let x = get_window_by_window_id(&window_id);
                            x.window.resize(physical_size);
                        }
                        WindowEvent::CloseRequested => {
                            remove_window_by_window_id(&window_id);
                        }
                        _ => (),
                    },
                    glutin::event::Event::RedrawRequested(window_id) => {
                        let x = get_window_by_window_id(&window_id);
                        x.draw();
                        x.window.swap_buffers().unwrap();
                    }
                    _ => (),
                }

                if WINDOWS.is_empty() {
                    *control_flow = ControlFlow::Exit
                } else {
                    *control_flow = ControlFlow::Wait
                }
            });
        }
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
        let id = WINDOW_ID_MAP.get(&window_id).unwrap();
        get_window_by_id(id)
    }
}

/// 加上 & 就可以编译了
pub fn get_window_by_id(id: &i32) -> &mut Window {
    unsafe {
        let this_index = WINDOWS.binary_search_by(|(sid, _)| sid.cmp(&id)).unwrap();
        &mut WINDOWS[this_index].1
    }
}

//加上 & 就可以编译了
pub fn remove_window_by_id(id: &i32) -> String {
    unsafe {
        let win = get_window_by_id(id);
        // 删除window_id map数据
        WINDOW_ID_MAP.remove(&win.window.window().id());
        WINDOW_NAME_MAP.remove(win.name());
        let vec_index = WINDOWS.binary_search_by(|(sid, _)| sid.cmp(&win.id())).unwrap() + 1;
        WINDOWS.remove(vec_index);
        println!("Window with ID {:?} has been closed", id);
        win.name().to_string()
    }
}

pub fn remove_window_by_window_id(id: &WindowId) -> String {
    unsafe {
        // 删除window_id map数据
        WINDOW_ID_MAP.remove(&id);
        let win = get_window_by_window_id(id);
        WINDOW_NAME_MAP.remove(win.name());
        let vec_index = WINDOWS.binary_search_by(|(sid, _)| sid.cmp(&win.id())).unwrap() + 1;
        WINDOWS.remove(vec_index);
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