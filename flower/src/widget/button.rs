use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

use crate::widget::controls::{Controls, ControlState, ControlsType};

pub struct Button {
    control_state: ControlState,
    title: String,
    on_click: Option<Box<dyn Fn()>>,
}

impl Button {
    pub fn from(title: String) -> Button {
        Button {
            control_state: ControlState::create(vec![], ControlsType::BUTTON, 0, 0),
            title,
            on_click: None,
        }
    }
    pub fn on_click(&mut self, fn_on_click: Box<dyn Fn()>) -> &mut Self {
        self.on_click = Some(fn_on_click);
        self
    }
    pub fn set_text(&mut self, title: String) -> &mut Self {
        self.title = title;
        self
    }
    pub fn get_text(&self) -> String {
        self.title.clone()
    }
}

impl Deref for Button {
    type Target = ControlState;

    fn deref(&self) -> &ControlState {
        &self.control_state
    }
}

impl Controls for Button {
    fn get_controls_type(&self) -> ControlsType {
        ControlsType::BUTTON
    }
}