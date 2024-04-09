use leptos::*;

pub mod appbar;
pub mod drawer_doc;
pub mod overview;
pub mod switch_doc;

#[component]
pub fn ThemeExampleFrame(
    src: String, 
    #[prop(into)] heading: String
) -> impl IntoView {
    view! {
        <h1 class="text-xl font-semibold">{heading}</h1>
        <iframe class="w-full min-h-[25rem]" src=src frameborder="0" height="100%"></iframe>
    }
}
