use std::collections::HashMap;
use std::ops::Deref;
use windows::app;
use windows::window::{NativeWindow, NativeWindowSetting};

struct WidgetState {
    id: i32,
    class: Vec<String>,
}

struct Button {
    widget_state: WidgetState,
    text: String,
}

impl Deref for Button {
    type Target = WidgetState;

    fn deref(&self) -> &WidgetState {
        &self.widget_state
    }
}

struct AA {
    s: i32,
}

fn main() {
    unsafe {
        let mut s:HashMap<i32, Box<dyn Deref<Target=WidgetState>>> = HashMap::new();
        //App::new()
        //    .add_window("2b".to_string(), Window::createWindow(WindowSetting::default()))
        //    .add_window("3b".to_string(), Window::createWindow(WindowSetting::default()))
        //    .run();
        // App::get_window("ss".to_string()).css();
        s.insert(1,Box::new(Button{ widget_state: WidgetState { id: 0, class: vec![] }, text: "".to_string() }));
        // s.insert(1,Box::new(AA{ s: 0 }));


    }
}