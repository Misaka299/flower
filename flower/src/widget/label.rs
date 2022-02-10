use std::ops::Deref;
use crate::controlType;
use crate::widget::control::{control, controltate};

pub struct Label {
    control_state: controltate,
}


impl Deref for Label {
    type Target = controltate;

    fn deref(&self) -> &Self::Target {
        &self.control_state
    }
}

impl control for Label {
    fn get_control_type(&self) -> controlType {
        controlType::LABEL
    }

    fn find_event_control_id(&self, x: i32, y: i32) -> (u8, i32){
        (1,self.id())
    }
}