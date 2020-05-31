// Implementations of each type of application event.

use crate::events::common::*;
use std::string;

// ---------------------------------------- App Tick -----------------------------------------

pub struct AppTickEvent {
    pub handled: bool,
}

impl string::ToString for AppTickEvent {
    fn to_string(&self) -> String {
        "AppTickEvent".to_string()
    }
}

impl Event for AppTickEvent {
    fn event_type(&self) -> &EventType {
        &EventType::AppTick
    }

    fn event_category(&self) -> &EventCategory {
        &EventCategory::EventCategoryApplication
    }
}

// ---------------------------------------- App Update -----------------------------------------

pub struct AppUpdateEvent {
    pub handled: bool,
}

impl string::ToString for AppUpdateEvent {
    fn to_string(&self) -> String {
        "AppUpdateEvent".to_string()
    }
}

impl Event for AppUpdateEvent {
    fn event_type(&self) -> &EventType {
        &EventType::AppUpdate
    }

    fn event_category(&self) -> &EventCategory {
        &EventCategory::EventCategoryApplication
    }
}

// ---------------------------------------- App Render -----------------------------------------

pub struct AppRenderEvent {
    pub handled: bool,
}

impl string::ToString for AppRenderEvent {
    fn to_string(&self) -> String {
        "AppRenderEvent".to_string()
    }
}

impl Event for AppRenderEvent {
    fn event_type(&self) -> &EventType {
        &EventType::AppRender
    }

    fn event_category(&self) -> &EventCategory {
        &EventCategory::EventCategoryApplication
    }
}

// ------------------------------------------- Tests ----------------------------------------------

#[cfg(test)]
mod tests {
    #[test]
    fn app_tick_to_string() {
        let app_tick = crate::events::AppTickEvent { handled: false };

        assert_eq!("AppTickEvent", app_tick.to_string());
    }

    #[test]
    fn app_update_to_string() {
        let app_update = crate::events::AppUpdateEvent { handled: false };

        assert_eq!("AppUpdateEvent", app_update.to_string());
    }

    #[test]
    fn app_render_to_string() {
        let app_render = crate::events::AppRenderEvent { handled: false };

        assert_eq!("AppRenderEvent", app_render.to_string());
    }
}
