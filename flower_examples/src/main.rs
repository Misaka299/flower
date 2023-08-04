#![windows_subsystem = "windows"]

use std::fs;
use flower::controls::button::Button;
use flower::controls::label::Label;
use flower::flower_base::background::Background::Image;
use flower::flower_base::background::ImageSize::Cover;
use flower::flower_base::control::ControlBase;
use flower::flower_base::event::EventFn;
use flower::flower_base::glutin::event_loop::EventLoop;
use flower::flower_base::graphics::rect::Rect;
use flower::run;
use flower::window::Window;

fn main() {
    let event_loop = EventLoop::<()>::new();
    let win = Window::create(&event_loop, "win_1".to_string(), "windows 1".to_string());

    let mut btn = Button::create();

    win.background = Image(fs::read("img.png").unwrap(), Cover);

    btn.add_event(EventFn::LButtonClick(|x, y, state| {
        // println!("获得屠龙宝刀 * 0x1");
    }));
    btn.add_event(EventFn::MouseEnter(|| {
        // println!("鼠标进入按钮");
    }));
    btn.add_event(EventFn::MouseLeave(|| {
        // println!("鼠标离开按钮");
    }));
    // btn.add_event(EventFn::MouseMove(|x, y, state| {
    //     println!("鼠标在按钮内移动 x:{} y:{} ,ctrl:{} alt:{} shift:{} logo:{} ", x, y, state.ctrl(), state.alt(), state.shift(), state.logo());
    // }));
    win.add_child(btn.into());

    win.add_event(EventFn::MouseEnter(|| {
        // println!("鼠标进入窗口", );
    }));
    win.add_event(EventFn::MouseLeave(|| {
        // println!("鼠标离开窗口", );
    }));
    win.add_event(EventFn::MouseMove(|x, y, state| {
        // println!("鼠标在窗口内移动 x:{} y:{} ,ctrl:{} alt:{} shift:{} logo:{} ", x, y, state.ctrl(), state.alt(), state.shift(), state.logo());
    }));

    run(event_loop);
}

// #[event(2, MouseMove)]
// pub fn s() {
//
// }