use leptos::*;
use crate::prelude::*;

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
    #[prop(into, optional)] overlay_theme: Option<OverlayTheme>,
    children: Children,
) -> impl IntoView
where
    S: Fn() + 'static,
{
    
    let theme = overlay_theme.unwrap_or_else(move || use_context::<AppTheme>().unwrap_or_default().overlay);
    // let theme = use_context::<AppTheme>().unwrap_or_default();
    // let theme = theme.overlay;

    view! { 
        <div class=theme.wrapper>
            <div
                class=theme.inner
                on:click=move |_| (on_click)()
            >
                {children()}
            </div>
        </div>
    }
}