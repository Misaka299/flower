// use std::borrow::{Borrow, BorrowMut};
// use std::cell::RefCell;
// use std::ops::{Deref, DerefMut};
// use std::rc::Rc;
//
// use log::{error, info, warn};
//
// use miniquad::graphics;
// use nona::{Align, Color, Gradient, Point};
// use nonaquad::nvgimpl;
// use windows::event::Event;
// use windows::window::{NativeWindow, NativeWindowSetting};
//
// use crate::control::control::{Control, CONTROL_MAP, control_state, ControlType};
// use crate::get_control;
//
//
// #[derive(Clone)]
// pub struct Window {
//     control_state: control_state,
//     window: Rc<RefCell<NativeWindow>>,
//     /// 在 NativeWindow 里的proc是无法检索组件id的，所以这里做一层检索代理
//     event_proc: Option<fn(i32, Event)>,
// }
//
// impl Window {
//     pub fn create(window_title: String) -> Window {
//         let state = control_state::create(vec![], ControlType::WINDOW, 0, 0);
//         let window = NativeWindow::create(state.id(), NativeWindowSetting::build()
//             .window_title(window_title)
//             .scale(1.0)
//             .native_event_proc(proxy_event_proc),
//         );
//         Window {
//             control_state: state,
//             window,
//             event_proc: None,
//         }
//     }
//
//     pub fn window_title(mut self, window_title: String) -> Window {
//         let mut rc = Rc::clone(&mut self.window);
//         (*rc).borrow_mut().set_window_title(window_title);
//         self
//     }
//     pub fn scale(mut self, scale: f32) -> Window {
//         let mut rc = Rc::clone(&mut self.window);
//         (*rc).borrow_mut().set_scale(scale);
//         self
//     }
//     pub fn height(mut self, height: i32) -> Window {
//         let mut rc = Rc::clone(&mut self.window);
//         (*rc).borrow_mut().set_height(height);
//         self
//     }
//     pub fn width(mut self, width: i32) -> Window {
//         let mut rc = Rc::clone(&mut self.window);
//         (*rc).borrow_mut().set_width(width);
//         self
//     }
//
//     pub fn event_proc(mut self, event_proc: fn(i32, Event)) -> Window {
//         self.event_proc = Some(event_proc);
//         self
//     }
//
//     pub fn add_child(mut self, child: impl Control<Target=control_state>) -> Window {
//         let mut x = self.control_state.child().clone();
//         x.push(child.id());
//         self.borrow_mut().set_child(x);
//         self
//     }
// }
//
// impl Deref for Window {
//     type Target = control_state;
//     fn deref(&self) -> &control_state {
//         &self.control_state
//     }
// }
//
// impl DerefMut for Window {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         &mut self.control_state
//     }
// }
//
// impl Control for Window {
//     fn get_control_type(&self) -> ControlType {
//         ControlType::WINDOW
//     }
//
//     fn on_draw(&self) {
//
//     }
// }
//
// // pub fn proxy_event_proc(id: i32, event: Event) {
// //     if let Some(win) = get_control::<Window>(id) {
// //         match win.event_proc {
// //             Some(proc) => {
// //                 let mut event_control_id = win.id();
// //                 match event {
// //                     Event::LButtonDown(x, y) => {
// //                         win.find_event_control_id(x, y).2;
// //                     }
// //                     Event::OnPrint => {
// //                         win.on_draw();
// //                     }
// //                     _ => {}
// //                 };
// //                 info!("An event with window ID {} was received, to call event handle function.Response Control ID is : {}", id,event_control_id);
// //                 proc(event_control_id, event);
// //             }
// //             None => {
// //                 warn!("event_proc is null")
// //             }
// //         }
// //     } else {
// //         error!("An event with window ID {} was received, but the window was not found", id);
// //     }
// // }