use flower::event::Event;
use flower::flower::Flower;
use flower::window::{Window, WindowSetting};

fn main() {
    Flower::new()
        .window("main".to_string(),
                    Window::create(WindowSetting::default())
                        .event_proc(|id, event| {
                            match event {
                                Event::WindowMove => {}
                                _ => { println!("proc move event from {} hhhh", id) }
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
