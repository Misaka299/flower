use flower_base::control::Control;
use flower_base::event::EventMessage;
use flower_base::graphics::color::Color;
use flower_base::graphics::font::Font;
use flower_base::graphics::pen::Pen;
use flower_base::graphics::Render;
use flower_base::graphics::renderer::Renderer;
use flower_macro::control;

#[control]
pub struct Button {
    state: u8,
}

impl Button {
    pub fn create() -> Button {
        Self::create_control("name", Rect {
            left: 100.0,
            top: 100.0,
            width: 200.0,
            height: 200.0,
        }, 0)
    }
}

impl Control for Button {
    fn on_draw(&mut self, rdr: &mut Renderer) {
        // println!("draw button");

        let color = match self.state {
            2 => Color {
                r: 255,
                g: 40,
                b: 04,
                a: 255,
            },
            1 => Color {
                r: 200,
                g: 40,
                b: 04,
                a: 255,
            },
            _ => Color {
                r: 180,
                g: 40,
                b: 04,
                a: 255,
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

        //rdr.draw_text_rect(&rect, &font, &color, &"屠龙宝刀，点击就送");
    }

    fn on_event(&mut self, em: EventMessage) -> bool {
        self.state = match em {
            EventMessage::LButtonDown(_, _, _) => self.state + 1,
            EventMessage::LButtonUp(_, _, _) => self.state - 1,
            EventMessage::MouseEnter => self.state + 1,
            EventMessage::MouseLeave => self.state - 1,
            _ => self.state
        };
        // 调用真刷新。其他地方基本上都是假刷新，只有有变化，或者手动调用是真刷新
        self.redraw();
        true
    }
}