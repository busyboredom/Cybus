// Implementations of each type of mouse event.

use crate::events::common::*;
use std::string;

// ------------------------------------------- Mouse Moved ----------------------------------------

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
        &EventCategory::EventCategoryMouse
    }
}

// ------------------------------------------ Mouse Scrolled -------------------------------------------

pub struct MouseScrolledEvent {
    pub handled: bool,
    x_offset: f32,
    y_offset: f32,
}

impl MouseScrolledEvent {
    pub fn x_offset(&self) -> &f32 {
        &self.x_offset
    }
    pub fn y_offset(&self) -> &f32 {
        &self.y_offset
    }
}

impl string::ToString for MouseScrolledEvent {
    fn to_string(&self) -> String {
        let ss = "MouseScrolledEvent x_offset: ".to_string()
            + &self.x_offset.to_string()
            + &", y_offset: ".to_string()
            + &self.y_offset.to_string();
        ss
    }
}

impl Event for MouseScrolledEvent {
    fn event_type(&self) -> &EventType {
        &EventType::MouseScrolled
    }

    fn event_category(&self) -> &EventCategory {
        &EventCategory::EventCategoryMouse
    }
}

// ---------------------------------------- Mouse Pressed ------------------------------------------

pub struct MousePressedEvent {
    pub handled: bool,
    mouse_pressed_code: u8,
}

impl MousePressedEvent {
    pub fn mouse_pressed_code(&self) -> &u8 {
        &self.mouse_pressed_code
    }
}

impl string::ToString for MousePressedEvent {
    fn to_string(&self) -> String {
        let s = "MousePressedEvent Code: ".to_string() + &self.mouse_pressed_code.to_string();
        s
    }
}

impl Event for MousePressedEvent {
    fn event_type(&self) -> &EventType {
        &EventType::MouseButtonPressed
    }

    fn event_category(&self) -> &EventCategory {
        &EventCategory::EventCategoryMouse
    }
}

// ------------------------------------------- Tests ----------------------------------------------

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

    #[test]
    fn mouse_scrolled_to_string() {
        let mouse_scroll = crate::events::MouseScrolledEvent {
            handled: false,
            x_offset: 2.3,
            y_offset: 3.2,
        };

        assert_eq!(
            "MouseScrolledEvent x_offset: 2.3, y_offset: 3.2",
            mouse_scroll.to_string()
        );
    }

    #[test]
    fn mouse_pressed_to_string() {
        let mouse_press = crate::events::MousePressedEvent {
            handled: false,
            mouse_pressed_code: 5,
        };

        assert_eq!("MousePressedEvent Code: 5", mouse_press.to_string());
    }
}
