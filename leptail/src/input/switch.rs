use leptos::*;
use web_sys::KeyboardEvent;
use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct SwitchBaseTheme { 
    pub switch: String,
    pub icon_container: String, 
}


#[derive(Debug, Clone)]
pub struct SwitchTheme {
    pub base:  SwitchBaseTheme,
    pub on_modifier:  SwitchBaseTheme,
    pub off_modifier: SwitchBaseTheme, 
    pub disabled_modifier: SwitchBaseTheme,
    pub enabled_modifier: SwitchBaseTheme,
    pub on_icon: Option<icondata::Icon>,
    pub off_icon: Option<icondata::Icon>,
}


impl Default for SwitchTheme {
    fn default() -> Self {
        let base = SwitchBaseTheme{ 
            switch: String::from(""),
            icon_container: String::from(""), 
        };
        Self {
          base: base.clone(),
          on_modifier: base.clone(),
          off_modifier: base.clone(),  
          disabled_modifier: base.clone(),
          enabled_modifier: base.clone(),
          on_icon: None,
          off_icon: None
        }
    }
}


#[component]
pub fn Switch( 
    /// state to control if the drawer is open or not
    #[prop(into)] is_on: MaybeSignal<bool>,
    /// callback when state changes
    #[prop(into, optional)] set_on: Option<Out<bool>>,
    /// disabled property; if disabled, click event will have no effect
    #[prop(into, optional)] disabled: OptionalMaybeSignal<bool>,
    /// tab_index property;  Optional tab index; if not set, then tabindex will be by default 0
    #[prop(into, optional)] tab_index: OptionalMaybeSignal<i32>,
    /// optional on icon
    #[prop(into, optional)] on_icon: OptionalMaybeSignal<icondata::Icon>,
    /// optional off icon
    #[prop(into, optional)] off_icon: OptionalMaybeSignal<icondata::Icon>,
    /// optional swith theme variant
    #[prop(into, optional)] variant: OptionalMaybeSignal<SwitchTheme>,
) -> impl IntoView 
{

    
    let theme = variant.or_else(move || use_context::<AppTheme>().unwrap_or_default().switch);
    let on_icon = on_icon.or_else({
        let cloned = theme.clone();
        move || cloned.with(|cloned| cloned.on_icon.unwrap_or(icondata::BsCircleFill))
    });
    let off_icon = off_icon.or_else({
        let cloned = theme.clone();
        move || cloned.with(|cloned| cloned.off_icon.unwrap_or(icondata::BsCircle))
    });
    
    let is_disabled = disabled.or(false);
    
    let switch_modifier = {
        let cloned = theme.clone();
        move || with!(|is_on, is_disabled, cloned| format!("{} {} {}", 
            cloned.base.switch, 
            if *is_on { cloned.on_modifier.switch.clone()} else {cloned.off_modifier.switch.clone()},
            if *is_disabled { cloned.disabled_modifier.switch.clone() } else { cloned.enabled_modifier.switch.clone() }
        ))
    };

    let icon_modifier = {
        let cloned = theme.clone();
        move || with!(|is_on, is_disabled, cloned| format!("{} {} {}", 
            cloned.base.icon_container, 
            if *is_on { cloned.on_modifier.icon_container.clone() } else { cloned.off_modifier.icon_container.clone() },
            if *is_disabled { cloned.disabled_modifier.icon_container.clone()} else { cloned.enabled_modifier.icon_container.clone() }
        ))
    };

    let  toggle_state = move || { 
        let is_disabled = disabled.or(false)();
        // TODO: how is the above code is different than below line? 
        // disabled.0.as_ref().map(|it| it.get()).unwrap_or(false)
        if !is_disabled {
            if let Some(set) = &set_on { set.set(!is_on.get_untracked()) }
        }
    };

    let on_key_up = move |event: KeyboardEvent| {
        if event.key().as_str() == "Enter" || event.key().as_str() == " " {
            event.prevent_default();
            event.stop_propagation();
            toggle_state()
        }
    };

    let tab_index_str = move || tab_index.map(|v| v.to_string()).or("")();

    view! {
        <div
            role="switch"
            aria-checked=move || if is_on() { "true" } else { "false" }
            tabindex=tab_index_str
            // TODO: try this if it works ex: class=("button-20", move || count() % 2 == 1)
            class=switch_modifier
            on:click=move |_| toggle_state()
            on:keyup=on_key_up
        >
            <div class=icon_modifier>
                <Show
                    when=is_on
                    fallback=move || {
                        view! { <Icon icon=off_icon/> }
                    }
                >

                    <Icon icon=on_icon/>
                </Show>
            </div>
        </div>
    }
}