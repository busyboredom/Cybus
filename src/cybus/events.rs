// This file contains the implementations of all event key, mouse, window and application events.

use std::string;

// ---------------- Enumerations of event types, and a trait common to all events. ----------------

pub enum EventType {
    //Misc. events.
    NoEvent = 0,

    // Window Events.
    WindowClose,
    WindowResize,
    WindowFocus,
    WindowMoved,

    // Application Events.
    AppTick,
    AppUpdate,
    AppRender,

    // Key Events.
    KeyPressed,
    KeyReleased,

    // Mouse Events.
    MouseButtonPressed,
    MouseButtonReleased,
    MouseMoved,
    MouseScrolled,
}

pub enum EventCategory {
    NoEvent = 0,
    EventCategoryWindow,
    EventCategoryApplication,
    EventCategoryKey,
    EventCategoryMouseButton,
}

pub trait Event {
    fn event_type(&self) -> &EventType;
    fn event_category(&self) -> &EventCategory;
}

// ------------------------------------------ Key Events ------------------------------------------

pub struct KeyPressedEvent {
    pub handled: bool,
    pub key_code: u8,
    pub repeat_count: u64,
}

impl KeyPressedEvent {
    pub fn key_code(&self) -> &u8 {
        &self.key_code
    }

    pub fn repeat_count(&self) -> &u64 {
        &self.repeat_count
    }
}

impl string::ToString for KeyPressedEvent {
    fn to_string(&self) -> String {
        let s = "KeyPressedEvent Key Code: ".to_string()
            + &self.key_code.to_string()
            + &", Repeats: ".to_string()
            + &self.repeat_count.to_string();
        s
    }
}

impl Event for KeyPressedEvent {
    fn event_type(&self) -> &EventType {
        &EventType::KeyPressed
    }

    fn event_category(&self) -> &EventCategory {
        &EventCategory::EventCategoryKey
    }
}

// ------------------------------------------- Tests ----------------------------------------------

#[cfg(test)]
mod tests {
    #[test]
    fn key_press_to_string() {
        let key_press = crate::events::KeyPressedEvent {
            handled: false,
            key_code: 5,
            repeat_count: 10,
        };

        assert_eq!(
            "KeyPressedEvent Key Code: 5, Repeats: 10",
            key_press.to_string()
        );
    }
}
