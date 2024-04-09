use leptos::*;
use leptos_meta::*;

use crate::leptail_doc::pages::ThemeExampleFrame;
use crate::leptail_doc::themes::moonlight::moonlight_routes::MoonlightRoutes;
use crate::leptail_doc::themes::gradiance::gradiance_routes::GradianceRoutes; 


#[component]
pub fn PageAppbar() -> impl IntoView {
    view! {
        <Title text="Leptail: Drawer"/>
        <h1 class="text-2xl">"Appbar"</h1>

        <div class="flex flex-col gap-4">
            <ThemeExampleFrame src=GradianceRoutes::Appbar.as_href() heading="Gradiance Examples"/>
            // <ThemeExampleFrame src=MoonlightRoutes::Appbar.as_href() heading="Moonlight Examples"/>
        </div>
    }
}
