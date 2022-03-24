use std::ops::{Deref, DerefMut};

use glow::Context;

use crate::control::control::{Control, ControlType};
use crate::ControlState;

pub struct Button {
    control_state: ControlState,
    title: String,
    on_click: Option<Box<dyn Fn()>>,
}

impl Button {
    pub fn from(name: String, title: String) -> Button {
        Button {
            control_state: ControlState::create(name, vec![], ControlType::Control, 0, 0),
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

    fn deref(&self) -> &Self::Target {
        &self.control_state
    }
}

impl DerefMut for Button {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self
    }
}

impl Control for Button {
    fn on_draw(&mut self, gl: &Context) {
        todo!()
    }
}