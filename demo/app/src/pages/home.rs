use leptos::prelude::*;
use leptos_meta::{Meta, Title};

stylance::import_style!(home_style, "home.module.scss");

#[component]
pub fn Home() -> impl IntoView {
    let (value, set_value) = signal(0);

    // thanks to https://tailwindcomponents.com/component/blue-buttons-example for the showcase layout
    view! {
        <Title text="Leptos"/>
        <Meta
            name="description"
            content="Component Library for Leptos that aims to be headless, with a default Design System"
        />
        <Meta
            name="keywords"
            content="Component Library Leptos, Headless Compoent Library, Unstlyed Component Library"
        />

        <div class=home_style::hero>
            <button on:click=move |_| set_value.update(|value| *value += 1) class="">
                "+"
            </button>
            <button class="">{value}</button>
            <button on:click=move |_| set_value.update(|value| *value -= 1) class="">
                "-"
            </button>
        </div>
    }
}
