use std::borrow::BorrowMut;

use flower::event::Event;
use flower::Flower;
use flower::widget::button::Button;
use flower::widget::controls;
use flower::window::{Window, WindowSetting};

fn main() {
    Flower::new()
        .window("main".to_string(),
                Window::create(WindowSetting::default())
                    .event_proc(|id, event| {
                        match event {
                            Event::WindowMove => {}
                            _ => unsafe {
                                // 取窗口
                                flower::get_window("ss".to_string());
                                // 取控件
                                flower::get_control::<Button>(1);
                                // 操作多个控件，不存在的id不会被操作
                                flower::get_multiple_control_id::<Button>(vec![1, 2, 3], |btn| {
                                    btn.set_text("给爷换个名字".to_string());
                                });
                                // 操作多个控件，寻找指定类的控件进行操作
                                flower::get_multiple_control_class::<Button>("menu".to_string(), |id| {
                                    // 取控件
                                    if let Some(btn) = flower::get_control::<Button>(id) {
                                        btn.set_text("给爷换个名字".to_string());
                                    }
                                });
                                println!("proc move event from {} hhhh", id)
                            }
                        }
                    }),
        )
        .window("two".to_string(),
                Window::create(WindowSetting::default())
                    .event_proc(|id, event| {
                        match event {
                            Event::WindowMove => {}
                            _ => { println!("proc move event from {} hhhh", id) }
                        }
                    }),
        )
        .open();
}
