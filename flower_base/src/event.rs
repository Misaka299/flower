use glutin::event::ModifiersState;

use crate::graphics::rect::Rect;

#[derive(Eq, PartialEq, Hash)]
pub enum EventType {
    LButtonDown,
    LButtonClick,
    LButtonUp,
    RButtonDown,
    RButtonClick,
    RButtonUp,
    MButtonDown,
    MButtonClick,
    MButtonUp,
    OtherButtonDown,
    OtherButtonClick,
    OtherButtonUp,
    MouseEnter,
    MouseLeave,
    MouseMove,
    FocusGet,
    FocusLost,
    ReSize,
}

#[derive(Copy, Clone, Debug)]
pub enum EventMessage {
    LButtonDown(i32, i32, ModifiersState),
    LButtonClick(i32, i32, ModifiersState),
    LButtonUp(i32, i32, ModifiersState),
    RButtonDown(i32, i32, ModifiersState),
    RButtonClick(i32, i32, ModifiersState),
    RButtonUp(i32, i32, ModifiersState),
    MButtonDown(i32, i32, ModifiersState),
    MButtonClick(i32, i32, ModifiersState),
    MButtonUp(i32, i32, ModifiersState),
    OtherButtonDown(i32, i32, ModifiersState),
    OtherButtonClick(i32, i32, ModifiersState),
    OtherButtonUp(i32, i32, ModifiersState),
    MouseEnter,
    MouseLeave,
    MouseMove(i32, i32, ModifiersState),
    FocusGet,
    FocusLost,
    ReSize(Rect),
}

impl Into<EventType> for EventMessage {
    fn into(self) -> EventType {
        match self {
            EventMessage::LButtonDown { .. } => EventType::LButtonUp,
            EventMessage::LButtonClick { .. } => EventType::LButtonClick,
            EventMessage::LButtonUp { .. } => EventType::LButtonUp,
            EventMessage::RButtonDown { .. } => EventType::RButtonDown,
            EventMessage::RButtonClick { .. } => EventType::RButtonClick,
            EventMessage::RButtonUp { .. } => EventType::RButtonUp,
            EventMessage::MButtonDown { .. } => EventType::MButtonDown,
            EventMessage::MButtonClick { .. } => EventType::MButtonClick,
            EventMessage::MButtonUp { .. } => EventType::MButtonUp,
            EventMessage::OtherButtonDown { .. } => EventType::OtherButtonDown,
            EventMessage::OtherButtonClick { .. } => EventType::OtherButtonClick,
            EventMessage::OtherButtonUp { .. } => EventType::OtherButtonUp,
            EventMessage::MouseEnter { .. } => EventType::MouseEnter,
            EventMessage::MouseLeave { .. } => EventType::MouseLeave,
            EventMessage::MouseMove { .. } => EventType::MouseMove,
            EventMessage::FocusGet { .. } => EventType::FocusGet,
            EventMessage::FocusLost { .. } => EventType::FocusLost,
            EventMessage::ReSize(_) => { EventType::ReSize }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum EventFn {
    LButtonDown(fn(i32, i32, ModifiersState)),
    LButtonClick(fn(i32, i32, ModifiersState)),
    LButtonUp(fn(i32, i32, ModifiersState)),
    RButtonDown(fn(i32, i32, ModifiersState)),
    RButtonClick(fn(i32, i32, ModifiersState)),
    RButtonUp(fn(i32, i32, ModifiersState)),
    MButtonDown(fn(i32, i32, ModifiersState)),
    MButtonClick(fn(i32, i32, ModifiersState)),
    MButtonUp(fn(i32, i32, ModifiersState)),
    OtherButtonDown(fn(i32, i32, ModifiersState)),
    OtherButtonClick(fn(i32, i32, ModifiersState)),
    OtherButtonUp(fn(i32, i32, ModifiersState)),
    MouseEnter(fn()),
    MouseLeave(fn()),
    /// x:i32
    /// y:i32,
    /// ModifiersState
    MouseMove(fn(i32, i32, ModifiersState)),
    FocusGet(fn()),
    FocusLost(fn()),
    ReSize(fn(Rect)),
}

impl Into<EventType> for EventFn {
    fn into(self) -> EventType {
        match self {
            EventFn::LButtonDown { .. } => EventType::LButtonUp,
            EventFn::LButtonClick { .. } => EventType::LButtonClick,
            EventFn::LButtonUp { .. } => EventType::LButtonUp,
            EventFn::RButtonDown { .. } => EventType::RButtonDown,
            EventFn::RButtonClick { .. } => EventType::RButtonClick,
            EventFn::RButtonUp { .. } => EventType::RButtonUp,
            EventFn::MButtonDown { .. } => EventType::MButtonDown,
            EventFn::MButtonClick { .. } => EventType::MButtonClick,
            EventFn::MButtonUp { .. } => EventType::MButtonUp,
            EventFn::OtherButtonDown { .. } => EventType::OtherButtonDown,
            EventFn::OtherButtonClick { .. } => EventType::OtherButtonClick,
            EventFn::OtherButtonUp { .. } => EventType::OtherButtonUp,
            EventFn::MouseEnter { .. } => EventType::MouseEnter,
            EventFn::MouseLeave { .. } => EventType::MouseLeave,
            EventFn::MouseMove { .. } => EventType::MouseMove,
            EventFn::FocusGet { .. } => EventType::FocusGet,
            EventFn::FocusLost { .. } => EventType::FocusLost,
            EventFn::ReSize { .. } => EventType::ReSize,
        }
    }
}