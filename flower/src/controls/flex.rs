// use std::ops::{Deref, DerefMut};
// use crate::control::{Control, ControlProperty, ControlState};
//
//
// pub enum FlexDirection {
//     Row,
//     RowReverse,
//     Column,
//     ColumnReverse,
// }
//
// pub enum JustifyContent {
//     FlexStart,
//     FlexEnd,
//     Center,
//     SpaceBetween,
//     SpaceAround,
//     SpaceEvenly,
// }
//
// pub enum AlignItems {
//     FlexStart,
//     FlexEnd,
//     Center,
//     Baseline,
//     Stretch,
// }
//
// pub enum FlexWrap {
//     NoWrap,
//     Wrap,
//     WrapReverse,
// }
//
// pub enum AlignContent {
//     FlexStart,
//     FlexEnd,
//     Center,
//     SpaceBetween,
//     SpaceAround,
//     Stretch,
// }
//
// pub struct Flex {
//     state: ControlState,
//     direction: FlexDirection,
// }
//
// impl Deref for Flex {
//     type Target = ControlState;
//
//     fn deref(&self) -> &Self::Target {
//         &self.state
//     }
// }
//
// impl DerefMut for Flex {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         &mut self.state
//     }
// }
//
// impl ControlProperty for Flex {
//     fn id(&self) -> i32 {
//         self.id
//     }
//
//     fn left(&self) -> f32 {
//         todo!()
//     }
//
//     fn top(&self) -> f32 {
//         todo!()
//     }
//
//     fn width(&self) -> f32 {
//         todo!()
//     }
//
//     fn height(&self) -> f32 {
//         todo!()
//     }
// }
//
// impl Control for Flex {
//     fn on_event(&mut self, em: EventMessage) -> bool{
//         false
//         // let available_width = parent_rect.width;
//         // let available_height = parent_rect.height;
//         //
//         // let mut main_size = match self.flex_direction {
//         //     FlexDirection::Row => available_width,
//         //     FlexDirection::Column => available_height,
//         // };
//         //
//         // let mut cross_size = match self.flex_direction {
//         //     FlexDirection::Row => available_height,
//         //     FlexDirection::Column => available_width,
//         // };
//         //
//         // let total_flex_grow = self.total_flex_grow();
//         // let total_flex_shrink = self.total_flex_shrink();
//         //
//         // let mut main_pos = match self.justify_content {
//         //     JustifyContent::FlexStart => 0.0,
//         //     JustifyContent::FlexEnd => main_size - self.total_main_size(),
//         //     JustifyContent::Center => (main_size - self.total_main_size()) / 2.0,
//         //     JustifyContent::SpaceBetween => 0.0,
//         //     JustifyContent::SpaceAround => 0.0,
//         // };
//         //
//         // let mut cross_pos = match self.align_items {
//         //     AlignItems::FlexStart => 0.0,
//         //     AlignItems::FlexEnd => cross_size - self.total_cross_size(),
//         //     AlignItems::Center => (cross_size - self.total_cross_size()) / 2.0,
//         //     AlignItems::Stretch => 0.0,
//         // };
//         //
//         // for item in &mut self.items {
//         //     let flex_factor = item.flex_grow / total_flex_grow;
//         //     let flex_shrink_factor = item.flex_shrink / total_flex_shrink;
//         //
//         //     let mut item_main_size = match self.flex_direction {
//         //         FlexDirection::Row => item.rect.width,
//         //         FlexDirection::Column => item.rect.height,
//         //     };
//         //
//         //     let mut item_cross_size = match self.flex_direction {
//         //         FlexDirection::Row => item.rect.height,
//         //         FlexDirection::Column => item.rect.width,
//         //     };
//         //
//         //     if item.flex_grow > 0.0 && main_size > self.total_main_size() {
//         //         item_main_size += (main_size - self.total_main_size()) * flex_factor;
//         //     }
//         //
//         //     if item.flex_shrink > 0.0 && main_size < self.total_main_size() {
//         //         item_main_size -= (self.total_main_size() - main_size) * flex_shrink_factor;
//         //     }
//         //
//         //     match self.flex_direction {
//         //         FlexDirection::Row => {
//         //             item.rect.x = parent_rect.x + main_pos;
//         //             item.rect.y = parent_rect.y + cross_pos;
//         //             item.rect.width = item_main_size;
//         //             item.rect.height = item_cross_size;
//         //
//         //             main_pos += item_main_size;
//         //         }
//         //         FlexDirection::Column => {
//         //             item.rect.x = parent_rect.x + cross_pos;
//         //             item.rect.y = parent_rect.y + main_pos;
//         //             item.rect.width = item_cross_size;
//         //             item.rect.height = item_main_size;
//         //
//         //             main_pos += item_main_size;
//         //         }
//         //     }
//         // }
//     }
//
//     fn on_draw(&mut self, rdr: &mut GdiPlusRenderer) {
//         todo!()
//     }
// }