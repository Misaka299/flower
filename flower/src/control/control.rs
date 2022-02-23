use std::any::{Any, TypeId};
use std::ops::Deref;
use std::sync::atomic::{AtomicI32, Ordering};

use log::debug;
use once_cell::sync::Lazy;
use rustc_hash::FxHashMap;

// 控件存储。窗口也视作一个控件
pub static mut CONTROL_MAP: Lazy<FxHashMap<i32, &mut dyn Control<Target=ControlState>>> = Lazy::new(|| FxHashMap::default());

// Why does setting zero make Windows invisible
static mut ID_TAG: AtomicI32 = AtomicI32::new(1);


#[derive(Clone)]
pub enum ControlType {
    Window,
    Label,
    Button,
    // TEXT_BOX,
    // COMBO_BOX,
    List,
}

#[derive(Clone)]
pub enum Position {
    // 绝对坐标，左边和顶部根据window窗口的对应位置进行计算
    Absolute,
    // 相对坐标，左边和顶部根据父级组件的对应位置进行计算
    Relative,
}

#[derive(Clone)]
pub struct ControlState {
    /// 组件id
    id: i32,
    /// 父级组件id
    parent_id: i32,
    /// 组件类名
    class: Vec<String>,
    /// 组件类型
    control_type: ControlType,
    /// 父级组件的位置
    base_left: i32,
    base_top: i32,
    rect: Rect,
    /// 是否禁用
    disable: bool,
    /// 是否可视
    visual: bool,
    /// 层级
    z_index: i32,
    /// 子级组件
    child: Vec<i32>,
}

impl ControlState {
    pub fn create(class: Vec<String>, control_type: ControlType, base_left: i32, base_top: i32) -> ControlState {
        let id = unsafe { ID_TAG.fetch_add(1, Ordering::Release) };
        debug!("control_state Register id: {}",id);
        ControlState {
            id,
            parent_id: 0,
            class,
            control_type,

            base_left,
            base_top,
            rect: Rect {
                position: Position::Relative,
                left: 0,
                top: 0,
                width: 200,
                height: 20,
            },
            disable: false,
            visual: true,
            z_index: 0,
            child: vec![],
        }
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
    pub fn base_left(&self) -> i32 {
        self.base_left
    }
    pub fn base_top(&self) -> i32 {
        self.base_top
    }
    pub fn rect(&self) -> &Rect {
        &self.rect
    }
    pub fn disable(&self) -> bool {
        self.disable
    }
    pub fn visual(&self) -> bool {
        self.visual
    }
    pub fn z_index(&self) -> i32 {
        self.z_index
    }
    pub fn child(&self) -> &Vec<i32> {
        &self.child
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
    pub fn set_base_left(&mut self, base_left: i32) {
        self.base_left = base_left;
    }
    pub fn set_base_top(&mut self, base_top: i32) {
        self.base_top = base_top;
    }
    pub fn set_rect(&mut self, rect: Rect) {
        self.rect = rect;
    }
    pub fn set_disable(&mut self, disable: bool) {
        self.disable = disable;
    }
    pub fn set_visual(&mut self, visual: bool) {
        self.visual = visual;
    }
    pub fn set_z_index(&mut self, z_index: i32) {
        self.z_index = z_index;
    }
    pub fn set_child(&mut self, child: Vec<i32>) {
        self.child = child;
    }
}

impl Deref for ControlState {
    type Target = Rect;

    fn deref(&self) -> &Self::Target {
        &self.rect
    }
}

pub trait Control: Any + Deref<Target=ControlState> {
    fn get_control_type(&self) -> ControlType;
    /// x,y 窗口发生事件时，鼠标在窗口内的相对坐标
    /// 层级数字越大，这个控件就越优先级高
    /// 层级相等，id大的控件优先级高
    /// (i32, u8, i32) z-index,层级,组件id
    // 层级数字越大，这个控件就越优先级高
    // 层级相等，id大的控件优先级高
    fn find_event_control_id(&self, x: i32, y: i32) -> (i32, u8, i32) {
        let mut self_level = (self.z_index(), 0, self.id());
        for id in self.child().iter() {
            unsafe {
                if let Some(control) = CONTROL_MAP.get_mut(&id) {
                    let child_level = control.find_event_control_id(x, y);
                    // z-index 优先级最高
                    if self_level.0 < child_level.0 {
                        self_level = child_level;
                    }
                    // 如果子控件的层级更高，那就用子控件
                    if self_level.1 < child_level.1 {
                        self_level = child_level;
                    }
                    // 如果子控件的层级一样
                    if self_level.1 == child_level.1 {
                        // 那就用id数量大的，也就是后来创建的
                        if self_level.2 < child_level.2 {
                            self_level = child_level;
                        }
                    }
                };
            }
        }
        self_level
    }

    fn on_draw(&self);
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

#[derive(Clone)]
pub struct Rect {
    /// 位置计算方式
    position: Position,
    /// 本组件的位置
    left: i32,
    top: i32,
    /// 组件宽高
    width: i32,
    height: i32,
}

impl Rect {
    pub fn position(&self) -> &Position {
        &self.position
    }
    pub fn left(&self) -> i32 {
        self.left
    }
    pub fn top(&self) -> i32 {
        self.top
    }
    pub fn width(&self) -> i32 {
        self.width
    }
    pub fn height(&self) -> i32 {
        self.height
    }
    pub fn set_position(&mut self, position: Position) {
        self.position = position;
    }
    pub fn set_left(&mut self, left: i32) {
        self.left = left;
    }
    pub fn set_top(&mut self, top: i32) {
        self.top = top;
    }
    pub fn set_width(&mut self, width: i32) {
        self.width = width;
    }
    pub fn set_height(&mut self, height: i32) {
        self.height = height;
    }
}

pub fn get<T: Any>(id: i32) -> &'static T {
    let control = unsafe { CONTROL_MAP.get_mut(&id).unwrap() };
    control.downcast_ref().unwrap()
}