// use std::ops::Deref;
//
// use crate::control::control::{Control, control_state, ControlType};
//
// pub struct Button {
//     control_state: control_state,
//     title: String,
//     on_click: Option<Box<dyn Fn()>>,
// }
//
// impl Button {
//     pub fn from(title: String) -> Button {
//         Button {
//             control_state: control_state::create(vec![], ControlType::BUTTON, 0, 0),
//             title,
//             on_click: None,
//         }
//     }
//     pub fn on_click(&mut self, fn_on_click: Box<dyn Fn()>) -> &mut Self {
//         self.on_click = Some(fn_on_click);
//         self
//     }
//     pub fn set_text(&mut self, title: String) -> &mut Self {
//         self.title = title;
//         self
//     }
//     pub fn get_text(&self) -> String {
//         self.title.clone()
//     }
// }
//
// impl Deref for Button {
//     type Target = control_state;
//
//     fn deref(&self) -> &Self::Target {
//         &self.control_state
//     }
// }
//
// impl Control for Button {
//     fn get_control_type(&self) -> ControlType {
//         ControlType::BUTTON
//     }
//
//     fn on_draw(&self) {
//         todo!()
//     }
// }