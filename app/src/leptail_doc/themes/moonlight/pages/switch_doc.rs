use leptail::prelude::*;
use leptos::*;
use leptos_meta::*;
use leptail_theme_moonlight::*; 

#[component]
pub fn PageSwitch() -> impl IntoView { 

    let (s1, set_s1) = create_signal(false);
    let (s2, set_s2) = create_signal(true);
    let (s3, set_s3) = create_signal(false);
    let (s4, set_s4) = create_signal(true);
    // let (state, set_state) = create_signal(false);

    view! {
        <Title text="Leptail: Gradiance Switch Theme and Variants"/>

        <div class="flex flex-col gap-4">
            <Switch
                is_on=s1
                set_on=set_s1
                variant=SwitchVariant::variant(Color::Default.into(), None)
            />
            <Switch
                is_on=s2
                set_on=set_s2
                variant=SwitchVariant::variant(Color::Primary.into(), None)
            />
            <Switch
                is_on=s3
                set_on=set_s3
                disabled=true
                variant=SwitchVariant::variant(Color::Warning.into(), Size::Large.into())
            />
            <Switch
                is_on=s4
                set_on=set_s4
                variant=SwitchVariant::variant(Color::Secondary.into(), Size::Xsmall.into())
            />
            <Switch
                is_on=s1
                set_on=set_s1
                variant=SwitchVariant::variant(Color::Success.into(), Size::Small.into())
            />
            <Switch
                is_on=s2
                set_on=set_s2
                variant=SwitchVariant::variant(Color::Info.into(), Size::Medium.into())
            />
            <Switch
                is_on=s3
                set_on=set_s3
                variant=SwitchVariant::variant(Color::Warning.into(), Size::Large.into())
            />
            <Switch
                is_on=s4
                set_on=set_s4
                variant=SwitchVariant::variant(Color::Danger.into(), Size::Xlarge.into())
            />
            <Switch
                is_on=s1
                set_on=set_s1
                variant=SwitchVariant::variant(Color::Info.into(), Size::Large.into())
                off_icon=icondata::BsCircle
                on_icon=icondata::BsCircleFill
            />
            <Switch
                is_on=s1
                set_on=set_s1
                variant=SwitchVariant::variant(Color::Success.into(), Size::Large.into())
                off_icon=icondata::BsMoonFill
                on_icon=icondata::BsSunFill
            />

            <Switch is_on=s1 set_on=set_s1/>
        </div>
    }
}
