use leptos::*;
use crate::prelude::*;

#[derive(Debug, Clone, Default)]
pub struct OverlayTheme {
    //TODO: add container theme here...
    pub overlay:  &'static str,
}



#[component]
pub fn Overlay<S>( 
    /// callback when clicked on the backdrop 
    on_click: S,
    children: Children,
) -> impl IntoView
where
    S: Fn() + 'static,
{
    
    let theme = use_context::<AppTheme>().unwrap_or_default();
    let theme = theme.overlay;

    view! { 
        <div class="relative">
            <div
                class=theme.overlay
                on:click=move |_| (on_click)()
            >
                {children()}
            </div>
        </div>
    }
}