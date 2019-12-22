// Implementations of each type of key event.

use crate::events::common::*;
use std::string;

// ------------------------------------------- Key Pressed ----------------------------------------

pub struct KeyPressedEvent {
    pub handled: bool,
    key_code: u8,
    repeat_count: u64,
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

// ---------------------------------------- Key Released ------------------------------------------

pub struct KeyReleasedEvent {
    pub handled: bool,
    key_code: u8,
}

impl KeyReleasedEvent {
    pub fn key_code(&self) -> &u8 {
        &self.key_code
    }
}

impl string::ToString for KeyReleasedEvent {
    fn to_string(&self) -> String {
        let s = "KeyReleasedEvent Key Code: ".to_string() + &self.key_code.to_string();
        s
    }
}

impl Event for KeyReleasedEvent {
    fn event_type(&self) -> &EventType {
        &EventType::KeyReleased
    }

    fn event_category(&self) -> &EventCategory {
        &EventCategory::EventCategoryKey
    }
}

// ------------------------------------------ Key Typed -------------------------------------------

pub struct KeyTypedEvent {
    pub handled: bool,
    key_code: u8,
}

impl KeyTypedEvent {
    pub fn key_code(&self) -> &u8 {
        &self.key_code
    }
}

impl string::ToString for KeyTypedEvent {
    fn to_string(&self) -> String {
        let s = "KeyTypedEvent Key Code: ".to_string() + &self.key_code.to_string();
        s
    }
}

impl Event for KeyTypedEvent {
    fn event_type(&self) -> &EventType {
        &EventType::KeyTyped
    }

    fn event_category(&self) -> &EventCategory {
        &EventCategory::EventCategoryKey
    }
}

// ------------------------------------------- Tests ----------------------------------------------

#[cfg(test)]
mod tests {
    #[test]
    fn key_pressed_to_string() {
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

    #[test]
    fn key_released_to_string() {
        let key_release = crate::events::KeyReleasedEvent {
            handled: false,
            key_code: 5,
        };

        assert_eq!("KeyReleasedEvent Key Code: 5", key_release.to_string());
    }

    #[test]
    fn key_typed_to_string() {
        let key_typed = crate::events::KeyTypedEvent {
            handled: false,
            key_code: 5,
        };

        assert_eq!("KeyTypedEvent Key Code: 5", key_typed.to_string());
    }
}
