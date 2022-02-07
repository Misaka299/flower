use std::any::Any;
use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

use once_cell::sync::Lazy;
use rustc_hash::FxHashMap;

use crate::widget::button::Button;

// 基本状态存储
pub static mut CONTROLS_MAP: Lazy<FxHashMap<i32, &mut dyn Controls<Target=ControlState>>> = Lazy::new(|| FxHashMap::default());

#[derive(Clone)]
pub enum ControlsType {
    WINDOW,
    BUTTON,
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
    controls_type: ControlsType,
    /// 位置计算方式
    position: Position,
    /// 父级组件的位置
    base_left: i32,
    base_top: i32,
    /// 本组件的位置
    left: i32,
    top: i32,
    /// 组件宽高
    width: i32,
    height: i32,
    /// 是否禁用
    disable: bool,
    /// 是否可视
    visual: bool,
    /// 层级
    z_index: i32,
    /// 子级组件
    child: Vec<Rc<RefCell<dyn Controls<Target=ControlState>>>>,
}

impl ControlState {
    pub fn create(class: Vec<String>, controls_type: ControlsType, base_left: i32, base_top: i32) -> ControlState {
        unsafe {
            ControlState {
                id: (CONTROLS_MAP.len() + 1) as i32,
                parent_id: 0,
                class,
                controls_type,
                position: Position::Relative,
                base_left,
                base_top,
                left: 0,
                top: 0,
                width: 200,
                height: 20,
                disable: false,
                visual: true,
                z_index: 0,
                child: vec![],
            }
        }
    }
    pub fn set_left_top(mut self, left: i32, top: i32) -> Self {
        self.left = left;
        self.top = top;
        self
    }
    pub fn set_width_height(mut self, width: i32, height: i32) -> Self {
        self.width = width;
        self.height = height;
        self
    }
    pub fn set_rect(mut self, left: i32, top: i32, width: i32, height: i32) -> Self {
        self.left = left;
        self.top = top;
        self.width = width;
        self.height = height;
        self
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
    pub fn position(&self) -> &Position {
        &self.position
    }
    pub fn base_left(&self) -> i32 {
        self.base_left
    }
    pub fn base_top(&self) -> i32 {
        self.base_top
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
    pub fn disable(&self) -> bool {
        self.disable
    }
    pub fn visual(&self) -> bool {
        self.visual
    }
    pub fn z_index(&self) -> i32 {
        self.z_index
    }
    pub fn child(&self) -> &Vec<Rc<RefCell<dyn Controls<Target=ControlState>>>> {
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
    pub fn set_position(&mut self, position: Position) {
        self.position = position;
    }
    pub fn set_base_left(&mut self, base_left: i32) {
        self.base_left = base_left;
    }
    pub fn set_base_top(&mut self, base_top: i32) {
        self.base_top = base_top;
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
    pub fn set_disable(&mut self, disable: bool) {
        self.disable = disable;
    }
    pub fn set_visual(&mut self, visual: bool) {
        self.visual = visual;
    }
    pub fn set_z_index(&mut self, z_index: i32) {
        self.z_index = z_index;
    }
    pub fn set_child(&mut self, child: Vec<Rc<RefCell<dyn Controls<Target=ControlState>>>>) {
        self.child = child;
    }
    //other
    pub fn set_parent(&mut self, parent: Box<dyn Controls<Target=ControlState>>) {
        self.parent_id = parent.id;
    }
}

pub trait Controls: Deref<Target=ControlState> {
    fn get_controls_type(&self) -> ControlsType;
}

pub fn get<T: Any>(id: i32) -> &'static T {
    let control = unsafe { CONTROLS_MAP.get(&id).unwrap() } as &dyn Any;
    control.downcast_ref::<T>().unwrap()
}