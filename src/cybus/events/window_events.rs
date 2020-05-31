// Implementations of each type of window event.

use crate::events::common::*;
use std::string;

// ------------------------------------------ Window Resized --------------------------------------

pub struct WindowResizedEvent {
    pub handled: bool,
    pub width: u32,
    pub height: u32,
}

impl WindowResizedEvent {
    pub fn width(&self) -> &u32 {
        &self.width
    }

    pub fn height(&self) -> &u32 {
        &self.height
    }
}

impl string::ToString for WindowResizedEvent {
    fn to_string(&self) -> String {
        "WindowResizedEvent Width: ".to_string()
            + &self.width.to_string()
            + &", Height: ".to_string()
            + &self.height.to_string()
    }
}

impl Event for WindowResizedEvent {
    fn event_type(&self) -> &EventType {
        &EventType::WindowResized
    }

    fn event_category(&self) -> &EventCategory {
        &EventCategory::EventCategoryWindow
    }
}

// ---------------------------------------- Window Closed -----------------------------------------

pub struct WindowClosedEvent {
    pub handled: bool,
}

impl string::ToString for WindowClosedEvent {
    fn to_string(&self) -> String {
        "WindowClosedEvent".to_string()
    }
}

impl Event for WindowClosedEvent {
    fn event_type(&self) -> &EventType {
        &EventType::WindowClosed
    }

    fn event_category(&self) -> &EventCategory {
        &EventCategory::EventCategoryWindow
    }
}

// ------------------------------------------- Tests ----------------------------------------------

#[cfg(test)]
mod tests {
    #[test]
    fn window_resized_to_string() {
        let window_resize = crate::events::WindowResizedEvent {
            handled: false,
            width: 1920,
            height: 1080,
        };

        assert_eq!(
            "WindowResizedEvent Width: 1920, Height: 1080",
            window_resize.to_string()
        );
    }

    #[test]
    fn window_closed_to_string() {
        let window_close = crate::events::WindowClosedEvent { handled: false };

        assert_eq!("WindowClosedEvent", window_close.to_string());
    }
}
