use std::fmt::Display;
use leptos::*;
use leptos_router::*;
use leptail::prelude::*;
use leptos_meta::*;

use crate::leptail_doc::themes::gradiance::pages::{drawer_doc::*, switch_doc::PageSwitch, appbar_doc::PageAppbar};

#[derive(Debug, Copy, Clone)]
pub enum GradianceRoutes { 

    // Layout
    // Stack,
    // Grid,
    // Separator,
    // Skeleton,
    Appbar,
    Drawer,
    DrawerResponsive,
    DrawerStaggered,
    DrawerStaggeredMini,
    // Tab,
    // Table,
    // Collapsible,

    // Input
    // Button,
    // Input, 
    // DateTime,
    // Slider,
    // Select,
    Switch,
    // ColorPicker,

    // Feedback
    // Alert,
    // Toast,
    // Modal,
    // Progress,
    // Popover,
    // Chip,
    // Kbd,

    // General
    // Typography,
    // Icon,
    // Link,
    // Anchor,
    // Callback,

    // Animation
    // Transition,

    // Technical
    // NotFound,
}

impl GradianceRoutes {
    pub fn route(self) -> &'static str {
        match self { 

            // GradianceRoutes::Stack => "stack",
            // GradianceRoutes::Grid => "grid",
            // GradianceRoutes::Separator => "separator",
            // GradianceRoutes::Skeleton => "skeleton",
            GradianceRoutes::Appbar => "app-bar",
            GradianceRoutes::Drawer => "drawer",
            GradianceRoutes::DrawerResponsive => "drawer-responsive",
            GradianceRoutes::DrawerStaggered => "drawer-staggered",
            GradianceRoutes::DrawerStaggeredMini => "drawer-staggered-mini",
            // GradianceRoutes::Tab => "tabs",
            // GradianceRoutes::Table => "table",
            // GradianceRoutes::Collapsible => "collapsible",

            // GradianceRoutes::Button => "button",
            // GradianceRoutes::Input => "input",
            // GradianceRoutes::TiptapEditor => "tiptap-editor",
            // GradianceRoutes::DateTime => "date-time",
            // GradianceRoutes::Slider => "slider",
            // GradianceRoutes::Select => "select",
            GradianceRoutes::Switch => "switch",
            // GradianceRoutes::ColorPicker => "color-picker",

            // GradianceRoutes::Alert => "alert",
            // GradianceRoutes::Toast => "toast",
            // GradianceRoutes::Modal => "modal",
            // GradianceRoutes::Progress => "progress",
            // GradianceRoutes::Popover => "popover",
            // GradianceRoutes::Chip => "chip",
            // GradianceRoutes::Kbd => "kbd",

            // GradianceRoutes::Typography => "typography",
            // GradianceRoutes::Icon => "icon",
            // GradianceRoutes::Link => "link",
            // GradianceRoutes::Anchor => "anchor",
            // GradianceRoutes::Callback => "callback",

            // GradianceRoutes::Transition => "transition",
            // GradianceRoutes::NotFound => "*", // Leptos requires this to be be named "*"!
        }
    }

    pub fn as_href(&self) -> String {
        format!("/{}/{}", "theme/gradiance", self.route())
    }

}

impl Display for GradianceRoutes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.route())
    }
}

/// Required so that `Routes` variants can be used in `<Link href=Routes::Foo ...>` definitions.
impl ToHref for GradianceRoutes {
    fn to_href(&self) -> Box<dyn Fn() -> String + '_> {
        Box::new(move || format!("/{}/{}", "theme/gradiance", self.route()))
    }   
}

#[component(transparent)]
pub fn GradianceRoutes<P>(path: P) -> impl IntoView
where
    P: std::fmt::Display,
{
    view! {
        <Route
            path=path
            view=|| {
                view! { <GradianceLayout/> }
            }
        >

            <Route path=GradianceRoutes::Appbar view=|| { view! {  <PageAppbar/> } } />
            <Route
                path=GradianceRoutes::Drawer
                view=|| {
                    view! { <PageDrawer/> }
                }
            />
            <Route
                path=GradianceRoutes::DrawerResponsive
                view=|| {
                    view! { <PageDrawerResponsive/> }
                }
            />
            <Route
                path=GradianceRoutes::DrawerStaggered
                view=|| {
                    view! { <PageDrawerStaggered/> }
                }
            />
            <Route
                path=GradianceRoutes::DrawerStaggeredMini
                view=|| {
                    view! { <PageDrawerStaggeredMini/> }
                }
            />
            <Route
                path=GradianceRoutes::Switch
                view=|| {
                    view! { <PageSwitch/> }
                }
            />
        </Route>
    }
}



#[component]
pub fn GradianceLayout() -> impl IntoView {

    provide_context(leptail_theme_gradiance::build_theme());
    // provide_context(leptail_theme_moonlight::build_theme());
    let theme = use_context::<AppTheme>().unwrap_or_default();
     
    view! {
        <Body class=theme.body/>
        <div class="mx-auto">
            <div class="">
                <Outlet/>
            </div>
        </div>
    }
}