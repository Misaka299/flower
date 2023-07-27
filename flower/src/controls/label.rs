use flower_base::control::Control;
use flower_base::event::EventMessage;
use flower_base::graphics::renderer::default::Renderer;
use flower_macro::control;


#[control]
pub struct Label {}

impl Control for Label {
    fn on_draw(&mut self, rdr: &mut Renderer) {

    }

    fn on_event(&mut self, em: EventMessage) -> bool {
        true
    }
}