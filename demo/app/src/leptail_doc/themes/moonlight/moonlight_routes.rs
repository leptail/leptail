use leptail::prelude::*;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use std::fmt::Display;

use crate::leptail_doc::themes::moonlight::pages::{
    drawer_doc::PageDrawer, switch_doc::PageSwitch,
};

#[derive(Debug, Copy, Clone)]
pub enum MoonlightRoutes {
    // Layout
    // Stack,
    // Grid,
    // Separator,
    // Skeleton,
    Appbar,
    Drawer,
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

impl MoonlightRoutes {
    pub fn route(self) -> &'static str {
        match self {
            // MoonlightRoutes::Stack => "stack",
            // MoonlightRoutes::Grid => "grid",
            // MoonlightRoutes::Separator => "separator",
            // MoonlightRoutes::Skeleton => "skeleton",
            MoonlightRoutes::Appbar => "app-bar",
            MoonlightRoutes::Drawer => "drawer",
            // MoonlightRoutes::Tab => "tabs",
            // MoonlightRoutes::Table => "table",
            // MoonlightRoutes::Collapsible => "collapsible",

            // MoonlightRoutes::Button => "button",
            // MoonlightRoutes::Input => "input",
            // MoonlightRoutes::TiptapEditor => "tiptap-editor",
            // MoonlightRoutes::DateTime => "date-time",
            // MoonlightRoutes::Slider => "slider",
            // MoonlightRoutes::Select => "select",
            MoonlightRoutes::Switch => "switch",
            // MoonlightRoutes::ColorPicker => "color-picker",

            // MoonlightRoutes::Alert => "alert",
            // MoonlightRoutes::Toast => "toast",
            // MoonlightRoutes::Modal => "modal",
            // MoonlightRoutes::Progress => "progress",
            // MoonlightRoutes::Popover => "popover",
            // MoonlightRoutes::Chip => "chip",
            // MoonlightRoutes::Kbd => "kbd",

            // MoonlightRoutes::Typography => "typography",
            // MoonlightRoutes::Icon => "icon",
            // MoonlightRoutes::Link => "link",
            // MoonlightRoutes::Anchor => "anchor",
            // MoonlightRoutes::Callback => "callback",

            // MoonlightRoutes::Transition => "transition",
            // MoonlightRoutes::NotFound => "*", // Leptos requires this to be be named "*"!
        }
    }

    pub fn as_href(&self) -> String {
        format!("/{}/{}", "theme/moonlight", self.route())
    }
}

impl Display for MoonlightRoutes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.route())
    }
}

/// Required so that `Routes` variants can be used in `<Link href=Routes::Foo ...>` definitions.
impl ToHref for MoonlightRoutes {
    fn to_href(&self) -> Box<dyn Fn() -> String + '_> {
        Box::new(move || format!("/{}/{}", "theme/moonlight", self.route()))
    }
}

#[component(transparent)]
pub fn MoonlightRoutes<P>(path: P) -> impl IntoView
where
    P: std::fmt::Display,
{
    view! {
        <Route path=path view=|| view! { <MoonlightLayout/> }>

            // <Route path=MoonlightRoutes::Appbar view=|| { view! {  <PageAppbar/> } } />
            <Route path=MoonlightRoutes::Drawer view=|| view! { <PageDrawer/> }/>

            <Route path=MoonlightRoutes::Switch view=|| view! { <PageSwitch/> }/>

        </Route>
    }
}

#[component]
pub fn MoonlightLayout() -> impl IntoView {
    provide_context(leptail_theme_moonlight::OverlayVariant::default_variant());
    provide_context(leptail_theme_moonlight::DrawerVariant::default_variant());
    // provide_context(leptail_theme_moonlight::build_theme());
    // provide_context(leptail_theme_moonlight::build_theme());
    // let theme = use_context::<AppTheme>().unwrap_or_default();

    view! {
        <Body class=leptail_theme_moonlight::BodyTheme::default_variant().get_body().to_string()/>
        <div class="mx-auto max-w-screen-xl">
            <div class="m-4">
                <Outlet/>
            </div>
        </div>
    }
}
