use std::cell::RefCell;

use once_cell::sync::Lazy;
use rustc_hash::FxHashMap;

use windows::run;

use crate::window::Window;

pub static mut WINDOWS: Lazy<FxHashMap<String, RefCell<Window>>> = Lazy::new(|| FxHashMap::default());

pub struct Flower {}

impl Flower {
    pub fn new() -> Flower {
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