use leptos::*;
 

#[component]
pub fn Icon(
    #[prop(into)] icon: MaybeSignal<icondata::Icon>,
    // #[prop(into, optional)] id: Option<AttributeValue>,
    // #[prop(into, optional)] class: Option<AttributeValue>,
    // #[prop(into, optional)] style: Option<AttributeValue>,
    // #[prop(into, optional)] aria_label: Option<AttributeValue>, 
) -> impl IntoView {
    // let ico: View = icondata::Icon( 
    //     icondata::IconProps {
    //         icon,
    //         width: None,
    //         height: None,
    //         class: None,
    //         style: None,
    //     },
    // )
    // .into_view();

    // let ico: View = icon.into();

    view! { 
        <>
            <leptos_icons::Icon icon=icon  />
            // { ico }
        </>
    }
}
