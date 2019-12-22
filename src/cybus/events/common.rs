// Enumerations of event types and categories, as well as a common event trait and the event queue.

// ---------------- Enumerations of event types, and a trait common to all events. ----------------

pub enum EventType {
    //Misc. events.
    NoEvent = 0,

    // Window Events.
    WindowClosed,
    WindowResized,
    WindowFocus,
    WindowMoved,

    // Application Events.
    AppTick,
    AppUpdate,
    AppRender,

    // Key Events.
    KeyPressed,
    KeyReleased,
    KeyTyped,

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
    EventCategoryMouse,
}

pub trait Event {
    fn event_type(&self) -> &EventType;
    fn event_category(&self) -> &EventCategory;
}
