use std::sync::atomic::{AtomicI32, Ordering};

pub use glutin;
pub use rustc_hash;


pub mod event;
pub mod graphics;
pub mod control;
pub mod background;

static mut CONTROL_ID_TAG: AtomicI32 = AtomicI32::new(1);

pub fn next_id() -> i32 {
    unsafe {
        let id = CONTROL_ID_TAG.fetch_add(1, Ordering::Release);
        println!("分配新ID {}", id);
        id
    }
}

#[derive(Copy, Clone, Debug)]
pub enum InteractiveState {
    // 普通
    Ordinary = 1,
    // 激活
    Active = 2,
    // 按下
    Pressed = 3,
    // 禁用
    Disable = 4,
}

#[derive(Clone, Debug)]
pub enum ControlType {
    // 常规的控件
    Control,
    // 用于MessageBox或者ToolWindow的窗口，及其控件的id注册
    Tool,
}
