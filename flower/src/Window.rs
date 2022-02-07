use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

use windows::event::Event;
use windows::window::{NativeWindow, NativeWindowSetting};

use crate::widget::controls::{CONTROLS_MAP, Controls, ControlState, ControlsType};
use crate::WINDOWS;

#[derive(Clone, Debug)]
pub struct WindowSetting {
    window_title: String,
    width: i32,
    height: i32,
    scale: f32,
}

impl Default for WindowSetting {
    fn default() -> Self {
        WindowSetting {
            window_title: "flower ui".to_string(),
            width: 800,
            height: 400,
            scale: 1.0,
        }
    }
}

#[derive(Clone)]
pub struct Window {
    control_state: ControlState,
    window: Rc<RefCell<NativeWindow>>,
    /// 在 NativeWindow 里的proc是无法检索组件id的，所以这里做一层检索代理
    event_proc: Option<Rc<Box<dyn Fn(i32, Event)>>>,
}

impl Window {
    pub fn create(setting: WindowSetting) -> Self {
        let window = Window {
            control_state: ControlState::create(vec![], ControlsType::WINDOW,0, 0),
            window: NativeWindow::create(NativeWindowSetting::build()
                .window_title(setting.window_title)
                .height(setting.height)
                .width(setting.width)
                .scale(setting.scale)
            ),
            event_proc: None,
        };
        unsafe {
            CONTROLS_MAP.insert((CONTROLS_MAP.len() + 1).try_into().unwrap(), Box::leak(Box::new(window.clone())));
        }
        window
    }
    pub fn event_proc(mut self, event_proc: impl Fn(i32, Event) + 'static + std::ops::Fn(i32, windows::event::Event) -> ()) -> Self {
        self.event_proc = Some(Rc::new(Box::new(event_proc)));
        let mut rc = Rc::clone(&self.window);
        // let mut rfc = rc.borrow_mut();
        (*rc).borrow_mut().event_proc(Box::new(proxy_event_proc));
        self
    }
}

impl Deref for Window {
    type Target = ControlState;
    fn deref(&self) -> &ControlState {
        &self.control_state
    }
}

impl Controls for Window {
    fn get_controls_type(&self) -> ControlsType {
        ControlsType::WINDOW
    }
}

pub fn proxy_event_proc(event: Event) {
    unsafe {
        // for s in STATE_MAP.iter() {
        //     println!("{}", s.0);
        // }

        let x = match CONTROLS_MAP.get(&1) {
            None => {
                println!("state not found");
                return;
            }
            Some(state) => { state }
        };
        let x1 = match WINDOWS.get("main") {
            None => {
                println!("main not found");
                return;
            }
            Some(win) => { win }
        };
        let proc = match x1.borrow_mut().event_proc.as_ref() {
            None => {
                println!("event_proc is null");
                return;
            }
            Some(event) => { Rc::clone(event) }
        };
        proc(x.id(), event);
    }
    println!("widget event");
}