use std::ops::Deref;

pub use image;
pub use flower_base;

use once_cell::sync::Lazy;
use flower_base::control::{Control, ControlBase};
use flower_base::event::EventMessage::*;
use flower_base::glutin::event::ElementState::Pressed;
use flower_base::glutin::event::{ElementState, Event, MouseButton, WindowEvent};
use flower_base::glutin::event::VirtualKeyCode::Tab;
use flower_base::glutin::event_loop::{ControlFlow, EventLoop};
use flower_base::glutin::window::WindowId;
use flower_base::InteractiveState;
use flower_base::rustc_hash::FxHashMap;

use crate::window::{ButtonInfo, Window};

pub mod window;
pub mod control;
pub mod controls;

// pub type TControl = Box<dyn Control>;

pub static mut WINDOWS: Lazy<FxHashMap<i32, Window>> = Lazy::new(|| FxHashMap::default());
pub static mut WINDOWS_ID_MAP: Lazy<FxHashMap<WindowId, i32>> = Lazy::new(|| FxHashMap::default());

pub fn remove_window_by_window_id(id: &WindowId) {
    unsafe {
        println!("remove window_id {:?}", id);
        println!("remove find window_id {:?}", WINDOWS_ID_MAP.get(id));
        if let Some(id) = WINDOWS_ID_MAP.remove(id) {
            println!("remove id {:?}", id);
            WINDOWS.remove(&id);
        }
    }
}

#[macro_export]
macro_rules! window_id {
    ($id:expr) => {
        unsafe {
            if let Some(id) = WINDOWS_ID_MAP.get($id) {
                return WINDOWS.get_mut($id);
            }
            None
        }
    };
}

pub fn get_window_by_window_id(id: &WindowId) -> Option<&mut Window> {
    unsafe {
        if let Some(id) = WINDOWS_ID_MAP.get(id) {
            return WINDOWS.get_mut(id);
        }
        None
    }
}

pub fn get_window_by_id(id: &i32) -> Option<&Window> {
    unsafe {
        WINDOWS.get(id)
    }
}

pub fn run<T>(event_loop: EventLoop<T>) {
    event_loop.run(|event, event_loop, control_flow| {
        match event {
            Event::LoopDestroyed => return,
            Event::WindowEvent { event, window_id } => match event {
                WindowEvent::Resized(physical_size) => {
                    if let Some(window) = get_window_by_window_id(&window_id) {
                        // todo update gl
                        // window.px.update(physical_size.width, physical_size.height);
                        window.context_wrapper.resize(physical_size);
                    }
                }
                WindowEvent::CloseRequested => {
                    println!("CloseRequested {:?}", &window_id);
                    remove_window_by_window_id(&window_id);
                }
                WindowEvent::CursorEntered { .. } => {
                    if let Some(window) = get_window_by_window_id(&window_id) {
                        window.fire_event(MouseEnter);
                    }
                }
                WindowEvent::CursorLeft { .. } => {
                    if let Some(window) = get_window_by_window_id(&window_id) {
                        window.fire_event(MouseLeave);
                    }
                }
                // 状态交互有问题，指向边界会变成激活状态。而且不进入时，激活状态不会被取消掉。
                WindowEvent::CursorMoved { device_id, position, modifiers } => {

                    // let now = minstant::Instant::now();
                    if let Some(mut window) = get_window_by_window_id(&window_id) {
                        // 记录鼠标位置
                        window.mouse_location.x = position.x as i32;
                        window.mouse_location.y = position.y as i32;
                        println!("window fire");
                        window.fire_event(MouseMove(position.x as i32, position.y as i32, modifiers));

                        // 更新控件激活
                        // 寻找响应的控件
                        if let Some(option) = window.find_event_control_id(0, position.x as f32, position.y as f32) {
                            if let Some(control) = window.search_control_by_id(&option.1) {
                                println!("control fire");
                                control.fire_event(MouseMove(position.x as i32, position.y as i32, modifiers));
                            }
                            // 如果新的控件和旧的控件的id值一样，那么取消这次的处理
                            let active_id = if option.1 == window.active_id {
                                return;
                            } else {
                                window.active_id
                            };
                            // 取消激活上一个被激活的控件
                            if let Some(old_control) = window.search_control_by_id(&active_id) {
                                old_control.fire_event(MouseLeave);
                                if let InteractiveState::Active = old_control.interactive_state() {
                                    old_control.set_interactive_state(InteractiveState::Ordinary);
                                }
                            } else {
                                window.fire_event(MouseLeave);
                            }
                            // 激活新的控件
                            if let Some(control) = window.search_control_by_id(&option.1) {
                                control.fire_event(MouseEnter);
                                if let InteractiveState::Ordinary = control.interactive_state() {
                                    control.set_interactive_state(InteractiveState::Active);
                                    window.active_id = control.id();
                                }
                            } else {
                                // 如果没有控件响应，那就指定到窗口
                                window.active_id = window.id;
                                window.fire_event(MouseEnter);
                            }
                        }
                        window.request_redraw();
                    }
                }
                WindowEvent::Focused(f) => {}
                WindowEvent::KeyboardInput { device_id, input, is_synthetic } => {
                    if input.state != Pressed || input.virtual_keycode != Some(Tab) {
                        // debug!("return");
                        return;
                    }
                    if let Some(mut window) = get_window_by_window_id(&window_id) {
                        if input.modifiers.shift() {
                            // debug!("keyboard input to change focus to previous");
                            window.move_focus_to_previous_control();
                        } else {
                            // debug!("keyboard input to change focus to next");
                            window.move_focus_to_next_control();
                        }
                        window.request_redraw();
                    }
                }
                WindowEvent::MouseInput { device_id, state, button, modifiers } => {
                    if let Some(mut window) = get_window_by_window_id(&window_id) {
                        // 寻找响应的控件
                        if let Some(option) = window.find_event_control_id(0, window.mouse_location.x as f32, window.mouse_location.y as f32) {
                            // 按键按下，标记响应控件
                            match state {
                                Pressed => {
                                    window.button_info = ButtonInfo {
                                        mouse_button: button,
                                        press_id: option.1,
                                    };
                                    // let control = if let Some(control) = window.search_control_by_id(&option.1) {
                                    //     //对控件发送单击事件
                                    //     control
                                    // } else {
                                    //     // 没找到控件则事件在窗口上
                                    //     window
                                    // };
                                    // control.event(match window.button_info.mouse_button {
                                    //     MouseButton::Left => LButtonDown,
                                    //     MouseButton::Right => RButtonDown,
                                    //     MouseButton::Middle => MButtonDown,
                                    //     MouseButton::Other(_) => OtherButtonDown,
                                    // });
                                }
                                ElementState::Released => {
                                    if window.button_info.press_id == option.1 {
                                        let button_info = window.button_info;
                                        let mouse_location = window.mouse_location;

                                        if let Some(control) = window.search_control_by_id(&option.1) {
                                            //对控件发送单击事件
                                            control.fire_event(match button_info.mouse_button {
                                                MouseButton::Left => LButtonClick(mouse_location.x, mouse_location.y, modifiers),
                                                MouseButton::Right => RButtonClick(mouse_location.x, mouse_location.y, modifiers),
                                                MouseButton::Middle => MButtonClick(mouse_location.x, mouse_location.y, modifiers),
                                                MouseButton::Other(_) => OtherButtonClick(mouse_location.x, mouse_location.y, modifiers),
                                            });

                                            control.fire_event(match button_info.mouse_button {
                                                MouseButton::Left => LButtonUp(mouse_location.x, mouse_location.y, modifiers),
                                                MouseButton::Right => RButtonUp(mouse_location.x, mouse_location.y, modifiers),
                                                MouseButton::Middle => MButtonUp(mouse_location.x, mouse_location.y, modifiers),
                                                MouseButton::Other(_) => OtherButtonUp(mouse_location.x, mouse_location.y, modifiers),
                                            });
                                        } else {
                                            // 没找到控件则事件在窗口上
                                            window.fire_event(match button_info.mouse_button {
                                                MouseButton::Left => LButtonClick(mouse_location.x, mouse_location.y, modifiers),
                                                MouseButton::Right => RButtonClick(mouse_location.x, mouse_location.y, modifiers),
                                                MouseButton::Middle => MButtonClick(mouse_location.x, mouse_location.y, modifiers),
                                                MouseButton::Other(_) => OtherButtonClick(mouse_location.x, mouse_location.y, modifiers),
                                            });

                                            window.fire_event(match button_info.mouse_button {
                                                MouseButton::Left => LButtonUp(mouse_location.x, mouse_location.y, modifiers),
                                                MouseButton::Right => RButtonUp(mouse_location.x, mouse_location.y, modifiers),
                                                MouseButton::Middle => MButtonUp(mouse_location.x, mouse_location.y, modifiers),
                                                MouseButton::Other(_) => OtherButtonUp(mouse_location.x, mouse_location.y, modifiers),
                                            });
                                        };
                                    }
                                }
                            }
                        }
                    }
                }
                _ => (),
            },
            Event::RedrawRequested(window_id) => {
                if let Some(mut window) = get_window_by_window_id(&window_id) {
                    window.draw();
                }
            }

            _ => (),
        }
        if unsafe { WINDOWS.is_empty() } {
            *control_flow = ControlFlow::Exit
        } else {
            *control_flow = ControlFlow::Wait
        }
    });
}