use leptos::prelude::Callback;
use web_sys::Element;

pub enum HoverPointerType {
    Mouse,
    Pen,
}

pub enum PointerType {
    HoverPointer(HoverPointerType),
    Touch,
    Keyboard,
    Virtual,
}

pub enum PressEventType {
    PressStart,
    PressEnd,
    PressUp,
    Press,
}

pub struct KeyModifiers {
    /// Whether the shift keyboard modifier was held during the press event.
    shift_key: bool,
    /// Whether the ctrl keyboard modifier was held during the press event.
    ctrl_key: bool,
    /// Whether the meta keyboard modifier was held during the press event.
    meta_key: bool,
    /// Whether the alt keyboard modifier was held during the press event.
    alt_key: bool,
}

pub struct PressEventPayload {
    /// The pointer type that triggered the press event.
    pointer_type: PointerType,
    /// The target element of the press event.
    target: Element,
    /// Keyboard modifier was held during the press event.
    key_modifiers: KeyModifiers,
    /// X position relative to the target.
    x: usize,
    /// Y position relative to the target.
    y: usize,
}

pub struct PressEvent {
    /// The type of press event being fired.
    event_type: PressEventType,
    /// Press event payload.
    payload: PressEventPayload,
    /// By default, press events stop propagation to parent elements.
    /// In cases where a handler decides not to handle a specific event,
    /// it can call `continue_propagation()` to allow a parent to handle it.
    continue_propagation: Callback<()>,
}
