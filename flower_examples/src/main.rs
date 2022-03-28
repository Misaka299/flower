use flower::{ExtendLoopEvent, Flower};
use flower::controls::button::Button;
use flower::window::Window;

// use flower::controls::button::Button;

enum MyEvent {}

impl ExtendLoopEvent for MyEvent {}

fn main() {
    // fast_log::init_log("requests.log", log::Level::Debug, None, true);

    // let btn: Button = Button::build().parent(&win_one).create();

    // 待决定
    // win_one.addChild(btn);
    // btn.set_parent(win_one);
    let flower = Flower::new();
    let win1 = Window::create(flower.el(), "win_1".to_string(), "windows 1".to_string());
    println!("pre add");
    win1.add_child(Box::new(Button::from("btn_ok".to_string(),"hello".to_string())));
    println!("bef add end");
    Window::create(flower.el(), "windwos 2".to_string(), "windows 2".to_string());
    Window::create(flower.el(), "windwos 3".to_string(), "windows 3".to_string());
    Window::create(flower.el(), "windwos 4".to_string(), "windows 4".to_string());
    Window::create(flower.el(), "windwos 5".to_string(), "windows 5".to_string());
    Window::create(flower.el(), "windwos 6".to_string(), "windows 6".to_string());


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
