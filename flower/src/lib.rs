use std::any::Any;
use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::rc::Rc;

use once_cell::sync::Lazy;
use rustc_hash::FxHashMap;

pub use windows::event;
use windows::run;

use crate::widget::controls::{Controls, CONTROLS_MAP, ControlState, ControlsType};
use crate::window::Window;

pub mod window;
pub mod widget;

static mut WINDOWS: Lazy<FxHashMap<String, RefCell<Window>>> = Lazy::new(|| FxHashMap::default());

pub struct Flower {}

impl Flower {
    pub fn new() -> Self {
        Flower {}
    }
    pub fn window(self, id: String, window: Window) -> Self {
        unsafe { WINDOWS.insert(id, RefCell::new(window)); }
        self
    }
    pub fn open(self) {
        run();
    }
}

pub fn get_window(id: String) -> Option<&'static RefCell<Window>> {
    unsafe { WINDOWS.get(&id) }
}

pub fn get_control_type(id: i32) -> Option<ControlsType> {
    unsafe {
        match CONTROLS_MAP.get_mut(&id) {
            Some(controls) => {
                Some(controls.get_controls_type())
            }
            None => { None }
        }
    }
}

pub fn get_control<T: Any>(id: i32) -> Option<&'static mut T> {
    let mut control = unsafe { CONTROLS_MAP.get_mut(&id).unwrap() } as &mut dyn Any;
    control.downcast_mut()
}

pub fn get_multiple_control_id<T: Any>(ids: Vec<i32>, handle: fn(&mut T)) {
    for id in ids.iter() {
        unsafe {
            if let Some(val) = CONTROLS_MAP.get_mut(&id) {
                let mut any = val as &mut dyn Any;
                if let Some(control) = any.downcast_mut() {
                    handle(control);
                }
            }
        }
    }
}

pub fn get_multiple_control_class<T: Any>(class: String, handle: fn(i32)) {
    for val in unsafe { CONTROLS_MAP.values_mut() } {
        if val.class().contains(&class) {
            handle(val.id());
        }
    }
}