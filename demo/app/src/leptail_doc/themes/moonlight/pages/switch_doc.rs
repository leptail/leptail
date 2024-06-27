use leptail::prelude::*;
use leptail_theme_moonlight::*;
use leptos::*;
use leptos_meta::*;

#[component]
pub fn PageSwitch() -> impl IntoView {
    let (s1, set_s1) = create_signal(false);
    let (s2, set_s2) = create_signal(true);
    let (s3, set_s3) = create_signal(false);
    let (s4, set_s4) = create_signal(true);
    // let (state, set_state) = create_signal(false);

    view! {
        <Title text="Leptail: Moonlight Switch Theme and Variants"/>

        <div class="flex flex-col gap-4">
            <Switch tab_index=2 is_on=s1 set_on=set_s1 variant=SwitchVariant::builder().build()/>
            <Switch tab_index=1 is_on=s2 set_on=set_s2 variant=SwitchVariant::builder().color(Color::Primary).build()/>
            <Switch
                tab_index=3
                is_on=s3
                set_on=set_s3
                disabled=true
                variant=SwitchVariant::builder().color(Color::Warning).size(Size::Large).build()
            />
            <Switch
                is_on=s4
                set_on=set_s4
                variant=SwitchVariant::builder().color(Color::Secondary).size(Size::XSmall).build()
            />
            <Switch
                is_on=s1
                set_on=set_s1
                variant=SwitchVariant::builder().color(Color::Success).size(Size::Small).build()
            />
            <Switch
                is_on=s2
                set_on=set_s2
                variant=SwitchVariant::builder().color(Color::Info).size(Size::Medium).build()
            />
            <Switch
                is_on=s3
                set_on=set_s3
                variant=SwitchVariant::builder().color(Color::Warning).size(Size::Large).build()
            />
            <Switch
                is_on=s4
                set_on=set_s4
                variant=SwitchVariant::builder().color(Color::Danger).size(Size::XLarge).build()
            />
            <Switch
                is_on=s1
                set_on=set_s1
                variant=SwitchVariant::builder().color(Color::Info).size(Size::Large).build()
                off_icon=icondata::BsCircle
                on_icon=icondata::BsCircleFill
            />
            <Switch
                is_on=s1
                set_on=set_s1
                variant=SwitchVariant::builder()
                    .color(Color::Success)
                    .size(Size::Large)
                    .on_icon(icondata::BsSunFill)
                    .off_icon(icondata::BsMoonFill)
                    .build()
            />

            <Switch is_on=s1 set_on=set_s1/>
        </div>
    }
}
