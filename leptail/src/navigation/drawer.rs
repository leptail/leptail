use std::borrow::Borrow;

use leptos::*;
use crate::{prelude::*, theme};


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
// 1. Drawer Sides -- Theme -- done
// 2. Inset drawers -- Theme -- done
// 3. Resposive drawers -- Theme -- done
// 4. Persistent Drawers -- Theme -- done
// 5. Drawer size -- Theme -- partial; fix size
// 6. Focus on specific element -- Prop (Seen on chakra UI)
// 7. Disable click on the backdrop to close -- Prop  
// 8. Close on escape key press -- Prop --  
// 9. Half minimized drawer -- Theme -- done
#[component]
pub fn Drawer( 
    /// state to control if the drawer is open or not
    #[prop(into)] is_open: MaybeSignal<bool>,
    /// state to control if when backdrop is clicked on the drawer
    #[prop(into)] set_open: Out<bool>, 
    /// optional drawer theme variant
    #[prop(into, optional)] variant: OptionalMaybeSignal<DrawerTheme>,
    children: Children,
) -> impl IntoView{
    
    let theme = variant.or_else( || use_context::<AppTheme>().unwrap_or_default().drawer);

    // TODO: check is there away to get away from not cloning the 
    let drawer_wrapper = {
        let cloned = theme.clone();
        move || with!(|is_open, cloned| {
            let dim = if *is_open { cloned.maximized.clone() } else { cloned.minimized.clone() };
            format!("{} {}", cloned.wrapper.clone(), dim)
        })
    };

    let drawer_inner = {
        let cloned = theme.clone();
        move || with!(|cloned| cloned.inner.clone())
        // move || theme.clone().with(|x| x.inner.clone())
    };

    let overlay_variant = {
        let cloned = theme.clone();
        move || with!(|cloned| cloned.overlay_theme.clone().unwrap_or_else(|| use_context::<AppTheme>().unwrap_or_default().overlay))
    };
    let overlay_variant = Signal::derive(overlay_variant);

    
    view! {
        <>
            <div
                class=drawer_wrapper 
                on:click=move |e| { e.stop_propagation() }
            >
                <div class=drawer_inner>{children()}</div>
            </div>
            <Show
                when=is_open
                fallback=move || {
                    view! { <></> }
                }
            > 
                <Overlay 
                    on_click=move || set_open.set(false) 
                    variant={overlay_variant} 
                >
                    <div></div>
                </Overlay>
            </Show>
        </>
    }
}