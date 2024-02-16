use leptail::prelude::*;
use leptos::*;
use leptos_meta::*;
use leptail_theme_gradiance::*; 

#[component]
pub fn PageDrawer() -> impl IntoView { 

    let (is_left_drawer_open, set_left_drawer_open) = create_signal(false); 
    let (is_right_drawer_open, set_right_drawer_open) = create_signal(false); 
    let (is_top_drawer_open, set_top_drawer_open) = create_signal(false); 
    let (is_bottom_drawer_open, set_bottom_drawer_open) = create_signal(false); 

    view! { 
        <Title text="Leptail: Gradiance Drawer and Variants"/>

        <div class="flex flex-col gap-4" >
            <button 
                class="bg-slate-400 dark:bg-slate-700 border border-slate-500 rounded-lg p-4"
                on:click=move |_| set_left_drawer_open(true)
            >"Open Left Drawer"</button>

            <button 
                class="bg-slate-400 dark:bg-slate-700 border border-slate-500 rounded-lg p-4"
                on:click=move |_| set_right_drawer_open(true)
            >"Open Right Drawer"</button>

            <button 
                class="bg-slate-400 dark:bg-slate-700 border border-slate-500 rounded-lg p-4"
                on:click=move |_| set_top_drawer_open(true)
            >"Open Top Drawer"</button>

            <button 
                class="bg-slate-400 dark:bg-slate-700 border border-slate-500 rounded-lg p-4"
                on:click=move |_| set_bottom_drawer_open(true)
            >"Open Bottom Drawer"</button>


             <Drawer is_open=is_right_drawer_open set_open=set_right_drawer_open side=DrawerSide::Right > 
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

             <Drawer is_open=is_left_drawer_open set_open=set_left_drawer_open side=DrawerSide::Left > 
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

             <Drawer is_open=is_top_drawer_open set_open=set_top_drawer_open side=DrawerSide::Top > 
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

            <Drawer is_open=is_bottom_drawer_open set_open=set_bottom_drawer_open side=DrawerSide::Bottom > 
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
        </div>
    }
}
