use std::ops::{Deref, DerefMut};

use crate::control::{Control, ControlState, ControlType, InteractiveState};
use crate::event::EventMessage;
use crate::graphics::color::Color;
use crate::graphics::font::Font;
use crate::graphics::pen::Pen;
use crate::graphics::rect::Rect;
use crate::graphics::Render;
use crate::graphics::renderer::default::Renderer;

pub struct Button {
    state: ControlState,
}

impl Button {
    pub fn create() -> Button {
        Button {
            state: ControlState::create("按钮".to_string(), Rect {
                left: 100.0,
                top: 100.0,
                width: 200.0,
                height: 200.0,
            }, false, ControlType::Control)
        }
    }
}

impl Control for Button {
    fn on_draw(&mut self, rdr: &mut Renderer) {
        println!("draw button");

        let color = match self.interactive_state {
            InteractiveState::Ordinary => {
                Color {
                    r: 255,
                    g: 40,
                    b: 04,
                    a: 255,
                }
            }
            _ => {
                Color {
                    r: 0,
                    g: 40,
                    b: 04,
                    a: 255,
                }
            }
        };

        let pen = &Pen {
            width: 1.,
            color,
        };

        let mut border_rect = self.rect;
        border_rect.left = 0.;
        border_rect.top = 0.;

        rdr.store(&border_rect, pen);

        let font = Font::new("Microsoft YaHei");
        let mut rect = rdr.measure_text(&font, &"屠龙宝刀，点击就送");
        let rect = rect.move_to_target_center(&self.rect);

        rdr.store(&rect, pen);

        rdr.draw_text_rect(&rect, &font, &color, &"屠龙宝刀，点击就送");
    }

    fn on_event(&mut self, em: EventMessage) -> bool {
        todo!()
    }
}

impl Deref for Button {
    type Target = ControlState;

    fn deref(&self) -> &Self::Target {
        &self.state
    }
}

impl DerefMut for Button {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.state
    }
}