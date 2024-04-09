use leptail::prelude::*;
use leptos::*;
use leptos_meta::*;
use leptail_theme_gradiance::*; 
use leptail_theme_gradiance::gradiance::navigation::drawer_theme::DrawerVariant;
use leptail_theme_gradiance::gradiance::surfaces::appbar_theme::AppbarVariant;
use leptail_theme_gradiance::gradiance::HorizontalSide;
use leptail_theme_gradiance::Size;


use crate::leptail_doc::themes::gradiance::{gradiance_routes::GradianceRoutes, pages::common_components::*};

#[component]
pub fn PageAppbar() -> impl IntoView { 

    let (is_drawer_open, set_drawer_open) = create_signal(false);

    let drawer_content = move || view! { <div class=""><NavMenuExample is_open=true/></div> };
    let toolbar_content = || view! { 
        <div class="flex flex-row items-end space-x-4 self-center" >
            <div>
                <div class="font-bold text-2xl">"Logo"</div>
            </div> 
        </div> 
    }; 


    
    

    view! {
        <Title text="Leptail: Gradiance Appbar Theme and Variants"/>

        <div class="w-full">
            <Appbar
                is_open=is_drawer_open
                set_open=set_drawer_open
                toolbar_content=toolbar_content
                drawer_title=|| view! { <div></div> }
                drawer_content=drawer_content
                variant={AppbarVariant::builder()
                    .sticky(true)
                    .shadow(Size::Medium)
                    .bg_color(Color::Primary)
                    .max_width(Size::Large)
                    // .drawer_variant(DrawerVariant::Responsive { side: HorizontalSide::Left })
                    .drawer_variant(DrawerVariant::Staggered { breakover_point: Size::Large, side: HorizontalSide::Left })
                    .build()
                }
            >
                <LoremIpsumLong/>
            </Appbar>
        </div>
    }
}
