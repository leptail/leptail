use std::borrow::Borrow;

use leptail::prelude::*;
use leptos::*;
use leptos_meta::*;
use leptail_theme_gradiance::{gradiance::{navigation::drawer_theme::DrawerVariant, HorizontalSide, Side}, *}; 
use crate::leptail_doc::themes::gradiance::{gradiance_routes::GradianceRoutes, pages::common_components::*};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct DrawerVariantState {
    state:  (ReadSignal<bool>, WriteSignal<bool>),
    variant_btn_text: &'static str,
    variant: DrawerVariant
}

#[component]
pub fn PageDrawer() -> impl IntoView { 

    let side_variants = vec![
        DrawerVariantState{ state: create_signal(false), variant_btn_text: "Open Left", variant: DrawerVariant::Temporary {  size: Size::Medium, side: Side::Left, has_inset: false } },
        DrawerVariantState{ state: create_signal(false), variant_btn_text: "Open Right", variant: DrawerVariant::Temporary {  size: Size::Medium, side: Side::Right, has_inset: true } },
        DrawerVariantState{ state: create_signal(false), variant_btn_text: "Open Top", variant: DrawerVariant::Temporary {  size: Size::Medium, side: Side::Top, has_inset: false } },
        DrawerVariantState{ state: create_signal(false), variant_btn_text: "Open Bottom", variant: DrawerVariant::Temporary {  size: Size::Medium, side: Side::Bottom, has_inset: true } }
    ];
    let (side_variants, _set_side_variants) = create_signal::<Vec<DrawerVariantState>>(side_variants);

    let size_variants = vec![
        DrawerVariantState{ state: create_signal(false), variant_btn_text: "Open XSmall", variant: DrawerVariant::Temporary {  size: Size::XSmall, side: Side::Left, has_inset: false } },
        DrawerVariantState{ state: create_signal(false), variant_btn_text: "Open Small", variant: DrawerVariant::Temporary {  size: Size::Small, side: Side::Left, has_inset: false } },
        DrawerVariantState{ state: create_signal(false), variant_btn_text: "Open Medium", variant: DrawerVariant::Temporary {  size: Size::Medium, side: Side::Left, has_inset: false } },
        DrawerVariantState{ state: create_signal(false), variant_btn_text: "Open Large", variant: DrawerVariant::Temporary {  size: Size::Large, side: Side::Left, has_inset: false } },
        DrawerVariantState{ state: create_signal(false), variant_btn_text: "Open XLarge", variant: DrawerVariant::Temporary {  size: Size::XLarge, side: Side::Left, has_inset: false } }
    ];
    let (size_variants, _set_side_variants) = create_signal::<Vec<DrawerVariantState>>(size_variants);
    

    view! { 
        <Title text="Leptail: Gradiance Drawer and Variants"/>

        <div class="" >
            <div class="my-16" >
                <h3 class="text-xl text-left mb-5" >"Drawer opening side"</h3>
                <div class="flex flex-col lg:flex-row gap-4 relative overflow-hidden" >
                    // buttons
                    <For
                        each=move || side_variants.get()
                        key=|side_variant| side_variant.variant_btn_text
                        let:dvs
                    >
                        <button 
                            class="bg-slate-400 dark:bg-slate-700 border border-slate-500 rounded-lg p-4"
                            on:click=move |_| dvs.state.1.borrow()(true)
                        >{dvs.variant_btn_text}</button>
                    </For>

                    // drawers 
                    <For
                        each=move || side_variants.get()
                        key=|side_variant| side_variant.variant_btn_text
                        let:dvs
                    >
                        <Drawer is_open=dvs.state.0 set_open=dvs.state.1 
                            variant=DrawerVariant::variant(&dvs.variant) > 
                            <div class="flex flex-row" >
                                <h1 class="text-2xl font-semibold" >"Drawer Title"</h1>
                                <button 
                                    class="bg-slate-400 dark:bg-slate-700 border border-slate-500 rounded-lg p-4"
                                    on:click=move |_| dvs.state.1(false)
                                >"X"</button>
                            </div>
                            <div class="mt-5" >
                                "Drawer content here..."
                            </div>
                        </Drawer>
                    </For>        
                </div>
            </div>
            
            <div class="my-16" >
                <h3 class="text-xl text-left mb-5" >"Drawer size"</h3>
                <div class="flex flex-col lg:flex-row gap-4" >
                    // buttons
                    <For
                        each=move || size_variants.get()
                        key=|side_variant| side_variant.variant_btn_text
                        let:dvs
                    >
                        <button 
                            class="bg-slate-400 dark:bg-slate-700 border border-slate-500 rounded-lg p-4"
                            on:click=move |_| dvs.state.1.borrow()(true)
                        >{dvs.variant_btn_text}</button>
                    </For>

                    // drawers 
                    <For
                        each=move || size_variants.get()
                        key=|side_variant| side_variant.variant_btn_text
                        let:dvs
                    >
                        <Drawer is_open=dvs.state.0 set_open=dvs.state.1 
                            variant=DrawerVariant::variant(&dvs.variant) > 
                            <div class="flex flex-row" >
                                <h1 class="text-2xl font-semibold" >"Drawer Title"</h1>
                                <button 
                                    class="bg-slate-400 dark:bg-slate-700 border border-slate-500 rounded-lg p-4"
                                    on:click=move |_| dvs.state.1(false)
                                >"X"</button>
                            </div>
                            <div class="mt-5" >
                                <NavMenuExample is_open=dvs.state.0 />
                            </div>
                        </Drawer>
                    </For>    
                </div>
            </div>

            <div class="my-16" >
                <h3 class="text-xl text-left mb-5" >"Responsive Drawer"</h3>
                <iframe class="w-full min-h-[25rem]" src=GradianceRoutes::DrawerResponsive.as_href() frameborder="0" height="100%"></iframe>  
            </div>

            <div class="my-16" >
                <h3 class="text-xl text-left mb-5" >"Staggered Drawer"</h3>
                <iframe class="w-full min-h-[25rem]" src=GradianceRoutes::DrawerStaggered.as_href() frameborder="0" height="100%"></iframe>  
            </div>

            <div class="my-16" >
                <h3 class="text-xl text-left mb-5" >"Staggered Mini Drawer"</h3>
                <iframe class="w-full min-h-[25rem]" src=GradianceRoutes::DrawerStaggeredMini.as_href() frameborder="0" height="100%"></iframe>  
            </div>
            
            
        </div>
        
    }
}


#[component]
pub fn PageDrawerResponsive() -> impl IntoView { 

    
    let (is_drawer_open, set_drawer_open) = create_signal(false);
    

    view! { 
        <Title text="Leptail: Gradiance Responsive Drawer"/>

        <div class="flex flex-col" >
            <div class="flex flew-row bg-slate-400 dark:bg-slate-600" >
                <div class="md:hidden" >
                    <Show
                        when=move || { !is_drawer_open() }
                        fallback=|| {
                            view! { <></> }
                        }
                    >
                        <button
                            type="button"
                            class="mt-6 mx-4"
                            aria-controls="mobile-menu"
                            aria-expanded="false"
                            on:click=move |_| {
                                set_drawer_open(true);
                            }
                        >
                            <span class="sr-only">"Open main menu"</span>
                            <Icon icon=icondata::ChMenuHamburger/>
                        </button>
                    </Show>
                </div>
                <div>
                    <div class="my-4 mx-2 text-2xl" >"Responsive Drawer"</div>
                </div>
            </div> 
            <div class="flex flex-row" >
                <Drawer is_open=is_drawer_open set_open=set_drawer_open 
                    variant=DrawerVariant::variant(&DrawerVariant::Responsive { side: HorizontalSide::Left }) > 
                    <div class="flex flex-row md:hidden" >
                        <h1 class="text-2xl font-semibold" >"Drawer Title"</h1>
                        <button 
                            class="bg-slate-400 dark:bg-slate-700 border border-slate-500 rounded-lg p-4"
                            on:click=move |_| set_drawer_open(false)
                        >"X"</button>
                    </div>
                    <div class="mt-5" >
                        <NavMenuExample is_open={true} />
                    </div>
                </Drawer> 
                <div>
                    <LoremIpsumLong />
                </div>         
            </div>
        </div>
        
    }
}


#[component]
pub fn PageDrawerStaggered() -> impl IntoView { 

    
    let (is_drawer_open, set_drawer_open) = create_signal(false);
    

    view! { 
        <Title text="Leptail: Gradiance Responsive Drawer"/>

        <div class="flex flex-col" >
            <div class="flex flew-row bg-slate-400 dark:bg-slate-600" >
                <div class="" >
                    // <Show
                    //     when=move || { !is_drawer_open() }
                    //     fallback=|| {
                    //         view! { <></> }
                    //     }
                    // >
                    // </Show>
                    <button
                        type="button"
                        class="mt-6 mx-4"
                        aria-controls="mobile-menu"
                        aria-expanded="false"
                        on:click=move |_| {
                            set_drawer_open(!is_drawer_open());
                        }
                    >
                        <span class="sr-only">"Open main menu"</span>
                        <Icon icon=icondata::ChMenuHamburger/>
                    </button>
                </div>
                <div>
                    <div class="my-4 mx-2 text-2xl" >"Staggered Drawer"</div>
                </div>
            </div> 
            <div class="flex flex-row" >
                <Drawer is_open=is_drawer_open set_open=set_drawer_open 
                    variant=DrawerVariant::variant(&DrawerVariant::Staggered { breakover_point: Size::Large, side: HorizontalSide::Left }) > 
                    <div class="flex flex-row md:hidden" >
                        <h1 class="text-2xl font-semibold" >"Drawer Title"</h1>
                        <button 
                            class="bg-slate-400 dark:bg-slate-700 border border-slate-500 rounded-lg p-4"
                            on:click=move |_| set_drawer_open(false)
                        >"X"</button>
                    </div>
                    <div class="mt-5" >
                        <NavMenuExample is_open=is_drawer_open />
                    </div>
                </Drawer> 
                <div>
                    <LoremIpsumLong />
                </div>         
            </div>
        </div>
        
    }
}


#[component]
pub fn PageDrawerStaggeredMini() -> impl IntoView { 

    
    let (is_drawer_open, set_drawer_open) = create_signal(false);
    

    view! { 
        <Title text="Leptail: Gradiance Responsive Drawer"/>

        <div class="flex flex-col" >
            <div class="flex flew-row bg-slate-400 dark:bg-slate-600" >
                <div class="" >
                    // <Show
                    //     when=move || { !is_drawer_open() }
                    //     fallback=|| {
                    //         view! { <></> }
                    //     }
                    // >
                    // </Show>
                    <button
                        type="button"
                        class="mt-6 mx-4"
                        aria-controls="mobile-menu"
                        aria-expanded="false"
                        on:click=move |_| {
                            set_drawer_open(!is_drawer_open());
                        }
                    >
                        <span class="sr-only">"Open main menu"</span>
                        <Icon icon=icondata::ChMenuHamburger/>
                    </button>
                </div>
                <div>
                    <div class="my-4 mx-2 text-2xl" >"Staggered Mini Drawer"</div>
                </div>
            </div> 
            <div class="flex flex-row" >
                <Drawer is_open=is_drawer_open set_open=set_drawer_open 
                    variant=DrawerVariant::variant(&DrawerVariant::StaggeredMini { breakover_point: Size::Large, side: HorizontalSide::Left }) > 
                    <div class="flex flex-row md:hidden" >
                        <h1 class="text-2xl font-semibold" >"Drawer Title"</h1>
                        <button 
                            class="bg-slate-400 dark:bg-slate-700 border border-slate-500 rounded-lg p-4"
                            on:click=move |_| set_drawer_open(false)
                        >"X"</button>
                    </div>
                    <div class="mt-5" >
                        <NavMenuExample is_open=is_drawer_open />
                    </div>
                </Drawer> 
                <div>
                    <LoremIpsumLong />
                </div>         
            </div>
        </div>
        
    }
}
