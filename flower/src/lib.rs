use std::ops::Deref;

pub use image;
use once_cell::sync::Lazy;

pub use flower_base;
use flower_base::control::{Control, ControlBase};
use flower_base::event::EventMessage::*;
use flower_base::glutin::event::{ElementState, Event, MouseButton, WindowEvent};
use flower_base::glutin::event::ElementState::Pressed;
use flower_base::glutin::event::VirtualKeyCode::Tab;
use flower_base::glutin::event_loop::{ControlFlow, EventLoop};
use flower_base::glutin::window::WindowId;
use flower_base::graphics::{Render, RENDERERS};
use flower_base::graphics::renderer::Renderer;
use flower_base::rustc_hash::FxHashMap;
use crate::window::{ButtonInfo, Window};

pub mod window;
pub mod control;
pub mod controls;

// pub type TControl = Box<dyn Control>;

static mut WINDOWS: Lazy<FxHashMap<i32, Window>> = Lazy::new(|| FxHashMap::default());
static mut WINDOWS_ID_MAP: Lazy<FxHashMap<WindowId, i32>> = Lazy::new(|| FxHashMap::default());


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
macro_rules! window {
    ($id:expr) => {
        unsafe { crate::WINDOWS.get_mut($id) }
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
                        window.resize(physical_size.width as f32, physical_size.height as f32);
                        // window.context_wrapper.resize(physical_size);
                    }
                }
                WindowEvent::CloseRequested => {
                    println!("CloseRequested {:?}", &window_id);
                    remove_window_by_window_id(&window_id);
                }
                WindowEvent::CursorEntered { .. } => {
                    if let Some(window) = get_window_by_window_id(&window_id) {
                        window.fire_message(MouseEnter);
                    }
                }
                WindowEvent::CursorLeft { .. } => {
                    if let Some(window) = get_window_by_window_id(&window_id) {
                        window.fire_message(MouseLeave);
                    }
                }
                // 状态交互有问题，指向边界会变成激活状态。而且不进入时，激活状态不会被取消掉。
                WindowEvent::CursorMoved { device_id, position, modifiers } => {

                    // let now = minstant::Instant::now();
                    if let Some(mut window) = get_window_by_window_id(&window_id) {
                        // 记录鼠标位置
                        window.mouse_location.x = position.x as i32;
                        window.mouse_location.y = position.y as i32;
                        // println!("window fire");
                        window.fire_message(MouseMove(position.x as i32, position.y as i32, modifiers));

                        // 更新控件激活
                        // 寻找响应的控件
                        if let Some(option) = window.find_event_control_id(0, position.x as f32, position.y as f32) {
                            if let Some(control) = window.search_control_by_id(&option.1) {
                                control.fire_message(MouseMove(position.x as i32, position.y as i32, modifiers));
                            }
                            // 如果新的控件和旧的控件的id值一样，那么取消这次的处理
                            if option.1 == window.active_id {
                                return;
                            }
                            // 取消激活上一个被激活的控件
                            let active_id = window.active_id;
                            if let Some(old_control) = window.search_control_by_id(&active_id) {
                                old_control.fire_message(MouseLeave);
                            } else {
                                window.fire_message(MouseLeave);
                            }
                            // 激活新的控件
                            if let Some(control) = window.search_control_by_id(&option.1) {
                                control.fire_message(MouseEnter);
                                window.active_id = control.id();
                            } else {
                                // 如果没有控件响应，那就指定到窗口
                                window.active_id = window.id;
                                window.fire_message(MouseEnter);
                            }
                        } else {
                            // fixme 如果是窗口在响应需要做什么吗?
                            return;
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
                            let mouse_location = window.mouse_location;
                            // 按键按下，标记响应控件
                            match state {
                                Pressed => {
                                    window.button_info = ButtonInfo {
                                        mouse_button: button,
                                        press_id: option.1,
                                    };
                                    if let Some(control) = window.search_control_by_id(&option.1) {
                                        //对控件发送单击事件
                                        control.fire_message(match button {
                                            MouseButton::Left => LButtonDown(mouse_location.x, mouse_location.y, modifiers),
                                            MouseButton::Right => RButtonDown(mouse_location.x, mouse_location.y, modifiers),
                                            MouseButton::Middle => MButtonDown(mouse_location.x, mouse_location.y, modifiers),
                                            MouseButton::Other(_) => OtherButtonDown(mouse_location.x, mouse_location.y, modifiers),
                                        });
                                    } else {
                                        // 没找到控件则事件在窗口上
                                        window.fire_message(match window.button_info.mouse_button {
                                            MouseButton::Left => LButtonDown(mouse_location.x, mouse_location.y, modifiers),
                                            MouseButton::Right => RButtonDown(mouse_location.x, mouse_location.y, modifiers),
                                            MouseButton::Middle => MButtonDown(mouse_location.x, mouse_location.y, modifiers),
                                            MouseButton::Other(_) => OtherButtonDown(mouse_location.x, mouse_location.y, modifiers),
                                        });
                                    };
                                }
                                ElementState::Released => {
                                    if window.button_info.press_id == option.1 {
                                        let button_info = window.button_info;

                                        if let Some(control) = window.search_control_by_id(&option.1) {
                                            //对控件发送单击事件
                                            control.fire_message(match button_info.mouse_button {
                                                MouseButton::Left => LButtonClick(mouse_location.x, mouse_location.y, modifiers),
                                                MouseButton::Right => RButtonClick(mouse_location.x, mouse_location.y, modifiers),
                                                MouseButton::Middle => MButtonClick(mouse_location.x, mouse_location.y, modifiers),
                                                MouseButton::Other(_) => OtherButtonClick(mouse_location.x, mouse_location.y, modifiers),
                                            });

                                            control.fire_message(match button_info.mouse_button {
                                                MouseButton::Left => LButtonUp(mouse_location.x, mouse_location.y, modifiers),
                                                MouseButton::Right => RButtonUp(mouse_location.x, mouse_location.y, modifiers),
                                                MouseButton::Middle => MButtonUp(mouse_location.x, mouse_location.y, modifiers),
                                                MouseButton::Other(_) => OtherButtonUp(mouse_location.x, mouse_location.y, modifiers),
                                            });
                                        } else {
                                            // 没找到控件则事件在窗口上
                                            window.fire_message(match button_info.mouse_button {
                                                MouseButton::Left => LButtonClick(mouse_location.x, mouse_location.y, modifiers),
                                                MouseButton::Right => RButtonClick(mouse_location.x, mouse_location.y, modifiers),
                                                MouseButton::Middle => MButtonClick(mouse_location.x, mouse_location.y, modifiers),
                                                MouseButton::Other(_) => OtherButtonClick(mouse_location.x, mouse_location.y, modifiers),
                                            });

                                            window.fire_message(match button_info.mouse_button {
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
                        window.request_redraw();
                    }
                }
                _ => (),
            },
            Event::RedrawRequested(window_id) => {
                if let Some(mut window) = get_window_by_window_id(&window_id) {
                    unsafe {
                        let now = minstant::Instant::now();
                        let rdr = RENDERERS.entry(window.id()).or_insert_with(|| {
                            let mut rdr = Renderer::create(window.id);
                            rdr.init(&window.context_wrapper);
                            rdr
                        });
                        rdr.new_canvas_buffer(window.id(), window.width() as i32 + 1, window.height() as i32 + 1);
                        if window.is_redraw {
                            window.on_draw(rdr);
                            window.is_redraw = false;
                        }
                        let child = window.child();
                        for x in child {
                            x.draw(rdr);
                        }
                        rdr.refresh_canvas_to_window();
                        println!("draw - {:?}", now.elapsed());
                    }
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