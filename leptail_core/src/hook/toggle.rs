use leptos::{ev::EventCallback, html::Input, prelude::*};
use tachys::dom::event_target_checked;
use web_sys::{Event, HtmlInputElement};

use crate::{
    dom::{AriaLabelingProps, AriaValidationProps, FocusableDOMProps, InputDOMProps},
    state::toggle::ToggleState,
};

struct ToggleProps {
    /// The label for the element.
    children: Option<Children>,
    /// Whether the element should be selected (uncontrolled).
    default_selected: Option<bool>,
    /// Whether the element should be selected (controlled).
    is_selected: Option<bool>,
    /// Handler that is called when the element's selection state changes.
    on_change: Option<Callback<bool>>,
    /// The value of the input element, used when submitting an HTML form.
    /// See [MDN](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input#htmlattrdefvalue).
    value: Option<&'static str>,
}

struct AriaToggleProps {
    // TODO: add toggle props here..
    toggle_props: ToggleProps,
    focusable_dom_props: FocusableDOMProps,
    aria_labeling_props: AriaLabelingProps,
    aria_validation_props: AriaValidationProps,
    input_dom_props: InputDOMProps,
    /// Identifies the element (or elements) whose contents or presence are controlled by the current element.
    aria_controls: Option<&'static str>,
}

pub fn use_toggle(
    props: AriaToggleProps,
    state: ToggleState,
    node_ref: Option<NodeRef<Input>>,
) -> () {
    let on_change = Callback::new(move |e: Event| {
        e.stop_propagation();
        state.set(event_target_checked(&e));
        // From solid-aria: Unlike in React, inputs `checked` state can be out of sync with our toggle state.
        // From solid-aria: `target.checked = state.isSelected();`
        // TODO: Not sure if the above code needs to be implemented.
    });

    //   let hasChildren = children != null;
    //   let hasAriaLabel = ariaLabel != null || ariaLabelledby != null;
    //   if (!hasChildren && !hasAriaLabel) {
    //     console.warn('If you do not provide children, you must specify an aria-label for accessibility');
    //   }

    // TODO: create use_press

    todo!()
}
