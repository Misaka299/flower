use std::ops::Deref;
use crate::ControlType;
use crate::control::control::{Control, ControlState};

pub struct Label {
    control_state: ControlState,
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

    fn find_event_control_id(&self, x: i32, y: i32) -> (u8, i32){
        (1,self.id())
    }
}