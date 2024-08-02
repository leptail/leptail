use leptos::prelude::{Callback, MaybeProp};
use web_sys::{Element, FocusEvent, KeyboardEvent};

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

pub enum LongPressEventType {
    LongPressStart,
    LongPressEnd,
    LongPress,
}
pub struct LongPressEvent {
    /// The type of long press event being fired.
    event_type: PressEventType,
    /// Long press event payload.
    payload: PressEventPayload,
}

pub enum HoverEventType {
    HoverStart,
    HoverEnd,
}

pub struct HoverEvent {
    /// The type of hover event being fired.
    event_type: HoverEventType,
    /// The pointer type that triggered the hover event.
    pointer_type: HoverPointerType,
    /// The target element of the hover event.
    target: Element,
}

pub struct KeyboardEvents {
    /// Handler that is called when a key is pressed.
    on_key_down: Option<Callback<KeyboardEvent, ()>>,
    /// Handler that is called when a key is released.
    on_key_up: Option<Callback<KeyboardEvent, ()>>,
}

pub struct FocusEvents {
    /// Handler that is called when the element receives focus.
    on_focus: Option<Callback<FocusEvent, ()>>,
    /// Handler that is called when the element loses focus.
    on_blur: Option<Callback<FocusEvent, ()>>,
    /// Handler that is called when the element's focus status changes.
    on_focus_change: Option<Callback<bool, ()>>,
}

pub struct HoverEvents {
    /// Handler that is called when a hover interaction starts.
    on_hover_start: Option<Callback<HoverEvent, ()>>,
    /// Handler that is called when a hover interaction ends.
    on_hover_end: Option<Callback<HoverEvent, ()>>,
    /// Handler that is called when the hover state changes.
    on_hover_change: Option<Callback<bool, ()>>,
}

pub struct PressEvents {
    /// Handler that is called when the press is released over the target.
    on_press: Option<Callback<PressEvent, ()>>,
    /// Handler that is called when a press interaction starts.
    on_press_start: Option<Callback<PressEvent, ()>>,
    /// Handler that is called when a press interaction ends, either
    /// over the target or when the pointer leaves the target.
    on_press_end: Callback<PressEvent, ()>,
    /// Handler that is called when the press state changes.
    on_press_change: Callback<bool, ()>,
    /// Handler that is called when a press is released over the target, regardless of
    /// whether it started on the target or not.
    on_press_up: Callback<PressEvent, ()>,
}

// TODO: FocusWithinEvents from solid aria

pub struct FocusableProps {
    focus_events: FocusEvents,
    keyboard_events: KeyboardEvents,
    auto_focus: MaybeProp<bool>, // TODO: should this be MaybeProp or Option<MaybeSignal>?
}

pub struct BaseMoveEvent {
    /// The pointer type that triggered the move event.
    pointer_type: PointerType,
    /// Keyboard modifier was held during the move event.
    key_modifiers: KeyModifiers,
}

pub struct MoveStartEvent {
    payload: BaseMoveEvent,
}

pub struct MoveMoveEvent {
    /// The amount moved in the X direction since the last event.
    delta_x: usize,
    /// The amount moved in the Y direction since the last event.
    delta_y: usize,
    payload: BaseMoveEvent,
}

pub struct MoveEndEvent {
    payload: BaseMoveEvent,
}

pub enum MoveEventType {
    // TODO: should it include the payload?
    Start,
    Move,
    End,
}

pub struct MoveEvents {
    /// Handler that is called when a move interaction starts.
    on_move_start: Option<Callback<MoveStartEvent, ()>>,
    /// Handler that is called when the element is moved.
    on_move: Option<Callback<MoveMoveEvent, ()>>,
    /// Handler that is called when a move interaction ends.
    on_move_end: Option<Callback<MoveMoveEvent, ()>>,
}

pub struct ScrollEvent {
    /// The amount moved in the X direction since the last event.
    delta_x: usize,
    /// The amount moved in the Y direction since the last event.
    delta_y: usize,
}

pub struct ScrollEvents {
    /// Handler that is called when the scroll wheel moves.
    on_scroll: Option<Callback<ScrollEvent, ()>>,
}
