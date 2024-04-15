use leptos::*;
use crate::prelude::*;

// TODO: drawer theme is part of the AppbarTheme
#[derive(Debug, Clone)]
pub struct AppbarTheme {
    pub layout: String,
    pub appbar_container:  String,
    pub appbar_inner:  String, 
    pub toolbar: String,
    pub hamburger_icon: icondata::Icon, 
    pub close_icon: icondata::Icon,  
    pub hamburger_container:  String,
    pub hamburger_button: String, // TODO: once the button theme is defined, refactor to use the button theme instead.  
    pub drawer_title_wrapper: String,
    pub drawer_title: String,
    pub drawer_container: String, 
    pub main_content: String,
    pub drawer_variant: DrawerTheme,
}

impl Default for AppbarTheme {
    fn default() -> Self {
        Self { 
            layout: Default::default(), 
            appbar_container: Default::default(), 
            appbar_inner: Default::default(), 
            hamburger_icon: icondata::ChMenuHamburger,
            close_icon: icondata::CgClose,  
            hamburger_container: Default::default(), 
            hamburger_button: Default::default(), 
            toolbar: Default::default(), 
            drawer_title_wrapper: Default::default(), 
            drawer_title: Default::default(), 
            drawer_container: Default::default(), 
            main_content: Default::default(),
            drawer_variant: Default::default(),
        }
    }
}


#[component]
pub fn Appbar<TC, DT, DC, IV>(  
    /// state to control if the drawer is open or not
    #[prop(into)] is_open: MaybeSignal<bool>,
    /// state to control if when backdrop is clicked on the drawer
    #[prop(into)] set_open: Out<bool>, 
    /// optional appbar theme variant
    #[prop(into, optional)] variant: OptionalMaybeSignal<AppbarTheme>, 
    /// Toolbar view
    toolbar_content: TC, 
    /// Drawer title view 
    drawer_title: DT,
    /// Drawer content view 
    drawer_content: DC,
    /// Main content
    children: Children,
) -> impl IntoView
where 
    TC: Fn() -> IV + 'static,
    DT: Fn() -> IV + 'static, 
    DC: Fn() -> IV + 'static, 
    IV: IntoView,
{
    let theme = variant.or_else(move || use_context::<AppTheme>().unwrap_or_default().appbar);
    
    // TODO: checkout https://leptos-rs.github.io/leptos/interlude_projecting_children.html see if it helps
    // test
    
    let hamburger_btn_class =  { 
        let cloned = theme.clone();
        move || with!(|cloned| cloned.hamburger_button.clone())
    };
    let close_btn_class = { 
        let cloned = theme.clone();
        move || with!(|cloned| cloned.hamburger_button.clone())
    };

    let hamburger_icon = Signal::derive({ 
        let cloned = theme.clone();
        move || with!(|cloned| cloned.hamburger_icon)
    });

    let close_icon = Signal::derive({ 
        let cloned = theme.clone();
        move || with!(|cloned| cloned.close_icon)
    });

    let hamburger_btn = view! {
        <button
            type="button"
            // TODO: add modifier class when the drawer state is changed.
            class=hamburger_btn_class
            aria-controls="mobile-menu"
            aria-expanded="false"
            on:click=move |_| {
                set_open.set(true);
            }
        >

            <span class="sr-only">"Open main menu"</span>
            <Icon icon=hamburger_icon/>
        </button>
    };

    let close_btn = view! {
        <button
            type="button"
            class=close_btn_class
            aria-controls="mobile-menu-close"
            aria-expanded="false"
            on:click=move |_| {
                set_open.set(false);
            }
        >

            <span class="sr-only">"Close main menu"</span>
            <Icon icon=close_icon/>
        </button>
    };

    let layout =  { 
        let cloned = theme.clone();
        move || with!(|cloned| cloned.layout.clone())
    };
    let appbar_container =  { 
        let cloned = theme.clone();
        move || with!(|cloned| cloned.appbar_container.clone())
    };
    let appbar_inner =  { 
        let cloned = theme.clone();
        move || with!(|cloned| cloned.appbar_inner.clone())
    };
    let hamburger_container =  { 
        let cloned = theme.clone();
        move || with!(|cloned| cloned.hamburger_container.clone())
    };
    let toolbar =  { 
        let cloned = theme.clone();
        move || with!(|cloned| cloned.toolbar.clone())
    };

    let drawer_container =  { 
        let cloned = theme.clone();
        move || with!(|cloned| cloned.drawer_container.clone())
    };
    let drawer_variant =  Signal::derive({ 
        let cloned = theme.clone();
        move || with!(|cloned| cloned.drawer_variant.clone())
    });
    let drawer_title_wrapper =  { 
        let cloned = theme.clone();
        move || with!(|cloned| cloned.drawer_title_wrapper.clone())
    };
    let drawer_title_class =  { 
        let cloned = theme.clone();
        move || with!(|cloned| cloned.drawer_title.clone())
    };
    let main_content =  { 
        let cloned = theme.clone();
        move || with!(|cloned| cloned.main_content.clone())
    };

    view! {
        <div class=layout>
            <div class=appbar_container>
                <div class=appbar_inner>
                    <div class=hamburger_container>{hamburger_btn.clone()}</div>
                    <div class=toolbar>{toolbar_content()}</div>
                </div>
            </div>
            <div class=drawer_container>
                <Drawer is_open=is_open set_open=set_open variant=drawer_variant>
                    <div class=drawer_title_wrapper>
                        <div class=drawer_title_class>{drawer_title()}</div>
                        {close_btn}
                    </div>
                    {drawer_content()}
                </Drawer>
                <div class=main_content>{children()}</div>
            </div>

        </div>
    }
}