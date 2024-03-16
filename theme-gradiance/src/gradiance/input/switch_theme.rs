use crate::gradiance::*;

pub struct SwitchVariant {}

impl SwitchVariant {

    pub fn base_theme() -> SwitchTheme{
        let base = SwitchBaseTheme { 
            switch: String::from("relative rounded-full hover:outline-none"),
            icon_container: String::from("absolute transition-all duration-150"),
        };
        let on_modifier: SwitchBaseTheme = SwitchBaseTheme{ 
            switch: String::from(""),
            icon_container: String::from("left-[50%]"), 
        };
        let off_modifier = SwitchBaseTheme{ 
            switch: String::from(""),
            icon_container: String::from("left-0"), 
        };
        let disabled_modifier = SwitchBaseTheme{ 
            switch: String::from("cursor-not-allowed opacity-50"),
            icon_container: String::from(""), 
        };
        let enabled_modifier = SwitchBaseTheme{ 
            switch: String::from("cursor-pointer hover:ring-4"),
            icon_container: String::from(""), 
        };
        SwitchTheme{
            base,
            on_modifier,
            off_modifier,
            disabled_modifier,
            enabled_modifier,
            // on_icon=Some(icondata::BsSunFill),
            // off_icon=Some(icondata::BsMoonFill),
            on_icon: Some(icondata::TbCircleDotFilled),  
            off_icon: Some(icondata::TbCircleDashed), 
        }
    }

    // This doesn't work since it dynamically gnerated and tailwind compiler doesn't know at the compile time
    fn switch_color(color: &Color) -> String {
        format!("{} {}", color.make_shade("hover:ring", Palette::S200), color.make_shade("dark:hover:ring", Palette::S800))
    }

    fn switch_off_color() -> String {
        // format!("{} {}", Color::Default.make_shade("bg", Palette::S200), Color::Default.make_shade("dark:bg", Palette::S600)) 
        let color = Color::Default;
        Gradient::Radial(None)
            .make("bg".into(), 
                GradientColors::from(TWColorPalette::from(color.to_tw_color(), Palette::S300), TWColorPalette::from(color.to_tw_color(), Palette::S300).into(), TWColorPalette::from(color.to_tw_color(), Palette::S200)), 
                GradientColors::from(TWColorPalette::from(color.to_tw_color(), Palette::S600), TWColorPalette::from(color.to_tw_color(), Palette::S600).into(), TWColorPalette::from(color.to_tw_color(), Palette::S700))
            )
    }

    fn switch_on_color(color: &Color) -> String {
        // format!("{} {}", color.make_shade("bg", Palette::S300), color.make_shade("dark:bg", Palette::S700)) 
        Gradient::Radial(None)
            .make("bg".into(), 
                GradientColors::from(TWColorPalette::from(color.to_tw_color(), Palette::S400), TWColorPalette::from(color.to_tw_color(), Palette::S400).into(), TWColorPalette::from(color.to_tw_color(), Palette::S300)), 
                GradientColors::from(TWColorPalette::from(color.to_tw_color(), Palette::S600), TWColorPalette::from(color.to_tw_color(), Palette::S600).into(), TWColorPalette::from(color.to_tw_color(), Palette::S700))
            )
    }

    fn switch_size(size: &Size) -> String {
        match  size {
            Size::XSmall => String::from("w-8 h-4"),
            Size::Small => String::from("w-10 h-5"),
            Size::Medium => String::from("w-12 h-6"),
            Size::Large => String::from("w-16 h-8"),
            Size::XLarge => String::from("w-20 h-10"),
        }
    }

    fn icon_size(size: &Size) -> String {
        match  size {
            Size::XSmall => String::from("text-[1rem]"),
            Size::Small => String::from("text-[1.25rem]"),
            Size::Medium => String::from("text-[1.5rem]"),
            Size::Large => String::from("text-[2rem]"),
            Size::XLarge => String::from("text-[2.5rem]"),
        }
    }

    fn apply_color(theme: &mut SwitchTheme, color: &Color) { 
        theme.base.switch = format!("{} {}", theme.base.switch, Self::switch_color(color));
        theme.off_modifier.switch = format!("{} {}", theme.off_modifier.switch, Self::switch_off_color());
        theme.on_modifier.switch = format!("{} {}", theme.on_modifier.switch, Self::switch_on_color(color));
    } 

    fn apply_size(theme: &mut SwitchTheme, size: Size) { 
        theme.base.switch = format!("{} {}", theme.base.switch, Self::switch_size(&size));
        theme.base.icon_container = format!("{} {}", theme.base.icon_container, Self::icon_size(&size));
    } 

    pub fn default() -> SwitchTheme {
        Self::variant(None, None)
    }
    pub fn variant(color: Option<Color>, size: Option<Size>) -> SwitchTheme{
        
        let mut theme = Self::base_theme().clone();
       
        Self::apply_color(&mut theme, &color.unwrap_or_default());
        Self::apply_size(&mut theme, size.unwrap_or_default());

        theme
    }
 

}
