use crate::prelude::*;
use leptos::*;

#[derive(Debug, Clone, Default)]
pub struct OverlayTheme {
    pub wrapper: String,
    pub inner: String,
}

#[component]
pub fn Overlay<S>(
    /// callback when clicked on the backdrop
    on_click: S,
    /// optional drawer theme variant
    // #[prop(into, optional)]
    #[prop(into, optional)]
    variant: OptionalMaybeSignal<OverlayTheme>,
    children: Children,
) -> impl IntoView
where
    S: Fn() + 'static,
{
    let theme = variant.or_else(|| use_context::<OverlayTheme>().unwrap_or_default());

    let wrapper_class = {
        let cloned = theme.clone();
        move || cloned.with(|x| x.wrapper.clone())
    };

    let inner_class = {
        let cloned = theme.clone();
        move || cloned.with(|x| x.inner.clone())
    };

    view! {
        <div class=wrapper_class>
            <div class=inner_class on:click=move |_| (on_click)()>
                {children()}
            </div>
        </div>
    }
}
