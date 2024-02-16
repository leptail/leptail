use leptos::*;
use crate::prelude::*;


#[derive(Debug, Clone, Default)]
pub struct DrawerSideTheme {
    pub side: String,
    pub minimized: String,
    pub maximized: String,
}

#[derive(Debug, Clone, Default)]
pub struct DrawerTheme {
    pub container: String,
    pub left_side: DrawerSideTheme,
    pub right_side: DrawerSideTheme,
    pub top_side: DrawerSideTheme,
    pub bottom_side: DrawerSideTheme,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum DrawerSide {
    #[default]
    Left,
    Right,
    Top,
    Bottom,
}

impl DrawerSide {
    pub fn class_name(self, theme: &DrawerTheme) -> DrawerSideTheme {
        match self {
            DrawerSide::Left => theme.left_side.clone(),
            DrawerSide::Right => theme.right_side.clone(),
            DrawerSide::Top => theme.top_side.clone(),
            DrawerSide::Bottom => theme.bottom_side.clone(),
        }
    }
}

#[component]
pub fn Drawer( 
    /// state to control if the drawer is open or not
    #[prop(into)] is_open: MaybeSignal<bool>,
    /// state to control if when backdrop is clicked on the drawer
    #[prop(into)] set_open: Out<bool>,
    /// which side the drawer is shown
    side: DrawerSide,
    children: Children,
) -> impl IntoView{
    
    let theme = use_context::<AppTheme>().unwrap_or_default();
    let theme = theme.drawer;
    let side_theme = side.class_name(&theme);
    let dimension_class = move || if is_open() { side_theme.maximized.clone() } else { side_theme.minimized.clone() };

    view! { 
        <div 
            // class:hidden=move || is_open() == false
            // class=move || if is_open() { "" } else { "hidden" }
            // class=move || if is_open() { "opacity-1" } else { "opacity-0" }
        >
            // <div
            //     class=move || format!("{} {} {}", theme.container, side_theme.side, dimension_class())
            //     on:click=move |e| { e.stop_propagation() }
            // >
            //     {children()}
            // </div>
            <div
                class=move || format!("{} {} {}", theme.container, side_theme.side, dimension_class())
                on:click=move |e| { e.stop_propagation() }
            >
                {children()}
            </div>
            <Show
                when=is_open
                fallback=move || { view! {  <div></div> } }
            >
                <Overlay on_click=move || set_open.set(false) >
                    <div></div>
                </Overlay>
            </Show>
            
        </div>
    }
}