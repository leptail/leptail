use leptos::*;
use crate::prelude::*;

// TODO: drawer theme is part of the AppbarTheme
#[derive(Debug, Clone, Default)]
pub struct AppbarTheme {
    pub layout: String,
    pub appbar_container:  String,
    pub appbar_inner:  String, 
    pub hamburger_container:  String,
    pub hamburger_button: String, // TODO: once the button theme is defined, refactor to use the button theme instead.  
    pub toolbar: String,
}


#[component]
pub fn Appbar<TC, DC, IV>( 
    hamburger_icon: icondata::Icon,
    close_icon: icondata::Icon,  
    toolbar_content: TC, 
    drawer_content: DC,
    children: Children,
    //TODO: move the drawer variant to appbar theme.
    drawer_variant: DrawerTheme,
    // TODO: add appbar theme here... 
) -> impl IntoView
where 
    TC: Fn() -> IV + 'static,
    DC: Fn() -> IV + 'static, 
    IV: IntoView,
{
    let theme = use_context::<AppTheme>().unwrap_or_default().appbar;
    // let theme = theme.appbar;
    
    let (is_drawer_open, set_drawer_open) = create_signal(false);
    // TODO: checkout https://leptos-rs.github.io/leptos/interlude_projecting_children.html see if it helps

    view! { 
        <div class=theme.layout>
            <div class=theme.appbar_container>
                <div class=theme.appbar_inner>
                    <div class=theme.hamburger_container>
                        <Show
                            when=move || { !is_drawer_open() }
                            fallback=|| {
                                view! { <></> }
                            }
                        >
                            <button
                                type="button"
                                // TODO: check why is it being moved if this is uncommeneted here
                                // class={theme.hamburger_button.clone()}
                                aria-controls="mobile-menu"
                                aria-expanded="false"
                                on:click=move |_| {
                                    set_drawer_open(true);
                                }
                            >
                                <span class="sr-only">"Open main menu"</span>
                                <Icon icon=hamburger_icon/>
                            </button>
                        </Show>
                    </div>
                    <div class=theme.toolbar>
                        {toolbar_content()}
                    </div>
                </div>
            </div>
            //TOOD: do we need one more theme class here???
            <div>
                <Drawer is_open=is_drawer_open set_open=set_drawer_open variant=drawer_variant >
                    <>
                        // TODO: How should the drawer title and close button be created? whoes responsibilities is it?
                        // TODO: move the following class to appbar theme
                        <div class="md:hidden" >
                            <button
                                type="button"
                                class=theme.hamburger_button
                                aria-controls="mobile-menu-close"
                                aria-expanded="false"
                                on:click=move |_| {
                                    set_drawer_open(false);
                                }
                            >
                                <span class="sr-only">"Close main menu"</span>
                                <Icon icon=close_icon/>
                            </button>
                        </div>
                        {drawer_content()}
                    </>
                </Drawer>
                // TODO: do we need one more theme class here? 
                <div>
                    {children()}
                </div>
            </div>
            
        </div>
    }
}