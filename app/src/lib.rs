use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use leptos_icons::*; 
use leptail::*;

pub mod leptail_doc;
use crate::leptail_doc::doc_routes::DocRoutes;
use crate::leptail_doc::themes::gradiance::gradiance_routes::GradianceRoutes;
use crate::leptail_doc::themes::moonlight::moonlight_routes::MoonlightRoutes;
pub mod error_template;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc. 
    provide_meta_context();

    view! { 
        <Stylesheet id="leptos" href="/pkg/start-axum-workspace.css"/>  
        
        <Router>
            <main>
                <Routes>
                    <Route path="" view=|| { view! {  <EmptyLayout/> } } >
                        <Route path="" view=|| { view! {  <AppLayout/> } } >
                            <Route path="" view=|| { view! {  <HomePage/> } } />
                        </Route>
                        <DocRoutes path="doc"/>
                        <GradianceRoutes path="theme/gradiance" />
                        <MoonlightRoutes path="theme/moonlight" />
                    </Route>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn EmptyLayout() -> impl IntoView {

    view! { 
        <div>
            <Outlet/>
        </div>
    }
}

/// Renders the home page of your application.
#[component]
fn AppLayout() -> impl IntoView {

    provide_context(leptail_theme_gradiance::build_theme());
    // provide_context(leptail_theme_moonlight::build_theme());
    let theme = use_context::<AppTheme>().unwrap_or_default();
    

    let mobile_menu = move || view! {  <div>"Mobile Menu "</div> };
    // let mobile_menu = || view! {  <div>"Mobile Menu " </div> };
    let logo = || view! {  <div ><A href="" class="text-bold text-xl" >"Leptail"</A></div> };
    let main_menu = || view! {  <div>" Main Menu "</div> };
    let right_menu = || view! {  <div>" Right Menu "</div> };

    view! { 
        <Body class=theme.body/>
        <Appbar
            hamburger_icon=icondata::ChMenuHamburger
            close_icon=icondata::CgClose
            logo=logo
            main_menu=main_menu
            right_menu=right_menu
            mobile_menu=mobile_menu
        />
        <div>
            <Outlet/>
        </div>
    }
}


/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {

    view! { 
        <Title text="Leptail: Headless with default styled component library for Leptos"/>
        <div class="flex flex-col content-center gap-5 my-10">
            <h1>"Welcome to Leptail!"</h1>
        </div>
        <div class="flex content-center justify-center">
            <A href=DocRoutes::Overview class="text-blue-800 dark:text-blue-400">
                "Documentation"
            </A>
        </div>
    }
}
