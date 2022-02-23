// use std::ops::Deref;
// use nona::{Align, Canvas, Color, Context, Gradient, Point};
//
// use nonaquad::nvgimpl;
//
// use crate::control::control::{Control, control_state};
// use crate::ControlType;
//
// pub struct Label {
//     control_state: control_state,
// }
//
// impl Label {
//     pub fn create() -> Label {
//         let state = control_state::create(vec![], ControlType::LABEL, 0, 0);
//         Label{
//             control_state: state
//         }
//     }
// }
//
//
// impl Deref for Label {
//     type Target = control_state;
//
//     fn deref(&self) -> &Self::Target {
//         &self.control_state
//     }
// }
//
// impl Control for Label {
//     fn get_control_type(&self) -> ControlType {
//         ControlType::LABEL
//     }
//
//     fn on_draw(&self) {
//
//     }
// }