// Implementations of each type of mouse event.

use crate::events::common::*;
use std::string;

pub struct MouseMovedEvent {
    pub handled: bool,
    x: f32,
    y: f32,
}

impl MouseMovedEvent {
    pub fn x(&self) -> &f32 {
        &self.x
    }
    pub fn y(&self) -> &f32 {
        &self.y
    }
}

impl string::ToString for MouseMovedEvent {
    fn to_string(&self) -> String {
        let s = "MouseMovedEvent x: ".to_string()
            + &self.x.to_string()
            + &", y: ".to_string()
            + &self.y.to_string();
        s
    }
}

impl Event for MouseMovedEvent {
    fn event_type(&self) -> &EventType {
        &EventType::MouseMoved
    }

    fn event_category(&self) -> &EventCategory {
        &EventCategory::EventCategoryMouseButton //Not sure
    }
}

pub struct MouseScrolledEvent {
    pub handled: bool,
    x_offset: f32,
    y_offset: f32,
}

#[cfg(test)]
mod tests {
    #[test]
    fn mouse_move_to_string() {
        let mouse_move = crate::events::MouseMovedEvent {
            handled: false,
            x: 2.2,
            y: 3.3,
        };
        assert_eq!("MouseMovedEvent x: 2.2, y: 3.3", mouse_move.to_string());
    }

}
