use std::any::{Any, TypeId};
use log::debug;
use crate::event::{EventFn, EventMessage};
use crate::{ControlType, InteractiveState};
use crate::graphics::renderer::gdiplus::Renderer;

pub trait ControlBase {
    fn id(&self) -> i32;
    fn base_left(&self) -> f32;
    fn base_top(&self) -> f32;
    fn left(&self) -> f32;
    fn top(&self) -> f32;
    fn width(&self) -> f32;
    fn height(&self) -> f32;
    fn visual(&self) -> bool;
    fn interactive_state(&self) -> InteractiveState;
    fn set_interactive_state(&mut self, new_interactive_state: InteractiveState);
    fn child(&mut self) -> &mut Vec<Box<dyn Control>>;
    fn set_focus(&mut self);
    fn cancel_focus(&mut self);
    fn add_event(&mut self, efn: EventFn);
    fn add_child(&mut self, child: Box<dyn Control>);
    fn focus_order(&self) -> i32;
    fn find_focus_control(&self) -> Option<i32>;
    fn find_max_order_focus(&self) -> i32;
    fn find_min_order_focus(&self) -> i32;
    fn find_previous_focus_control(&mut self, current_focus_order: i32) -> Option<i32>;
    fn find_next_focus_control(&mut self, current_focus_order: i32) -> Option<i32>;
    fn search_control_by_focus_order(&mut self, order: i32) -> Option<&mut Box<dyn Control>>;
    /// 获取组件的类型
    fn control_type(&self) -> ControlType;
    fn fire_event(&mut self, em: EventMessage);
}

///
pub trait Control: Any + ControlBase {
    /// x,y 窗口发生事件时，鼠标在窗口内的相对坐标
    /// 层级数字越大，这个控件就越优先级高
    /// 层级相等，id大的控件优先级高
    // 层级数字越大，这个控件就越优先级高
    // 层级相等，id大的控件优先级高
    fn find_event_control_id(&mut self, level: u8, x: f32, y: f32) -> Option<(u8, i32)> {
        // debug!("归入");
        if !self.visual() {
            return None;
        }
        let mut self_level = (level, self.id());
        //先看看指针在不在当前控件范围内
        if !self.in_scope(x, y) {
            return None;
        }
        for child in self.child() {
            // 不可视的控件，其子控件也不会绘制
            if !child.visual() {
                continue;
            }
            // 在的话，可以去看看是否在子控件里
            if let Some(child_level) = child.find_event_control_id(level + 1, x, y) {
                // 如果子控件的层级更高，那就用子控件
                if self_level.0 < child_level.0 {
                    self_level = child_level;
                }
                // 如果子控件的层级一样
                if self_level.0 == child_level.0 {
                    // 那就用id数量大的，也就是后来创建的
                    if self_level.1 < child_level.1 {
                        self_level = child_level;
                    }
                }
            }
        }
        Some(self_level)
    }

    // 控件范围检测，放在这里让子控件可以重写，支持异形控件
    fn in_scope(&self, x: f32, y: f32) -> bool {
        debug!("x->{} y->{}",x,y);
        return self.base_left() + self.left() <= x &&
            self.base_left() + self.left() + self.width() >= x &&
            self.base_top() + self.top() <= y &&
            self.base_top() + self.top() + self.height() >= y
        ;
    }

    // 绘制事件传播
    fn draw(&mut self, rdr: &mut Renderer) {
        self.on_draw(rdr);
        let child = self.child();
        for x in child {
            x.draw(rdr);
        }
    }

    // 组件自我绘制
    fn on_draw(&mut self, rdr: &mut Renderer);

    // 事件消息
    /// In this function, you can custom handle event message for this control.
    /// If `false` is returned, cancel the current event; otherwise, continue propagation.
    fn on_event(&mut self, em: EventMessage) -> bool;
}

impl dyn Control {
    #[inline]
    pub fn is<T: Any>(&self) -> bool {
        // Get `TypeId` of the type this function is instantiated with.
        let t = TypeId::of::<T>();

        // Get `TypeId` of the type in the trait object (`self`).
        let concrete = self.type_id();

        // Compare both `TypeId`s on equality.
        t == concrete
    }

    #[inline]
    pub fn downcast_ref<T: Any>(&self) -> Option<&T> {
        if self.is::<T>() {
            // SAFETY: just checked whether we are pointing to the correct type, and we can rely on
            // that check for memory safety because we have implemented Any for all types; no Other
            // impls can exist as they would conflict with our impl.
            unsafe { Some(&*(self as *const (dyn Control) as *const T)) }
        } else {
            None
        }
    }

    #[inline]
    pub fn downcast_mut<T: Any>(&mut self) -> Option<&mut T> {
        if self.is::<T>() {
            // SAFETY: just checked whether we are pointing to the correct type, and we can rely on
            // that check for memory safety because we have implemented Any for all types; no Other
            // impls can exist as they would conflict with our impl.
            unsafe { Some(&mut *(self as *mut (dyn Control) as *mut T)) }
        } else {
            None
        }
    }
}