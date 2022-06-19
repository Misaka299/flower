#![windows_subsystem = "windows"]

use fast_log::config::Config;
use log::Level::Debug;
use flower::controls::button::Button;
use flower::{Flower, Px};
use flower::dpi::PhysicalSize;
use flower::window::Window;

// use flower::controls::button::Button;


fn main() {
    let log = fast_log::init(Config::new().console().level(Debug)).unwrap();

    let flower = Flower::new();

    let win1 = Window::create(flower.el(), "win_1".to_string(), "windows 1".to_string());
    // win1.window().window().set_decorations(false);
    win1.window().window().set_inner_size(PhysicalSize::new(433, 433));
    win1.window().window().set_resizable(true);
    let mut btn1 = Button::from("btn_ok".to_string(), "hello".to_string());
    btn1.top = 200.;
    btn1.height = 200.;
    btn1.left = 400.;
    btn1.width = 400.;
    win1.add_child(btn1);

    // let win2 = Window::create(flower.el(), "windwos 2".to_string(), "windows 2".to_string());
    // let mut btn2 = Button::from("btn_ok".to_string(), "hello".to_string());
    // btn2.top = 200.;
    // btn2.height = 200.;
    // btn2.left = 400.;
    // btn2.width = 400.;
    // win2.add_child(btn2);
    // Window::create(flower.el(), "windwos 3".to_string(), "windows 3".to_string());
    // Window::create(flower.el(), "windwos 4".to_string(), "windows 4".to_string());
    // Window::create(flower.el(), "windwos 5".to_string(), "windows 5".to_string());
    // Window::create(flower.el(), "windwos 6".to_string(), "windows 6".to_string());


    flower.open();
}

//
// fn event_one(id: i32, event: Event) {
//     match event {
//         Event::WindowMove => {}
//         _ => unsafe {
//             // 取窗口
//             flower::get_window("ss".to_string());
//             // 取控件
//             // flower::get_control::<Button>(1);
//             // 操作多个控件，不存在的id不会被操作
//             // flower::get_multiple_control_id::<Button>(vec![1, 2, 3], |btn| {
//             //     btn.set_text("给爷换个名字".to_string());
//             // });
//             // // 操作多个控件，寻找指定类的控件进行操作
//             // flower::get_multiple_control_class::<Button>("menu".to_string(), |id| {
//             //     // 取控件
//             //     if let Some(btn) = flower::get_control::<Button>(id) {
//             //         btn.set_text("给爷换个名字".to_string());
//             //     }
//             // });
//             // info!("proc move event from {} hhhh", id)
//         }
//     }
// }
//
// fn event_two(id: i32, event: Event) {
//     match event {
//         Event::WindowMove => {}
//         _ => { info!("proc move event from {} hhhh", id) }
//     }
// }
