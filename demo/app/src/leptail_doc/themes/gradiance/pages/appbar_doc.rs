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
    let (is_sticky, set_sticky) = create_signal(true);
    let (shadow, set_shadow) = create_signal::<Option<Size>>(Some(Size::Medium));
    let (bg_color, set_bg_color) = create_signal::<Option<Color>>(Some(Color::Primary));
    let (max_width, set_max_width) = create_signal::<Option<Size>>(Some(Size::Large));
    let (drawer_variant, set_drawer_variant) = create_signal(DrawerVariant::Staggered { breakover_point: Size::Large, side: HorizontalSide::Left });

    let appbar_variant = Signal::derive(move || with!(|is_sticky, shadow, bg_color, max_width, drawer_variant| {
        AppbarVariant::builder()
            .sticky(*is_sticky)
            .shadow(*shadow)
            .bg_color(*bg_color)
            .max_width(*max_width)
            .drawer_variant(*drawer_variant)
            .build()
    }));

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
        <div class="w-full flex flex-col space-y-2 mx-5 my-5" >
            <div class="flex flex-row" >
                <div class="mr-2 opacity-75" >Shadow: </div>
                <OptionalSizeButtons size=shadow set_size=set_shadow group_name="shadow" />
            </div>
            <div class="flex flex-row" >
                <div class="mr-2 opacity-75" >Max Width: </div>
                <OptionalSizeButtons size=max_width set_size=set_max_width group_name="max_width" />
            </div>
            <div class="flex flex-row" >
                <div class="mr-2 opacity-75" >Max Width: </div>
                <OptionalColorButtons color=bg_color set_color=set_bg_color group_name="bg_color" />
            </div>
            <div class="flex flex-row" >
                <input type="checkbox" 
                    checked=move || is_sticky()
                    on:click=move |_| set_sticky.update(|b| *b = !*b ) />
                <span class="ml-2" >"Is Sticky"</span>
            </div>
        </div>
        <div class="w-full">
            <Appbar
                is_open=is_drawer_open
                set_open=set_drawer_open
                toolbar_content=toolbar_content
                drawer_title=|| view! { <div></div> }
                drawer_content=drawer_content
                variant={appbar_variant}
            >
                <LoremIpsumLong/>
            </Appbar>
        </div>
    }
}
