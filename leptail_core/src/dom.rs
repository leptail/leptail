use leptos::{
    attr::{Attribute, AttributeValue},
    prelude::*,
};

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

pub struct InputDOMProps {
    /// The name of the input element, used when submitting an HTML form.
    /// See [MDN](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input#htmlattrdefname).
    name: Option<&'static str>,
}
