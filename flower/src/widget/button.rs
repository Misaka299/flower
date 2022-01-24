use std::ops::Deref;
use crate::widget::widget::WidgetState;

pub struct Button {
    widget_state: WidgetState,
    title: String,
    on_click: Option<Box<dyn Fn()>>,
}

impl Button {
    pub fn from(title: String) -> Button {
        Button {
            widget_state: WidgetState::create(vec![], 0, 0),
            title,
            on_click: None,
        }
    }
    fn on_click(mut self, fn_on_click: Box<dyn Fn()>) -> Self {
        self.on_click = Some(fn_on_click);
        self
    }
    fn set_text(mut self, title: String) -> Self {
        self.title = title;
        self
    }
    fn get_text(self) -> String {
        self.title
    }
}

impl Deref for Button {
    type Target = WidgetState;

    fn deref(&self) -> &WidgetState {
        &self.widget_state
    }
}