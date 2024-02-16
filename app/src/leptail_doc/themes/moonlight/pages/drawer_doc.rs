use leptail::prelude::*;
use leptos::*;
use leptos_meta::*;
use leptail_theme_moonlight::*; 

#[component]
pub fn PageDrawer() -> impl IntoView { 

    let (is_drawer_open, set_drawer_open) = create_signal(false); 

    view! { 
        <Title text="Leptail: Gradiance Drawer and Variants"/>

        <div class="flex flex-col gap-4" >
            <button 
                class="bg-slate-400 dark:bg-slate-700 border border-slate-500 rounded-lg p-4"
                on:click=move |_| set_drawer_open(true)
            >"Open Drawer"</button>
            <Drawer is_open=is_drawer_open set_open=set_drawer_open side=DrawerSide::Left > 
                <div class="flex flex-row" >
                    <h1 class="text-2xl font-semibold" >"Left side drawer"</h1>
                    <button 
                        class="bg-slate-400 dark:bg-slate-700 border border-slate-500 rounded-lg p-4"
                        on:click=move |_| set_drawer_open(false)
                    >"X"</button>
                </div>
                <div class="mt-5" >
                    "Drawer content here..."
                </div>
            </Drawer>
        </div>
    }
}
