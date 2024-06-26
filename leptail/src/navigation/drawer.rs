use crate::prelude::*;
use leptos::*;

#[derive(Debug, Clone, Default)]
pub struct DrawerTheme {
    pub wrapper: String,
    pub inner: String,
    pub minimized: String,
    pub maximized: String,
    pub overlay_theme: Option<OverlayTheme>,
}

// Drawer Variants and Funtions
// -----------------------------
// TODO:
// 6. Focus on specific element -- Prop (Seen on chakra UI)
// 7. Disable click on the backdrop to close -- Prop
// 8. Close on escape key press -- Prop --
#[component]
pub fn Drawer(
    /// state to control if the drawer is open or not
    #[prop(into)]
    is_open: MaybeSignal<bool>,
    /// state to control if when backdrop is clicked on the drawer
    #[prop(into)]
    set_open: Out<bool>,
    /// optional drawer theme variant
    #[prop(into, optional)]
    variant: OptionalMaybeSignal<DrawerTheme>,
    children: Children,
) -> impl IntoView {
    let theme = variant.or_else(|| use_context::<DrawerTheme>().unwrap_or_default());

    // TODO: check is there away to get away from cloning the signal. Does cloning signals creates a new signal?
    let drawer_wrapper = {
        let cloned = theme.clone();
        move || {
            with!(|is_open, cloned| {
                let dim = if *is_open {
                    cloned.maximized.clone()
                } else {
                    cloned.minimized.clone()
                };
                format!("{} {}", cloned.wrapper.clone(), dim)
            })
        }
    };

    let drawer_inner = {
        let cloned = theme.clone();
        move || with!(|cloned| cloned.inner.clone())
    };

    let overlay_variant = {
        let cloned = theme.clone();
        move || {
            with!(|cloned| cloned
                .overlay_theme
                .clone()
                .unwrap_or_else(|| use_context::<OverlayTheme>().unwrap_or_default()))
        }
    };
    let overlay_variant = Signal::derive(overlay_variant);

    view! {
        <>
            <div class=drawer_wrapper on:click=move |e| { e.stop_propagation() }>
                <div class=drawer_inner>{children()}</div>
            </div>
            <Show
                when=is_open
                fallback=move || {
                    view! { <></> }
                }
            >

                <Overlay on_click=move || set_open.set(false) variant=overlay_variant>
                    <div></div>
                </Overlay>
            </Show>
        </>
    }
}
