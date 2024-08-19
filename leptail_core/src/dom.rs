use leptos::{
    attr::{Attribute, AttributeValue},
    html::ElementType,
    prelude::*,
};
use web_sys::{ClipboardEvent, CompositionEvent, Event, InputEvent};

// TODO: this is an exact port of line by line code from the react spectrum
// TODO: need to analyze the usage and refactor into more idiomatic rust and typesafe way!
pub struct AriaLabelingProps {
    /// Defines a string value that labels the current element.
    label: Option<&'static str>,
    /// Identifies the element (or elements) that labels the current element.
    labelled_by: Option<&'static str>,
    /// Identifies the element (or elements) that describes the object.
    described_by: Option<&'static str>,
    /// Identifies the element (or elements) that provide a detailed, extended description for the object.
    details: Option<&'static str>,
}

pub struct AriaValidationProps {
    /// Identifies the element that provides an error message for the object.
    /// See [WAI-ARIA](https://www.w3.org/TR/wai-aria-1.2/#aria-errormessage)
    error_message: Option<&'static str>,
}

pub struct DOMProps {
    /// The element's unique identifier.
    /// See [MDN](https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes/id).
    id: Option<&'static str>,
}

/// Whether to exclude the element from the sequential tab order. If true,
/// the element will not be focusable via the keyboard by tabbing. This should
/// be avoided except in rare scenarios where an alternative means of accessing
/// the element or its functionality via the keyboard is available.
pub struct FocusableDOMProps {
    dom_props: DOMProps,
    exclude_from_tab_order: bool,
}

pub struct TextInputDOMEvents {
    // Clipboard events
    /// Handler that is called when the user copies text. See [MDN](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/oncopy).
    on_copy: Option<Callback<ClipboardEvent>>,
    /// Handler that is called when the user cuts text. See [MDN](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/oncut).
    on_cut: Option<Callback<ClipboardEvent>>,
    /// Handler that is called when the user pastes text. See [MDN](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/onpaste).
    on_paste: Option<Callback<ClipboardEvent>>,
    /// Handler that is called when a text composition system starts a new text composition session. See [MDN](https://developer.mozilla.org/en-US/docs/Web/API/Element/compositionstart_event).
    on_composition_start: Option<Callback<CompositionEvent>>,
    /// Handler that is called when a text composition system completes or cancels the current text composition session. See [MDN](https://developer.mozilla.org/en-US/docs/Web/API/Element/compositionend_event).
    on_composition_end: Option<Callback<CompositionEvent>>,
    /// Handler that is called when a new character is received in the current text composition session. See [MDN](https://developer.mozilla.org/en-US/docs/Web/API/Element/compositionupdate_event).
    on_composition_update: Option<Callback<CompositionEvent>>,
    /// Handler that is called when text in the input is selected. See [MDN](https://developer.mozilla.org/en-US/docs/Web/API/Element/select_event).
    on_select: Option<Callback<Event>>,
    /// Handler that is called when the input value is about to be modified. See [MDN](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/beforeinput_event)
    on_before_input: Option<Callback<InputEvent>>,
    /// Handler that is called when the input value is modified. See [MDN](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/input_event).
    on_input: Option<Callback<InputEvent>>,
}

pub struct InputDOMProps {
    /// The name of the input element, used when submitting an HTML form. See [MDN](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input#htmlattrdefname).
    name: Option<&'static str>,
}

pub struct TextInputDOMProps {
    dom_props: DOMProps,
    input_dom_props: InputDOMProps,
    text_input_events: TextInputDOMEvents,
    /// Describes the type of autocomplete functionality the input should provide if any. See [MDN](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input#htmlattrdefautocomplete).
    auto_complete: Option<&'static str>,
    /// The maximum number of characters supported by the input. See [MDN](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input#htmlattrdefmaxlength).
    max_length: Option<usize>,
    /// The minimum number of characters required by the input. See [MDN](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input#htmlattrdefminlength).
    min_length: Option<usize>,
    // TODO: add more if required or use leptos attribute
    // pattern, placeholder, type, inputMode
}

pub trait RouterConfig {}

// TOOD: create Href and RouterOptions

pub struct LinkDOMProps {
    // TODO: add the body
}

/// Any focusable element, including both HTML and SVG elements.
pub trait FocusableElement {
    // TODO: make it inherit Element or HTMLOrSVGElement
}

pub struct DOMAttributes {
    // TODO: fill up the body
}

pub struct GroupDOMAttributes {
    // TODO: fill up the body
}
