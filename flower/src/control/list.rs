// use std::any::Any;
// use std::ops::Deref;
//
// use crate::control::control::{Control, ControlState};
// use crate::ControlType;
// use crate::graphics::draw::Draw;
//
// pub struct List {
//     control_state: ControlState,
//     len: i32,
//     items: Vec<Item>,
// }
//
// impl List {
//     pub fn create() -> List {
//         let state = ControlState::create(vec![], ControlType::List, 0, 0);
//         List {
//             control_state: state,
//             len: 0,
//             items: Vec::new(),
//         }
//     }
// }
//
// impl Deref for List {
//     type Target = ControlState;
//
//     fn deref(&self) -> &Self::Target {
//         &self.control_state
//     }
// }
//
// impl Control for List {
//     fn get_control_type(&self) -> ControlType {
//         ControlType::List
//     }
//
//     fn on_draw(&self) {
//         for item in self.items {
//             for el in item.content {
//                 match el {
//                     ItemElement::Text(text) => {
//
//                         let draw = Draw {};
//                         draw.font();
//                     }
//                     ItemElement::Image(img) => {
//                         let draw = Draw {};
//                         draw.font();
//                     }
//                 }
//             }
//         }
//     }
// }
//
// pub struct Item {
//     id: i32,
//     content: Vec<ItemElement>,
// }
//
// pub enum ItemElement {
//     Text(dyn Into<str>),
//     Image(dyn Into<str>)
// }
