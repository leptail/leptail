use std::{iter::Map, usize};

use leptos::{
    html::ElementType,
    prelude::{
        signal, store_value, use_context, Callable, Callback, MaybeProp, NodeRef, RwSignal, Signal,
        StoredValue,
    },
    tachys::renderer::mock_dom::Element,
};
use web_sys::{EventTarget, KeyboardEvent};

use crate::{
    dom::{DOMAttributes, DOMProps, FocusableElement},
    types::events::{KeyModifiers, PointerType, PressEvents},
};

pub struct PressProps {
    press_events: PressEvents,
    /// Whether the target is in a controlled press state (e.g. an overlay it triggers is open).
    is_pressed: MaybeProp<bool>,
    /// Whether the press events should be disabled.
    is_disabled: MaybeProp<bool>,
    /// Whether the target should not receive focus on press.
    prevent_focus_on_press: MaybeProp<bool>,
    /// Whether press events should be canceled when the pointer leaves the target while pressed.
    /// By default, this is `false`, which means if the pointer returns back over the target while
    /// still pressed, onPressStart will be fired again. If set to `true`, the press is canceled
    /// when the pointer leaves the target and onPressStart will not be fired if the pointer returns.
    should_cancel_on_pointer_exit: MaybeProp<bool>,
    /// Whether text selection should be enabled on the pressable element.
    allow_text_selection_on_press: MaybeProp<bool>,
}

pub struct PressState {
    is_pressed: bool,
    ignore_emulated_mouse_events: bool,
    ignore_click_after_press: bool,
    did_fire_press_start: bool,
    is_triggering_event: bool,
    active_pointer_id: Option<usize>,
    //TODO: target: Option<FocusableElement>,
    is_over_target: bool,
    pointer_type: Option<PointerType>,
    user_select: Option<&'static str>,
    meta_key_evnts: Option<Map<&'static str, KeyboardEvent>>,
}

pub struct PressHookProps {
    press_props: PressProps,
    // element_ref: Option<NodeRef<T>>,
}

pub struct EventBase {
    current_target: EventTarget,
    key_modifiers: KeyModifiers,
}

pub struct PressResult {
    /// Whether the target is currently pressed.
    is_pressed: bool,
    /// Props to spread on the target element.
    press_props: DOMAttributes,
    test: DOMProps,
}

#[derive(Clone)]
pub struct PressResponderContext {
    register: Callback<()>,
    // node_ref: NodeRef<T>,
}

// TODO: implement function usePressResponderContext(props: PressHookProps): PressHookProps
fn use_press_responder_context(props: PressHookProps) -> PressHookProps {
    let context = use_context::<PressResponderContext>();
    if let Some(context) = context {
        // let {register, ...contextProps} = context;
        // props = mergeProps(contextProps, props) as PressHookProps;
        // register();
        context.register.call(());
    }
    // useSyncRef(context, props.ref);
    // return props;
    todo!()
}

// TODO:: implement class PressEvent implements IPressEvent

/// Handles press interactions across mouse, touch, keyboard, and screen readers.
/// A press interaction starts when a user presses down with a mouse or their
/// finger on the target, and ends when they move the pointer off the target.
/// It may start again if the pointer re-enters the target.
/// use_press returns the current press state, which can be used to adjust the visual
/// appearance of the target. If the pointer is released over the target,
/// then an onPress event is fired.
pub fn use_press(props: PressHookProps) -> PressResult {
    let press_responder_context = use_press_responder_context(props);
    let (is_pressed, set_pressed) = signal(false);
    let press_state = store_value::<Option<PressState>>(None);

    // let press_state = StoredValue::<Option<PressState>>::new(None);
    todo!()
}
