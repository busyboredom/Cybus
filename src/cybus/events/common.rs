// Enumerations of ecent types and categories, as well as a common event trait.

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
