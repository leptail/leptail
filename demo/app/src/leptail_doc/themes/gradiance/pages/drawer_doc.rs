use std::borrow::Borrow;

use crate::leptail_doc::themes::gradiance::{
    gradiance_routes::GradianceRoutes, pages::common_components::*,
};
use leptail::prelude::*;
use leptail_theme_gradiance::{
    gradiance::{navigation::drawer_theme::DrawerVariant, HorizontalSide, Side},
    *,
};
use leptos::*;
use leptos_meta::*;

// #[derive(Copy, Debug, PartialEq, Eq)]
struct DrawerVariantState {
    state: (ReadSignal<bool>, WriteSignal<bool>),
    variant_btn_text: &'static str,
    variant: DrawerVariant,
}

#[component]
pub fn PageDrawer() -> impl IntoView {
    let (drawer_size, set_drawer_size) = create_signal(Size::Medium);
    let (drawer_side, set_drawer_side) = create_signal(Side::Left);
    let (has_inset, set_has_inset) = create_signal(false);

    let drawer_variant = Signal::derive(move || {
        with!(|drawer_size, drawer_side, has_inset| {
            DrawerVariant::temporary()
                .size(*drawer_size)
                .side(*drawer_side)
                .has_inset(*has_inset)
                .as_drawer_variant()
                .build()
        })
    });

    let (is_drawer_open, set_drawer_open) = create_signal(false);

    let all_sizes = vec![
        Size::XSmall,
        Size::Small,
        Size::Medium,
        Size::Large,
        Size::XLarge,
    ];
    let size_text = |size: &Size| match size {
        Size::XSmall => "Extra Small",
        Size::Small => "Small",
        Size::Medium => "Medium",
        Size::Large => "Large",
        Size::XLarge => "Extra Large",
    };
    let size_buttons = all_sizes
        .into_iter()
        .map(|size| {
            view! {
                <div class="mr-5">
                    <input
                        type="radio"
                        name="size_radio"
                        checked=move || size == drawer_size()
                        on:click=move |_| set_drawer_size(size)
                    />
                    <span class="ml-2">{size_text(&size)}</span>
                </div>
            }
        })
        .collect_view();

    let all_sides = vec![Side::Left, Side::Right, Side::Top, Side::Bottom];
    let side_text = |side: &Side| match side {
        Side::Left => "Left",
        Side::Right => "Right",
        Side::Top => "Top",
        Side::Bottom => "Bottom",
    };
    let side_buttons = all_sides
        .into_iter()
        .map(|side| {
            view! {
                <div class="mr-5">
                    <input
                        type="radio"
                        name="side_radio"
                        checked=move || side == drawer_side()
                        on:click=move |_| set_drawer_side(side)
                    />
                    <span class="ml-2">{side_text(&side)}</span>
                </div>
            }
        })
        .collect_view();

    view! {
        <Title text="Leptail: Gradiance Drawer and Variants"/>

        <div class="">
            <div class="my-16">
                <h3 class="text-xl text-left mb-5">"Temporary Drawer"</h3>
                <div class="w-full flex flex-col space-y-2 mx-5">
                    <div class="flex flex-row">
                        <div class="mr-2 opacity-75">Side:</div>
                        {side_buttons}
                    </div>
                    <div class="flex flex-row">
                        <div class="mr-2 opacity-75">Size:</div>
                        {size_buttons}
                    </div>
                    <div class="flex flex-row">
                        <input
                            type="checkbox"
                            checked=move || has_inset() == true
                            on:click=move |_| set_has_inset.update(|b| *b = !*b)
                        />
                        <span class="ml-2">"Has Inset"</span>
                    </div>
                    <div class="flex flex-row">
                        <button
                            class="bg-slate-400 dark:bg-slate-700 border border-slate-500 rounded-lg p-4 mt-5"
                            on:click=move |_| set_drawer_open(true)
                        >
                            "Open Drawer "
                        </button>
                    </div>
                </div>

                <Drawer is_open=is_drawer_open set_open=set_drawer_open variant=drawer_variant>
                    <div class="flex flex-row">
                        <h1 class="text-2xl font-semibold">"Drawer Title"</h1>
                        <button
                            class="bg-slate-400 dark:bg-slate-700 border border-slate-500 rounded-lg p-4"
                            on:click=move |_| set_drawer_open(false)
                        >
                            "X"
                        </button>
                    </div>
                    <div class="mt-5">
                        // TODO: if has_inset show from inside the drawer else show nav menu

                        <NavMenuExample is_open=is_drawer_open/>
                    </div>
                </Drawer>
            </div>

            <div class="my-16">
                <h3 class="text-xl text-left mb-5">"Responsive Drawer"</h3>
                <iframe
                    class="w-full min-h-[25rem]"
                    src=GradianceRoutes::DrawerResponsive.as_href()
                    frameborder="0"
                    height="100%"
                ></iframe>
            </div>

            <div class="my-16">
                <h3 class="text-xl text-left mb-5">"Staggered Drawer"</h3>
                <iframe
                    class="w-full min-h-[25rem]"
                    src=GradianceRoutes::DrawerStaggered.as_href()
                    frameborder="0"
                    height="100%"
                ></iframe>
            </div>

            <div class="my-16">
                <h3 class="text-xl text-left mb-5">"Staggered Mini Drawer"</h3>
                <iframe
                    class="w-full min-h-[25rem]"
                    src=GradianceRoutes::DrawerStaggeredMini.as_href()
                    frameborder="0"
                    height="100%"
                ></iframe>
            </div>

        </div>
    }
}

#[component]
pub fn PageDrawerResponsive() -> impl IntoView {
    let (is_drawer_open, set_drawer_open) = create_signal(false);

    view! {
        <Title text="Leptail: Gradiance Responsive Drawer"/>

        <div class="flex flex-col">
            <div class="flex flew-row bg-slate-400 dark:bg-slate-600">
                <div class="md:hidden">
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
                    <div class="my-4 mx-2 text-2xl">"Responsive Drawer"</div>
                </div>
            </div>
            <div class="flex flex-row">
                <Drawer
                    is_open=is_drawer_open
                    set_open=set_drawer_open
                    variant=DrawerVariant::responsive().side(HorizontalSide::Left).build()
                >

                    <div class="flex flex-row md:hidden">
                        <h1 class="text-2xl font-semibold">"Drawer Title"</h1>
                        <button
                            class="bg-slate-400 dark:bg-slate-700 border border-slate-500 rounded-lg p-4"
                            on:click=move |_| set_drawer_open(false)
                        >
                            "X"
                        </button>
                    </div>
                    <div class="mt-5">
                        <NavMenuExample is_open=true/>
                    </div>
                </Drawer>
                <div>
                    <LoremIpsumLong/>
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

        <div class="flex flex-col">
            <div class="flex flew-row bg-slate-400 dark:bg-slate-600">
                <div class="">
                    // <Show
                    // when=move || { !is_drawer_open() }
                    // fallback=|| {
                    // view! { <></> }
                    // }
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
                    <div class="my-4 mx-2 text-2xl">"Staggered Drawer"</div>
                </div>
            </div>
            <div class="flex flex-row">
                <Drawer
                    is_open=is_drawer_open
                    set_open=set_drawer_open
                    variant=DrawerVariant::staggered().breakover_point(Size::Large).side(HorizontalSide::Left).build()
                >

                    <div class="flex flex-row md:hidden">
                        <h1 class="text-2xl font-semibold">"Drawer Title"</h1>
                        <button
                            class="bg-slate-400 dark:bg-slate-700 border border-slate-500 rounded-lg p-4"
                            on:click=move |_| set_drawer_open(false)
                        >
                            "X"
                        </button>
                    </div>
                    <div class="mt-5">
                        <NavMenuExample is_open=is_drawer_open/>
                    </div>
                </Drawer>
                <div>
                    <LoremIpsumLong/>
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

        <div class="flex flex-col">
            <div class="flex flew-row bg-slate-400 dark:bg-slate-600">
                <div class="">
                    // <Show
                    // when=move || { !is_drawer_open() }
                    // fallback=|| {
                    // view! { <></> }
                    // }
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
                    <div class="my-4 mx-2 text-2xl">"Staggered Mini Drawer"</div>
                </div>
            </div>
            <div class="flex flex-row">
                <Drawer
                    is_open=is_drawer_open
                    set_open=set_drawer_open
                    variant=DrawerVariant::staggered_mini()
                        .breakover_point(Size::Large)
                        .side(HorizontalSide::Left)
                        .build()
                >

                    <div class="flex flex-row md:hidden">
                        <h1 class="text-2xl font-semibold">"Drawer Title"</h1>
                        <button
                            class="bg-slate-400 dark:bg-slate-700 border border-slate-500 rounded-lg p-4"
                            on:click=move |_| set_drawer_open(false)
                        >
                            "X"
                        </button>
                    </div>
                    <div class="mt-5">
                        <NavMenuExample is_open=is_drawer_open/>
                    </div>
                </Drawer>
                <div>
                    <LoremIpsumLong/>
                </div>
            </div>
        </div>
    }
}
