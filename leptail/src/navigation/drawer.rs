use leptos::*;
use crate::prelude::*;


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
    #[prop(into, optional)] variant: Option<DrawerTheme>,
    children: Children,
) -> impl IntoView{
    
    let theme = variant.unwrap_or_else(move || use_context::<AppTheme>().unwrap_or_default().drawer);
    let dimension_class = move || if is_open() { theme.maximized.clone() } else { theme.minimized.clone() };
    // let show_overlay = move || is_open() && theme.has_overlay;
    // let overlay_variant = theme.overlay_theme;

    view! {
        <>
            <div
                class=move || format!("{} {}", theme.wrapper, dimension_class())
                on:click=move |e| { e.stop_propagation() }
            >
                <div class=theme.inner>{children()}</div>
            </div>
            <Show
                when=is_open
                fallback=move || {
                    view! { <></> }
                }
            >
                <Overlay on_click=move || set_open.set(false) variant=theme.overlay_theme.clone()>
                    <div></div>
                </Overlay>
            </Show>
        </>
    }
}