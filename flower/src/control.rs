// use std::any::{Any, TypeId};
// use std::sync::atomic::{AtomicI32, Ordering};
//
// use log::debug;
// use flower_base::{ControlType, InteractiveState};
// use flower_base::event::{EventFn, EventMessage, EventType};
// use flower_base::graphics::rect::Rect;
// use flower_base::graphics::renderer::default::Renderer;
// use flower_base::rustc_hash::FxHashMap;
//
//
// // Why does setting zero make Windows invisible
// static mut CONTROL_ID_TAG: AtomicI32 = AtomicI32::new(1);
// static mut TOOL_ID_TAG: AtomicI32 = AtomicI32::new(-1);
//
//
// pub struct ControlState {
//     /// 组件id
//     pub(crate) id: i32,
//     pub(crate) name: String,
//     /// 父级组件id
//     pub(crate) parent_id: i32,
//     /// 组件类名
//     pub(crate) class: Vec<String>,
//     /// 组件类型
//     pub(crate) control_type: ControlType,
//     /// 父级组件的位置
//     pub(crate) base_left: f32,
//     pub(crate) base_top: f32,
//     /// 位置计算方式
//     // pub(crate) position: Position,
//     pub(crate) rect: Rect,
//     /// 子级组件
//     pub(crate) child: Vec<Box<dyn Control>>,
//     /// 是否可视
//     pub(crate) visual: bool,
//     // 交互状态
//     pub(crate) interactive_state: InteractiveState,
//     // 焦点顺序，默认使用控件id
//     pub(crate) focus_order: i32,
//     // 是否焦点
//     pub(crate) focus: bool,
//     // 是否禁止捕获焦点
//     pub(crate) non_focus: bool,
//     pub(crate) events: FxHashMap<EventType, Vec<EventFn>>,
// }
//
// impl ControlState {
//     // todo 初始化需要重新处理
//     pub fn create(name: String, rect: Rect, non_focus: bool, control_type: ControlType) -> ControlState {
//         let id = unsafe {
//             match control_type {
//                 ControlType::Control => { CONTROL_ID_TAG.fetch_add(1, Ordering::Release) }
//                 ControlType::Tool => { TOOL_ID_TAG.fetch_sub(1, Ordering::Release) }
//             }
//         };
//         debug!("control_state Register id: {}",id);
//         let mut events = FxHashMap::default();
//         Self {
//             id,
//             name,
//             parent_id: 0,
//             class: vec![],
//             control_type,
//             base_left: 0.,
//             base_top: 0.,
//             // position: Position::Relative,
//             // rect: Rect::new(0, 0, 50, 20),
//             visual: true,
//             interactive_state: InteractiveState::Ordinary,
//             focus_order: id,
//             focus: false,
//             non_focus,
//             child: vec![],
//             rect,
//             events,
//         }
//     }
//     // FIXME ok
//     pub fn set_focus(&mut self) {
//         self.non_focus = true;
//         self.fire_event(EventMessage::FocusGet);
//     }
//     // FIXME ok
//     pub fn cancel_focus(&mut self) {
//         self.non_focus = false;
//         self.fire_event(EventMessage::FocusLost);
//     }
//
//     pub fn add_event(&mut self, efn: EventFn) {
//         self.events.entry(efn.into()).or_insert(vec![]).push(efn);
//     }
//
//     pub fn add_child(&mut self, child: impl Control) {
//         self.child.push(Box::new(child));
//     }
//
//     pub fn search_control_by_id(&mut self, id: &i32) -> Option<&mut Box<dyn Control>> {
//         match self.child.binary_search_by(|c| c.id().cmp(id)) {
//             Ok(this_index) => {
//                 if self.child.len() - 1 < this_index {
//                     return None;
//                 }
//                 return Some(&mut self.child[this_index]);
//             }
//             _ => { None }
//         }
//     }
//
//     pub fn search_control_by_focus_order(&mut self, order: i32) -> Option<&mut Box<dyn Control>> {
//         match self.child.binary_search_by(|c| c.focus_order().cmp(&order)) {
//             Ok(this_index) => {
//                 if self.child.len() - 1 < this_index {
//                     return None;
//                 }
//                 return Some(&mut self.child[this_index]);
//             }
//             _ => { None }
//         }
//     }
//
//     /// Find controls in this control and controls under this control whose focus is true
//     pub fn find_focus_control(&self) -> Option<i32> {
//         if self.focus { return Some(self.id); }
//         for x in &self.child {
//             return x.find_focus_control();
//         }
//         None
//     }
//
//     pub(crate) fn find_previous_focus_control(&mut self, current_focus_order: i32) -> Option<i32> {
//         let mut loop_index = current_focus_order;
//         let min = self.find_min_order_focus();
//
//         loop {
//             if loop_index <= min {
//                 // If it's the last one, search for the first one
//                 loop_index = self.find_max_order_focus();
//             } else {
//                 // find previous one
//                 loop_index = loop_index - 1;
//             }
//
//             // check back to the origin
//             if loop_index == current_focus_order {
//                 return None;
//             }
//             // check self
//             if loop_index == self.focus_order {
//                 return Some(self.id);
//             }
//             // check child
//             match self.search_control_by_focus_order(loop_index) {
//                 // The previous one may prohibit focus, so skip it
//                 None => { continue; }
//                 Some(control) => {
//                     return Some(control.id());
//                 }
//             }
//         }
//     }
//
//     fn find_max_order_focus(&self) -> i32 {
//         if self.non_focus == true { return 0; }
//         let mut max = self.focus_order;
//         for ref mut x in &self.child {
//             let i = x.find_max_order_focus();
//             if i > max {
//                 max = i;
//             }
//         }
//         max
//     }
//
//     fn find_min_order_focus(&self) -> i32 {
//         if self.non_focus == true { return 0; }
//         let mut min = self.focus_order;
//         for x in &self.child {
//             let i = x.find_max_order_focus();
//             if i < min {
//                 min = i;
//             }
//         }
//         min
//     }
//
//     pub(crate) fn find_next_focus_control(&mut self, current_focus_order: i32) -> Option<i32> {
//         let mut loop_index = current_focus_order;
//         let max = self.find_max_order_focus();
//
//         loop {
//             if loop_index >= max {
//                 // if it's the last one, search for the first one
//                 loop_index = self.find_min_order_focus();
//             } else {
//                 // find previous one
//                 loop_index = loop_index + 1;
//             }
//
//             // check back to the origin
//             if loop_index == current_focus_order {
//                 return None;
//             }
//             // check self
//             if loop_index == self.focus_order {
//                 return Some(self.id);
//             }
//             // check child
//             match self.search_control_by_focus_order(loop_index) {
//                 // The next one may prohibit focus, so skip it
//                 None => { continue; }
//                 Some(control) => {
//                     return Some(control.id());
//                 }
//             }
//         }
//     }
//
//     pub fn fire_event(&mut self, em: EventMessage) {
//         if let Some(vec) = self.events.get(&em.into()) {
//             for f in vec {
//                 match f {
//                     EventFn::LButtonDown(f) => if let EventMessage::LButtonDown(x, y, state) = em { f(x, y, state) },
//                     EventFn::LButtonClick(f) => if let EventMessage::LButtonClick(x, y, state) = em { f(x, y, state) },
//                     EventFn::LButtonUp(f) => if let EventMessage::LButtonUp(x, y, state) = em { f(x, y, state) },
//                     EventFn::RButtonDown(f) => if let EventMessage::RButtonDown(x, y, state) = em { f(x, y, state) },
//                     EventFn::RButtonClick(f) => if let EventMessage::RButtonClick(x, y, state) = em { f(x, y, state) },
//                     EventFn::RButtonUp(f) => if let EventMessage::RButtonUp(x, y, state) = em { f(x, y, state) },
//                     EventFn::MButtonDown(f) => if let EventMessage::MButtonDown(x, y, state) = em { f(x, y, state) },
//                     EventFn::MButtonClick(f) => if let EventMessage::MButtonClick(x, y, state) = em { f(x, y, state) },
//                     EventFn::MButtonUp(f) => if let EventMessage::MButtonUp(x, y, state) = em { f(x, y, state) },
//                     EventFn::OtherButtonDown(f) => if let EventMessage::OtherButtonDown(x, y, state) = em { f(x, y, state) },
//                     EventFn::OtherButtonClick(f) => if let EventMessage::OtherButtonClick(x, y, state) = em { f(x, y, state) },
//                     EventFn::OtherButtonUp(f) => if let EventMessage::OtherButtonUp(x, y, state) = em { f(x, y, state) },
//                     EventFn::MouseEnter(f) => f(),
//                     EventFn::MouseLeave(f) => f(),
//                     EventFn::MouseMove(f) => if let EventMessage::MouseMove(x, y, state) = em { f(x, y, state) },
//                     EventFn::FocusGet(f) => f(),
//                     EventFn::FocusLost(f) => f(),
//                     EventFn::ReSize(_) => {}
//                 }
//             }
//         }
//     }
// }
//
// pub trait ControlBase {
//     fn id(&self) -> i32;
//     fn base_left(&self) -> f32;
//     fn base_top(&self) -> f32;
//     fn left(&self) -> f32;
//     fn top(&self) -> f32;
//     fn width(&self) -> f32;
//     fn height(&self) -> f32;
//     fn visual(&self) -> bool;
//     fn interactive_state(&self) -> InteractiveState;
//     fn set_interactive_state(&mut self, new_interactive_state: InteractiveState);
//     fn child(&mut self) -> &mut Vec<Box<dyn Control>>;
//     fn set_focus(&mut self);
//     fn cancel_focus(&mut self);
//     fn add_event(&mut self, efn: EventFn);
//     fn add_child(&mut self, child: Box<dyn Control>);
//     fn focus_order(&self) -> i32;
//     fn find_focus_control(&self) -> Option<i32>;
//     fn find_max_order_focus(&self) -> i32;
//     fn find_min_order_focus(&self) -> i32;
//     fn find_previous_focus_control(&mut self, current_focus_order: i32) -> Option<i32>;
//     fn find_next_focus_control(&mut self, current_focus_order: i32) -> Option<i32>;
//     fn search_control_by_focus_order(&mut self, order: i32) -> Option<&mut Box<dyn Control>>;
//     /// 获取组件的类型
//     fn control_type(&self) -> ControlType;
//     fn fire_event(&mut self, em: EventMessage);
// }
//
// ///
// pub trait Control: Any + ControlBase {
//     /// x,y 窗口发生事件时，鼠标在窗口内的相对坐标
//     /// 层级数字越大，这个控件就越优先级高
//     /// 层级相等，id大的控件优先级高
//     // 层级数字越大，这个控件就越优先级高
//     // 层级相等，id大的控件优先级高
//     fn find_event_control_id(&mut self, level: u8, x: f32, y: f32) -> Option<(u8, i32)> {
//         // debug!("归入");
//         if !self.visual() {
//             return None;
//         }
//         let mut self_level = (level, self.id());
//         //先看看指针在不在当前控件范围内
//         if !self.in_scope(x, y) {
//             return None;
//         }
//         for child in self.child() {
//             // 不可视的控件，其子控件也不会绘制
//             if !child.visual() {
//                 continue;
//             }
//             // 在的话，可以去看看是否在子控件里
//             if let Some(child_level) = child.find_event_control_id(level + 1, x, y) {
//                 // 如果子控件的层级更高，那就用子控件
//                 if self_level.0 < child_level.0 {
//                     self_level = child_level;
//                 }
//                 // 如果子控件的层级一样
//                 if self_level.0 == child_level.0 {
//                     // 那就用id数量大的，也就是后来创建的
//                     if self_level.1 < child_level.1 {
//                         self_level = child_level;
//                     }
//                 }
//             }
//         }
//         Some(self_level)
//     }
//
//     // 控件范围检测，放在这里让子控件可以重写，支持异形控件
//     fn in_scope(&self, x: f32, y: f32) -> bool {
//         debug!("x->{} y->{}",x,y);
//         return self.base_left() + self.left() <= x &&
//             self.base_left() + self.left() + self.width() >= x &&
//             self.base_top() + self.top() <= y &&
//             self.base_top() + self.top() + self.height() >= y
//         ;
//     }
//
//     // 绘制事件传播
//     fn draw(&mut self, rdr: &mut Renderer) {
//         self.on_draw(rdr);
//         let child = self.child();
//         for x in child {
//             x.draw(rdr);
//         }
//     }
//
//     // 组件自我绘制
//     fn on_draw(&mut self, rdr: &mut Renderer);
//
//     // 事件消息
//     /// In this function, you can custom handle event message for this control.
//     /// If `false` is returned, cancel the current event; otherwise, continue propagation.
//     fn on_event(&mut self, em: EventMessage) -> bool;
// }
//
// impl dyn Control {
//     #[inline]
//     pub fn is<T: Any>(&self) -> bool {
//         // Get `TypeId` of the type this function is instantiated with.
//         let t = TypeId::of::<T>();
//
//         // Get `TypeId` of the type in the trait object (`self`).
//         let concrete = self.type_id();
//
//         // Compare both `TypeId`s on equality.
//         t == concrete
//     }
//
//     #[inline]
//     pub fn downcast_ref<T: Any>(&self) -> Option<&T> {
//         if self.is::<T>() {
//             // SAFETY: just checked whether we are pointing to the correct type, and we can rely on
//             // that check for memory safety because we have implemented Any for all types; no Other
//             // impls can exist as they would conflict with our impl.
//             unsafe { Some(&*(self as *const (dyn Control) as *const T)) }
//         } else {
//             None
//         }
//     }
//
//     #[inline]
//     pub fn downcast_mut<T: Any>(&mut self) -> Option<&mut T> {
//         if self.is::<T>() {
//             // SAFETY: just checked whether we are pointing to the correct type, and we can rely on
//             // that check for memory safety because we have implemented Any for all types; no Other
//             // impls can exist as they would conflict with our impl.
//             unsafe { Some(&mut *(self as *mut (dyn Control) as *mut T)) }
//         } else {
//             None
//         }
//     }
// }