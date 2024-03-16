use leptos::*;
use crate::prelude::*;

// TODO: drawer theme is part of the AppbarTheme
#[derive(Debug, Clone, Default)]
pub struct AppbarTheme {
    pub nav: String,
    pub nav_inner:  &'static str,
    pub lg_menu_container:  &'static str,
    pub sm_menu_container:  &'static str,
    pub hamburger_container:  &'static str,
    pub hamburger_button: &'static str,
    pub left_side_container:  &'static str,
    pub right_side_container: &'static str,
    pub logo: &'static str,
    pub menu: &'static str,
}


#[component]
pub fn Appbar<L, LM, SM, RM, IV>( 
    hamburger_icon: icondata::Icon,
    close_icon: icondata::Icon,
    logo: L,
    main_menu: LM,
    right_menu: RM,
    mobile_menu: SM,
) -> impl IntoView
where
    L: Fn() -> IV + 'static,
    LM: Fn() -> IV + 'static,
    SM: Fn() -> IV + 'static,
    RM: Fn() -> IV + 'static,
    IV: IntoView,
{
    let theme = use_context::<AppTheme>().unwrap_or_default();
    let theme = theme.appbar;
    
    let (is_drawer_open, set_drawer_open) = create_signal(false);
    // TODO: checkout https://leptos-rs.github.io/leptos/interlude_projecting_children.html see if it helps

    view! { 
        <nav class=theme.nav>
            <div class=theme.nav_inner>
                <div class=theme.lg_menu_container>
                    <div class=theme.hamburger_container>
                        <Show
                            when=move || { !is_drawer_open() }
                            fallback=|| {
                                view! { <></> }
                            }
                        >
                            <button
                                type="button"
                                class=theme.hamburger_button
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
                    <div class=theme.left_side_container>
                        <div class=theme.logo>{logo()}</div> 
                        <div class="hidden sm:ml-6 sm:block">
                            <div class=theme.menu>{main_menu()}</div>
                        </div>
                    </div>
                    <div class=theme.right_side_container>{right_menu()}</div>
                </div>
            </div>
            <Drawer is_open=is_drawer_open set_open=set_drawer_open >
                <>
                    // TODO: provide theming option for close menu on drawer
                    <div>
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
                    {mobile_menu()}
                </>
            </Drawer>
        </nav>
    }
}