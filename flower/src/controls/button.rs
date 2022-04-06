use std::ops::{Deref, DerefMut};
use glow::HasContext;

use log::debug;

use crate::color::Color;
use crate::control::{Control, ControlState, ControlType};
use crate::draw::Draw;
use crate::rect::Rect;

pub struct Button {
    control_state: ControlState,
    text: String,
    on_click: Option<Box<dyn Fn()>>,
}

impl Button {
    pub fn from(name: String, text: String) -> Button {
        Button {
            control_state: ControlState::create(name, false, ControlType::Control),
            text,
            on_click: None,
        }
    }
    pub fn on_click(&mut self, fn_on_click: Box<dyn Fn()>) -> &mut Self {
        self.on_click = Some(fn_on_click);
        self
    }
    pub fn set_text(&mut self, text: String) -> &mut Self {
        self.text = text;
        self
    }
    pub fn get_text(&self) -> String {
        self.text.clone()
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
        &mut self.control_state
    }
}

impl Control for Button {
    fn on_draw(&mut self, gl: &mut Draw) {
        println!("button[{}] draw rect {:?}",self.id(),&self.rect);
        gl.create_canvas(&Rect::new(self.base_left + self.left,self.base_top + self.top, self.width, self.height));
        println!("button[{}] focus {}",self.id(), self.focus);
        if self.focus {
            gl.fill(&self.rect, &Color::rgb(0, 191, 255));
        } else {
            gl.rect(&self.rect, &Color::rgb(255, 191, 255));
        }
    }
}