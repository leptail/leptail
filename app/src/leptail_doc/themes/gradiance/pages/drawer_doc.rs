use std::borrow::Borrow;

use leptail::prelude::*;
use leptos::*;
use leptos_meta::*;
use leptail_theme_gradiance::{gradiance::{navigation::drawer_theme::DrawerVariant, Side}, *}; 

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
        DrawerVariantState{ state: create_signal(false), variant_btn_text: "Open Right", variant: DrawerVariant::Temporary {  size: Size::Medium, side: Side::Right, has_inset: false } },
        DrawerVariantState{ state: create_signal(false), variant_btn_text: "Open Top", variant: DrawerVariant::Temporary {  size: Size::Medium, side: Side::Top, has_inset: false } },
        DrawerVariantState{ state: create_signal(false), variant_btn_text: "Open Bottom", variant: DrawerVariant::Temporary {  size: Size::Medium, side: Side::Bottom, has_inset: false } }
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

        <div class="flex flex-col gap-4" >

            <h3 class="text-xl" >"Drawer opening side"</h3>
            <div class="flex flex-col lg:flex-row gap-4" >
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
                            <h1 class="text-2xl font-semibold" >"Right side drawer"</h1>
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

            <h3 class="text-xl" >"Drawer size"</h3>
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
                            <h1 class="text-2xl font-semibold" >"Right side drawer"</h1>
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
    }
}
