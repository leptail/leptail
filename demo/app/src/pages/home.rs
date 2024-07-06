use leptos::prelude::*;
use leptos_meta::Title;

stylance::import_style!(home, "home.module.scss");

#[component]
pub fn Home() -> impl IntoView {
    let (value, set_value) = signal(0);

    // thanks to https://tailwindcomponents.com/component/blue-buttons-example for the showcase layout
    view! {
        <Title text="Leptos"/>

        <div class=home::hero>
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
