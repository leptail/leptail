pub mod errors;
pub mod pages;

use crate::errors::AppError;
use crate::pages::error_template::ErrorTemplate;
use crate::pages::home::Home;

use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{
    components::{FlatRoutes, Route, Router, Routes},
    StaticSegment,
};
use pages::error_example::ExampleErrors;

stylance::import_style!(lib_styles, "lib.module.scss");

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone()/>
                <HydrationScripts options/>
                <link rel="stylesheet" id="leptos" href="/pkg/start-axum-workspace.css"/>
                <link rel="shortcut icon" type="image/ico" href="/favicon.ico"/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Router>
            <nav class=lib_styles::nav>
                <div class=lib_styles::logo>
                    <a href="/">"Leptail"</a>
                </div>
                <div class=lib_styles::menu>
                    <ul>
                        <li>
                            <a href="/error">Err!</a>
                        </li>
                    </ul>
                </div>
            </nav>
            <main class=lib_styles::main>
                <div class="">
                    <Routes fallback=|| {
                        let mut errors = Errors::default();
                        errors.insert_with_default_key(AppError::NotFound);
                        view! { <ErrorTemplate errors/> }.into_view()
                    }>
                        // <Route path=StaticSegment("") view=|| view! { <AppLayout/> }>
                        <Route path=StaticSegment("") view=Home/>
                        <Route path=StaticSegment("/error") view=ExampleErrors/>
                    // </Route>
                    </Routes>

                </div>
            </main>
        </Router>
    }
}

// #[component]
// pub fn AppLayout() -> impl IntoView {
//     view! {
//         <main>
//             <div class="">
//                 <div class="">
//                     <Outlet/>
//                 </div>
//             </div>
//         </main>
//     }
// }
