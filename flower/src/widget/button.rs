use std::ops::Deref;

use crate::widget::control::{control, controltate, controlType};

pub struct Button {
    control_state: controltate,
    title: String,
    on_click: Option<Box<dyn Fn()>>,
}

impl Button {
    pub fn from(title: String) -> Button {
        Button {
            control_state: controltate::create(vec![], controlType::BUTTON, 0, 0),
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
    type Target = controltate;

    fn deref(&self) -> &Self::Target {
        &self.control_state
    }
}

impl control for Button {
    fn get_control_type(&self) -> controlType {
        controlType::BUTTON
    }

    fn find_event_control_id(&self, x: i32, y: i32) -> (u8, i32){
        (1,self.id())
    }
}