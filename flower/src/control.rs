use std::any::{Any, TypeId};
use std::ops::{Deref, DerefMut};
use std::sync::atomic::{AtomicI32, Ordering};

use log::debug;

use crate::render::draw::Draw;
use crate::Px;
use crate::rect::Rect;
use crate::render::render::{Renderer, Render};

// Why does setting zero make Windows invisible
static mut CONTROL_ID_TAG: AtomicI32 = AtomicI32::new(1);
static mut TOOL_ID_TAG: AtomicI32 = AtomicI32::new(-1);

#[derive(Clone, Debug)]
pub enum ControlType {
    // 常规的控件
    Control,
    // 用于MessageBox或者ToolWindow的窗口，及其控件的id注册
    Tool,
}

#[derive(Clone, Debug)]
pub enum Position {
    // 绝对坐标，左边和顶部根据window窗口的对应位置进行计算
    Absolute,
    // 相对坐标，左边和顶部根据父级组件的对应位置进行计算
    Relative,
}

#[derive(Copy, Clone, Debug)]
pub enum InteractiveState {
    Ordinary,
    Active,
    Pressed,
    Disable,
}

pub struct ControlState {
    /// 组件id
    pub(crate) id: i32,
    pub(crate) name: String,
    /// 父级组件id
    pub(crate) parent_id: i32,
    /// 组件类名
    pub(crate) class: Vec<String>,
    /// 组件类型
    pub(crate) control_type: ControlType,
    /// 父级组件的位置
    pub(crate) base_left: Px,
    pub(crate) base_top: Px,
    /// 位置计算方式
    pub(crate) position: Position,
    pub(crate) rect: Rect,
    /// 子级组件
    pub(crate) child: Vec<Box<dyn Control<Target=ControlState>>>,
    /// 是否可视
    pub(crate) visual: bool,
    // 交互状态
    pub(crate) interactive_state: InteractiveState,
    // 焦点顺序，默认使用控件id
    pub(crate) focus_order: i32,
    // 是否焦点
    pub(crate) focus: bool,
    // 是否禁止捕获焦点
    pub(crate) non_focus: bool,
}

impl ControlState {
    pub fn create(name: String, non_focus: bool, control_type: ControlType) -> ControlState {
        let id = unsafe {
            match control_type {
                ControlType::Control => { CONTROL_ID_TAG.fetch_add(1, Ordering::Release) }
                ControlType::Tool => { TOOL_ID_TAG.fetch_sub(1, Ordering::Release) }
            }
        };
        debug!("control_state Register id: {}",id);
        Self {
            id,
            name,
            parent_id: 0,
            class: vec![],
            control_type,
            base_left: 0 as Px,
            base_top: 0 as Px,
            position: Position::Relative,
            rect: Rect::new(0., 0., 50., 20.),
            visual: true,
            interactive_state: InteractiveState::Ordinary,
            focus_order: id,
            focus: false,
            non_focus,
            child: vec![],
        }
    }

    pub fn add_child(&mut self, mut child: impl Control<Target=ControlState>) {
        let base_top = self.left;
        let base_left = self.top;
        child.set_base_top(base_top);
        child.set_base_left(base_left);
        self.child.push(Box::new(child));
    }

    pub fn in_scope(&self, x: i32, y: i32) -> bool {
        // debug!("x->{} y->{}",x,y);
        return self.base_left + self.left <= x as Px &&
            self.base_left + self.left + self.width >= x as Px &&
            self.base_top + self.top <= y as Px &&
            self.base_top + self.top + self.height >= y as Px
        ;
    }

    pub fn search_control_by_id(&mut self, id: &i32) -> Option<&mut Box<dyn Control<Target=ControlState>>> {
        match self.child.binary_search_by(|c| c.id.cmp(id)) {
            Ok(this_index) => {
                if self.child.len() - 1 < this_index {
                    return None;
                }
                return Some(&mut self.child[this_index]);
            }
            _ => { None }
        }
    }

    pub fn search_control_by_focus_order(&mut self, order: i32) -> Option<&mut Box<dyn Control<Target=ControlState>>> {
        match self.child.binary_search_by(|c| c.focus_order.cmp(&order)) {
            Ok(this_index) => {
                if self.child.len() - 1 < this_index {
                    return None;
                }
                return Some(&mut self.child[this_index]);
            }
            _ => { None }
        }
    }

    /// Find controls in this control and controls under this control whose focus is true
    pub fn find_focus_control(&self) -> Option<i32> {
        if self.focus { return Some(self.id); }
        for x in &self.child {
            return x.find_focus_control();
        }
        None
    }

    pub(crate) fn find_previous_focus_control(&mut self, current_focus_order: i32) -> Option<i32> {
        let mut loop_index = current_focus_order;
        let min = self.find_min_order_focus();

        loop {
            if loop_index <= min {
                // If it's the last one, search for the first one
                loop_index = self.find_max_order_focus();
            } else {
                // find previous one
                loop_index = loop_index - 1;
            }

            // check back to the origin
            if loop_index == current_focus_order {
                return None;
            }
            // check self
            if loop_index == self.focus_order {
                return Some(self.id);
            }
            // check child
            match self.search_control_by_focus_order(loop_index) {
                // The previous one may prohibit focus, so skip it
                None => { continue; }
                Some(control) => {
                    return Some(control.id);
                }
            }
        }
    }

    fn find_max_order_focus(&self) -> i32 {
        if self.non_focus == true { return 0; }
        let mut max = self.focus_order;
        for ref mut x in &self.child {
            let i = x.find_max_order_focus();
            if i > max {
                max = i;
            }
        }
        max
    }

    fn find_min_order_focus(&self) -> i32 {
        if self.non_focus == true { return 0; }
        let mut min = self.focus_order;
        for x in &self.child {
            let i = x.find_max_order_focus();
            if i < min {
                min = i;
            }
        }
        min
    }

    pub(crate) fn find_next_focus_control(&mut self, current_focus_order: i32) -> Option<i32> {
        let mut loop_index = current_focus_order;
        let max = self.find_max_order_focus();

        loop {
            if loop_index >= max {
                // if it's the last one, search for the first one
                loop_index = self.find_min_order_focus();
            } else {
                // find previous one
                loop_index = loop_index + 1;
            }

            // check back to the origin
            if loop_index == current_focus_order {
                return None;
            }
            // check self
            if loop_index == self.focus_order {
                return Some(self.id);
            }
            // check child
            match self.search_control_by_focus_order(loop_index) {
                // The next one may prohibit focus, so skip it
                None => { continue; }
                Some(control) => {
                    return Some(control.id);
                }
            }
        }
    }

    pub fn abs_rect(&self) -> Rect {
        Rect::new(self.base_left + self.left, self.base_top + self.top, self.width, self.height)
    }

    pub fn id(&self) -> i32 {
        self.id
    }
    pub fn parent_id(&self) -> i32 {
        self.parent_id
    }
    pub fn class(&self) -> &Vec<String> {
        &self.class
    }
    pub fn control_type(&self) -> &ControlType {
        &self.control_type
    }
    pub fn base_left(&self) -> Px {
        self.base_left
    }
    pub fn base_top(&self) -> Px {
        self.base_top
    }
    pub fn rect(&self) -> &Rect {
        &self.rect
    }
    pub fn child(&self) -> &Vec<Box<dyn Control<Target=ControlState>>> {
        &self.child
    }
    pub fn disable(&self) -> bool {
        match self.interactive_state {
            InteractiveState::Disable => { true }
            _ => { false }
        }
    }
    pub fn visual(&self) -> bool {
        self.visual
    }
    pub fn active(&self) -> bool {
        match self.interactive_state {
            InteractiveState::Active => { true }
            _ => { false }
        }
    }
    pub fn focus(&self) -> bool {
        self.focus
    }
    pub fn non_focus(&self) -> bool {
        self.non_focus
    }

    pub fn set_id(&mut self, id: i32) {
        self.id = id;
    }
    pub fn set_parent_id(&mut self, parent_id: i32) {
        self.parent_id = parent_id;
    }
    pub fn set_class(&mut self, class: Vec<String>) {
        self.class = class;
    }
    pub fn set_control_type(&mut self, control_type: ControlType) {
        self.control_type = control_type;
    }
    pub fn set_base_left(&mut self, base_left: Px) {
        self.base_left = base_left;
    }
    pub fn set_base_top(&mut self, base_top: Px) {
        self.base_top = base_top;
    }
    pub fn set_rect(&mut self, rect: Rect) {
        self.rect = rect;
    }
    pub fn set_child(&mut self, child: Vec<Box<dyn Control<Target=ControlState>>>) {
        self.child = child;
    }
    pub fn set_disable(&mut self, disable: bool) {
        self.interactive_state = if disable {
            InteractiveState::Ordinary
        } else {
            InteractiveState::Disable
        }
    }
    pub fn set_visual(&mut self, visual: bool) {
        self.visual = visual;
    }
    pub fn set_active(&mut self, active: bool) {
        self.interactive_state = if active {
            InteractiveState::Active
        } else {
            InteractiveState::Ordinary
        }
    }
    pub fn set_focus(&mut self, focus: bool) {
        self.focus = focus;
    }
    pub fn set_non_focus(&mut self, non_focus: bool) {
        self.non_focus = non_focus;
    }
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
}

impl Deref for ControlState {
    type Target = Rect;

    fn deref(&self) -> &Self::Target {
        &self.rect
    }
}

impl DerefMut for ControlState {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.rect
    }
}

pub trait Control: Any + Deref<Target=ControlState> + DerefMut {
    /// 获取组件的类型
    fn get_control_type(&self) -> ControlType {
        self.control_type.clone()
    }

    /// x,y 窗口发生事件时，鼠标在窗口内的相对坐标
    /// 层级数字越大，这个控件就越优先级高
    /// 层级相等，id大的控件优先级高
    /// (i32, u8, i32) z-index,层级,组件id
    // 层级数字越大，这个控件就越优先级高
    // 层级相等，id大的控件优先级高
    fn find_event_control_id(&self, level: u8, x: i32, y: i32) -> Option<(u8, i32)> {
        // debug!("归入");
        if !self.visual {
            return None;
        }
        let mut self_level = (level, self.id);
        //先看看指针在不在当前控件范围内
        if !self.in_scope(x, y) {
            return None;
        }
        for child in &self.child {
            // 不可视的控件，其子控件也不会绘制
            if !child.visual {
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

    /// 绘制事件传播
    fn draw(&mut self, gl: &mut Renderer) {
        self.on_draw(gl);
        let child = &mut self.child;
        for x in child {
            x.draw(gl);
        }
    }

    // 组件自我绘制
    fn on_draw(&mut self, gl: &mut Renderer);
}

impl dyn Control {
    /// Returns `true` if the boxed type is the same as `T`.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::any::Any;
    ///
    /// fn is_string(s: &dyn Any) {
    ///     if s.is::<String>() {
    ///         println!("It's a string!");
    ///     } else {
    ///         println!("Not a string...");
    ///     }
    /// }
    ///
    /// is_string(&0);
    /// is_string(&"cookie monster".to_string());
    /// ```
    #[inline]
    pub fn is<T: Any>(&self) -> bool {
        // Get `TypeId` of the type this function is instantiated with.
        let t = TypeId::of::<T>();

        // Get `TypeId` of the type in the trait object (`self`).
        let concrete = self.type_id();

        // Compare both `TypeId`s on equality.
        t == concrete
    }

    /// Returns some reference to the boxed value if it is of type `T`, or
    /// `None` if it isn't.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::any::Any;
    ///
    /// fn print_if_string(s: &dyn Any) {
    ///     if let Some(string) = s.downcast_ref::<String>() {
    ///         println!("It's a string({}): '{}'", string.len(), string);
    ///     } else {
    ///         println!("Not a string...");
    ///     }
    /// }
    ///
    /// print_if_string(&0);
    /// print_if_string(&"cookie monster".to_string());
    /// ```
    #[inline]
    pub fn downcast_ref<T: Any>(&self) -> Option<&T> {
        if self.is::<T>() {
            // SAFETY: just checked whether we are pointing to the correct type, and we can rely on
            // that check for memory safety because we have implemented Any for all types; no other
            // impls can exist as they would conflict with our impl.
            unsafe { Some(&*(self as *const (dyn Control<Target=ControlState>) as *const T)) }
        } else {
            None
        }
    }

    /// Returns some mutable reference to the boxed value if it is of type `T`, or
    /// `None` if it isn't.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::any::Any;
    ///
    /// fn modify_if_u32(s: &mut dyn Any) {
    ///     if let Some(num) = s.downcast_mut::<u32>() {
    ///         *num = 42;
    ///     }
    /// }
    ///
    /// let mut x = 10u32;
    /// let mut s = "starlord".to_string();
    ///
    /// modify_if_u32(&mut x);
    /// modify_if_u32(&mut s);
    ///
    /// assert_eq!(x, 42);
    /// assert_eq!(&s, "starlord");
    /// ```
    #[inline]
    pub fn downcast_mut<T: Any>(&mut self) -> Option<&mut T> {
        if self.is::<T>() {
            // SAFETY: just checked whether we are pointing to the correct type, and we can rely on
            // that check for memory safety because we have implemented Any for all types; no other
            // impls can exist as they would conflict with our impl.
            unsafe { Some(&mut *(self as *mut (dyn Control<Target=ControlState>) as *mut T)) }
        } else {
            None
        }
    }
}