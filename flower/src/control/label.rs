use std::ops::Deref;
use miniquad::graphics;
use nona::{Align, Canvas, Color, Context, Gradient, Point};

use nonaquad::nvgimpl;

use crate::control::control::{Control, ControlState};
use crate::ControlType;

pub struct Label {
    control_state: ControlState,
}

impl Label {
    pub fn create() -> Label {
        let state = ControlState::create(vec![], ControlType::LABEL, 0, 0);
        Label{
            control_state: state
        }
    }
}


impl Deref for Label {
    type Target = ControlState;

    fn deref(&self) -> &Self::Target {
        &self.control_state
    }
}

impl Control for Label {
    fn get_control_type(&self) -> ControlType {
        ControlType::LABEL
    }

    fn on_draw(&self) {

    }
}