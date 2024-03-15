use leptos::*;
use crate::prelude::*;


#[derive(Debug, Clone, Default)]
pub struct DrawerTheme {
    pub base: String,
    pub minimized: String,
    pub maximized: String,
    pub has_overlay: bool,
}


// Drawer Variants and Funtions
// -----------------------------
// 1. Drawer Sides -- Theme 
// 2. Inset drawers -- Theme (Not attached to screen like MUI(joy) drawers variants)
// 3. Resposive drawers -- Theme (Closed on mobile and open on large screen; seen on on MUI drawer)
// 4. Persistent Drawers (Body shifts when drawer opens; seen on on MUI drawer)
// 5. Drawer size -- Theme (xs, sm, md, lg, xl, 2xl, full etc)
// 6. Focus on specific element -- Prop (Seen on chakra UI)
// 7. Disable click on the backdrop to close -- Prop  
// 8. Close on escape key press -- Prop 
// 9. Half minimized drawer, which can also be persistent -- (MUI largescreen )


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
    let show_overlay = move || is_open() && theme.has_overlay;

    view! { 
        <div>
            <div
                class=move || format!("{} {}", theme.base, dimension_class())
                on:click=move |e| { e.stop_propagation() }
            >
                {children()}
            </div>
            <Show
                when={show_overlay}
                fallback=move || { view! {  <></> } }
            >
                <Overlay on_click=move || set_open.set(false) >
                    <div></div>
                </Overlay>
            </Show> 
        </div>
    }
}