use leptos::*;
use crate::prelude::*;

#[derive(Debug, Clone, Default)]
pub struct OverlayTheme { 
    pub wrapper: String,
    pub inner: String,
}



#[component]
pub fn Overlay<S>( 
    /// callback when clicked on the backdrop 
    on_click: S,
    /// optional drawer theme variant
    // #[prop(into, optional)]
    #[prop(into, optional)]  
    variant: OptionalMaybeSignal<OverlayTheme>,
    children: Children,
) -> impl IntoView
where
    S: Fn() + 'static,
{
    
    let theme = variant.or_else( || use_context::<AppTheme>().unwrap_or_default().overlay);
    
    
    // let wrapper_class =  move || theme.with(|x| x.wrapper.clone());
    // let inner_class = move || theme.with(|x| x.inner.clone());
     
    // let wrapper_class =  move || with!(|theme| theme.wrapper.clone());
    // let inner_class = move || with!(|theme| theme.inner.clone());

    // let wrapper_class =  move || theme.clone().with(|x| x.wrapper.clone());
    // let inner_class = move || theme.clone().with(|x| x.inner.clone());

    // let wrapper_class =  move || theme().wrapper.clone();
    // let inner_class = move || theme().inner.clone();


    // TODO: This may is wrong; cloning signals creates a new signal
    let wrapper_class  = {
        let cloned = theme.clone();
        move || cloned.with(|x| x.wrapper.clone()) 
    };

    let inner_class  =  {
        let cloned = theme.clone();
        move || cloned.with(|x| x.inner.clone())
    };

    // let wrapper_class = "";
    // let inner_class = "";



    view! {
        <div class=wrapper_class>
            <div 
                class=inner_class 
                on:click=move |_| (on_click)()
            >
                {children()}
            </div>
        </div>
    }
}