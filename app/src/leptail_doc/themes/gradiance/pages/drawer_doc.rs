use leptail::prelude::*;
use leptos::*;
use leptos_meta::*;
use leptail_theme_gradiance::{gradiance::{navigation::drawer_theme::DrawerVariant, Side}, *}; 

#[component]
pub fn PageDrawer() -> impl IntoView { 

    let (is_left_drawer_open, set_left_drawer_open) = create_signal(false); 
    let (is_right_drawer_open, set_right_drawer_open) = create_signal(false); 
    let (is_top_drawer_open, set_top_drawer_open) = create_signal(false); 
    let (is_bottom_drawer_open, set_bottom_drawer_open) = create_signal(false); 

    let (is_xs_drawer_open, set_xs_drawer_open) = create_signal(false); 
    let (is_sm_drawer_open, set_sm_drawer_open) = create_signal(false); 
    let (is_md_drawer_open, set_md_drawer_open) = create_signal(false); 
    let (is_lg_drawer_open, set_lg_drawer_open) = create_signal(false); 
    let (is_xl_drawer_open, set_xl_drawer_open) = create_signal(false); 

    view! { 
        <Title text="Leptail: Gradiance Drawer and Variants"/>

        <div class="flex flex-col gap-4" >

            <h3 class="text-xl" >"Drawer opening side"</h3>
            <div class="flex flex-row lg:flex-col gap-4" >
                <button 
                    class="bg-slate-400 dark:bg-slate-700 border border-slate-500 rounded-lg p-4"
                    on:click=move |_| set_left_drawer_open(true)
                >"Open Left"</button>

                <button 
                    class="bg-slate-400 dark:bg-slate-700 border border-slate-500 rounded-lg p-4"
                    on:click=move |_| set_right_drawer_open(true)
                >"Open Right"</button>

                <button 
                    class="bg-slate-400 dark:bg-slate-700 border border-slate-500 rounded-lg p-4"
                    on:click=move |_| set_top_drawer_open(true)
                >"Open Top"</button>

                <button 
                    class="bg-slate-400 dark:bg-slate-700 border border-slate-500 rounded-lg p-4"
                    on:click=move |_| set_bottom_drawer_open(true)
                >"Open Bottom"</button>            
            </div>


            <h3 class="text-xl" >"Drawer size"</h3>
            <div class="flex flex-row lg:flex-col gap-4" >
                <button 
                    class="bg-slate-400 dark:bg-slate-700 border border-slate-500 rounded-lg p-4"
                    on:click=move |_| set_xs_drawer_open(true)
                >"Open Extra Small"</button>

                          
            </div>

            
            <Drawer is_open=is_right_drawer_open set_open=set_right_drawer_open 
                variant=DrawerVariant::variant(Side::Right.into(), None) > 
                <div class="flex flex-row" >
                    <h1 class="text-2xl font-semibold" >"Right side drawer"</h1>
                    <button 
                        class="bg-slate-400 dark:bg-slate-700 border border-slate-500 rounded-lg p-4"
                        on:click=move |_| set_right_drawer_open(false)
                    >"X"</button>
                </div>
                <div class="mt-5" >
                    "Drawer content here..."
                </div>
             </Drawer>

             <Drawer is_open=is_left_drawer_open set_open=set_left_drawer_open 
                > 
                <div class="flex flex-row" >
                    <h1 class="text-2xl font-semibold" >"Left side drawer"</h1>
                    <button 
                        class="bg-slate-400 dark:bg-slate-700 border border-slate-500 rounded-lg p-4"
                        on:click=move |_| set_left_drawer_open(false)
                    >"X"</button>
                </div>
                <div class="mt-5" >
                    "Drawer content here..."
                </div>
             </Drawer>

             <Drawer is_open=is_top_drawer_open set_open=set_top_drawer_open 
                variant=DrawerVariant::variant(Side::Top.into(), None)  > 
                <div class="flex flex-row" >
                    <h1 class="text-2xl font-semibold" >"Top side drawer"</h1>
                    <button 
                        class="bg-slate-400 dark:bg-slate-700 border border-slate-500 rounded-lg p-4"
                        on:click=move |_| set_top_drawer_open(false)
                    >"X"</button>
                </div>
                <div class="mt-5" >
                    "Drawer content here..."
                </div>
            </Drawer>

            <Drawer is_open=is_bottom_drawer_open set_open=set_bottom_drawer_open 
                variant=DrawerVariant::variant(Side::Bottom.into(), None)  > 
                <div class="flex flex-row" >
                    <h1 class="text-2xl font-semibold" >"Bottom side drawer"</h1>
                    <button 
                        class="bg-slate-400 dark:bg-slate-700 border border-slate-500 rounded-lg p-4"
                        on:click=move |_| set_bottom_drawer_open(false)
                    >"X"</button>
                </div>
                <div class="mt-5" >
                    "Drawer content here..."
                </div>
            </Drawer>


            <Drawer is_open=is_xs_drawer_open set_open=set_xs_drawer_open 
                variant=DrawerVariant::variant(Side::Left.into(), Size::XSmall.into()) > 
                <div class="flex flex-row" >
                    <h1 class="text-2xl font-semibold" >"Extra Small Drawer"</h1>
                    <button 
                        class="bg-slate-400 dark:bg-slate-700 border border-slate-500 rounded-lg p-4"
                        on:click=move |_| set_xs_drawer_open(false)
                    >"X"</button>
                </div>
                <div class="mt-5" >
                    "Drawer content here..."
                </div>
            </Drawer>
        </div>
    }
}
