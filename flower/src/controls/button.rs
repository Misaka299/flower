use std::ops::{Deref, DerefMut};
use glow::HasContext;

use log::debug;

use crate::color::Color;
use crate::control::{Control, ControlState, ControlType, InteractiveState};
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
        // gl.create_canvas(&Rect::new(self.base_left + self.left,self.base_top + self.top, self.width, self.height));
        unsafe {
            // gl.clear_color(0.1, 0.2, 0.3, 1.0);
            // gl.clear(glow::COLOR_BUFFER_BIT);
        }
        println!("button[{}] focus {}",self.id(), self.focus);
        match self.interactive_state {
            InteractiveState::Ordinary => {
                gl.fill(&self.rect, &Color::from_hex_str("#FFF").unwrap());
            }
            InteractiveState::Active => {
                gl.fill(&self.rect, &Color::from_hex_str("#efefef").unwrap());
            }
            InteractiveState::Pressed => {
                gl.fill(&self.rect, &Color::from_hex_str("#3c4043").unwrap());
            }
            InteractiveState::Disable => {
                gl.fill(&self.rect, &Color::from_hex_str("##eaeaea").unwrap());
            }
        }
    }
}