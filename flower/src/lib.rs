use std::any::Any;

use once_cell::sync::Lazy;
use rustc_hash::FxHashMap;

pub use windows::event;
use windows::run;

use crate::widget::control::{control, control_MAP, controltate, controlType};
use crate::window::Window;

pub mod window;
pub mod widget;

static mut WINDOWS: Lazy<FxHashMap<String, i32>> = Lazy::new(|| FxHashMap::default());

pub struct Flower {}

impl Flower {
    pub fn new() -> Self {
        Flower {}
    }
    pub fn window(self, name: String, window: Window) -> Self {
        unsafe { WINDOWS.insert(name, window.id()); }
        unsafe {
            let le = Box::leak(Box::new(window));
            let i = (control_MAP.len() + 1) as i32;
            control_MAP.insert(i, le);
        }
        self
    }
    pub fn open(self) {
        run();
    }
}

pub fn get_window(id: String) -> Option<&'static mut Window> {
    match unsafe { WINDOWS.get(&id) } {
        None => { None }
        Some(window_id) => {
            get_control::<Window>(*window_id)
        }
    }
}

pub fn get_control_type(id: i32) -> Option<controlType> {
    match unsafe { control_MAP.get(&id) } {
        Some(control) => {
            Some(control.get_control_type())
        }
        None => { None }
    }
}

pub fn get_control<T: Any>(id: i32) -> Option<&'static mut T> {
    match unsafe { control_MAP.get_mut(&id) } {
        None => { None }
        Some(control) => {
            control.downcast_mut()
        }
    }
}

pub fn get_multiple_control_id<T: Any>(ids: Vec<i32>, handle: fn(&mut T)) {
    for id in ids.iter() {
        if let Some(val) = unsafe { control_MAP.get_mut(&id) } {
            if let Some(control) = val.downcast_mut::<T>() {
                handle(control);
            }
        }
    }
}

pub fn get_multiple_control_class<T: Any>(class: String, handle: fn(i32)) {
    for val in unsafe { control_MAP.values_mut() } {
        if val.class().contains(&class) {
            handle(val.id());
        }
    }
}