use leptail::prelude::*;
use leptail_theme_gradiance::gradiance::navigation::drawer_theme::DrawerVariant;
use leptail_theme_gradiance::gradiance::surfaces::appbar_theme::AppbarVariant;
use leptail_theme_gradiance::gradiance::HorizontalSide;
use leptail_theme_gradiance::Size;
use leptos_icons::*; 
use leptos_meta::*;
use leptos_router::*;
use leptos::*;
use std::fmt::Display;

use crate::leptail_doc::pages::appbar::PageAppbar;
use crate::leptail_doc::pages::drawer_doc::PageDrawer;
use crate::leptail_doc::pages::overview::PageOverview;
use crate::leptail_doc::pages::switch_doc::PageSwitch;

#[derive(Debug, Copy, Clone)]
pub enum DocRoutes {
    // Getting started
    Overview,
    // Installation,
    // Usage,
    // Themes,
    // Changelog,

    // Layout
    // Stack,
    // Grid,
    // Separator,
    // Skeleton,
    AppBar,
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

impl DocRoutes {
    pub fn route(self) -> &'static str {
        match self {
            DocRoutes::Overview => "overview",
            // DocRoutes::Installation => "installation",
            // DocRoutes::Usage => "usage",
            // DocRoutes::Themes => "themes",
            // DocRoutes::Changelog => "changelog",

            // DocRoutes::Stack => "stack",
            // DocRoutes::Grid => "grid",
            // DocRoutes::Separator => "separator",
            // DocRoutes::Skeleton => "skeleton",
            DocRoutes::AppBar => "app-bar",
            DocRoutes::Drawer => "drawer",
            // DocRoutes::Tab => "tabs",
            // DocRoutes::Table => "table",
            // DocRoutes::Collapsible => "collapsible",

            // DocRoutes::Button => "button",
            // DocRoutes::Input => "input",
            // DocRoutes::TiptapEditor => "tiptap-editor",
            // DocRoutes::DateTime => "date-time",
            // DocRoutes::Slider => "slider",
            // DocRoutes::Select => "select",
            DocRoutes::Switch => "switch",
            // DocRoutes::ColorPicker => "color-picker",

            // DocRoutes::Alert => "alert",
            // DocRoutes::Toast => "toast",
            // DocRoutes::Modal => "modal",
            // DocRoutes::Progress => "progress",
            // DocRoutes::Popover => "popover",
            // DocRoutes::Chip => "chip",
            // DocRoutes::Kbd => "kbd",

            // DocRoutes::Typography => "typography",
            // DocRoutes::Icon => "icon",
            // DocRoutes::Link => "link",
            // DocRoutes::Anchor => "anchor",
            // DocRoutes::Callback => "callback",

            // DocRoutes::Transition => "transition",
            // DocRoutes::NotFound => "*", // Leptos requires this to be be named "*"!
        }
    }
}

impl Display for DocRoutes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.route())
    }
}

/// Required so that `Routes` variants can be used in `<Link href=Routes::Foo ...>` definitions.
impl ToHref for DocRoutes {
    fn to_href(&self) -> Box<dyn Fn() -> String + '_> {
        Box::new(move || format!("/{}/{}", "doc", self.route()))
    }
}

#[component(transparent)]
pub fn DocRoutes<P>(path: P) -> impl IntoView
where
    P: std::fmt::Display,
{
    view! {
        <Route
            path=path
            view=|| {
                view! { <DocLayout/> }
            }
        >

            <Route path="" view=|| view! { <Redirect path=DocRoutes::Overview/> }/>
            <Route
                path=DocRoutes::Overview
                view=|| {
                    view! { <PageOverview/> }
                }
            />
            <Route
                path=DocRoutes::AppBar
                view=|| {
                    view! { <PageAppbar/> }
                }
            />
            <Route
                path=DocRoutes::Drawer
                view=|| {
                    view! { <PageDrawer/> }
                }
            />
            <Route
                path=DocRoutes::Switch
                view=|| {
                    view! { <PageSwitch/> }
                }
            />
        </Route>
    }
}



#[component]
pub fn DocLayout() -> impl IntoView {

    provide_context(leptail_theme_gradiance::build_theme());
    // provide_context(leptail_theme_moonlight::build_theme());
    let theme = use_context::<AppTheme>().unwrap_or_default();

    let nav_link_class = "px-4 py-2 text-left flex w-full items-start hover:bg-purple-100 hover:dark:bg-purple-900 no-underline hover:no-underline transition-colors duration-100 cursor-pointer";
    let links = move || view! {
        <ul class="space-y-1 list-reset">
            <li>
                <A href=DocRoutes::Overview class=nav_link_class >"Overview"</A>
            </li>
            <li>
                <A href=DocRoutes::AppBar class=nav_link_class >"Appbar"</A>
            </li>
            <li>
                <A href=DocRoutes::Drawer class=nav_link_class >"Drawer"</A>
            </li>
            <li>
                <A href=DocRoutes::Switch class=nav_link_class >"Switch"</A>
            </li>
        </ul>
    };

    let drawer_content = move || view! { <div class="">{links()}</div> };
    let toolbar_content = || view! { 
        <div class="flex flex-row items-end space-x-4 self-center" >
            <div>
                <A href="/" class="font-bold text-2xl">"Leptail"</A>
            </div> 
        </div> 
    }; 


    let (is_drawer_open, set_drawer_open) = create_signal(false);
     
    view! {
        <Body class=theme.body/>
        <Appbar
            is_open=is_drawer_open
            set_open=set_drawer_open
            toolbar_content=toolbar_content
            drawer_title=|| view! { <div></div> }
            drawer_content=drawer_content
            variant={AppbarVariant::builder()
                .sticky(true)
                .shadow(Size::Medium)
                // .bg_color(Color::Primary)
                // .max_width(Size::Large)
                .drawer_variant(DrawerVariant::Responsive { side: HorizontalSide::Left })
                .build()
            }
        >
            <div class="mx-auto">
                <div class="flex gap-4 mx-4">
                    <div class="flex-none w-80 text-left hidden md:inline-block ">// {links()}
                    </div>
                    <div class="flex-initial w-full">
                        <Outlet/>
                    </div>
                </div>

            </div>
        </Appbar>
    }
}