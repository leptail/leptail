use tailwind_fuse::tw_merge;

use crate::moonlight::*;

pub struct SwitchVariant {
    color: Option<Color>,
    size: Option<Size>,
    on_icon: Option<icondata::Icon>,
    off_icon: Option<icondata::Icon>,
}

impl SwitchVariant {
    pub fn builder() -> Self {
        SwitchVariant {
            color: None,
            size: None,
            on_icon: None,
            off_icon: None,
        }
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    pub fn set_color(mut self, color: Option<Color>) -> Self {
        self.color = color;
        self
    }

    pub fn size(mut self, size: Size) -> Self {
        self.size = Some(size);
        self
    }

    pub fn set_size(mut self, size: Option<Size>) -> Self {
        self.size = size;
        self
    }

    pub fn on_icon(mut self, on_icon: icondata::Icon) -> Self {
        self.on_icon = Some(on_icon);
        self
    }

    pub fn off_icon(mut self, off_icon: icondata::Icon) -> Self {
        self.off_icon = Some(off_icon);
        self
    }

    pub fn default_variant() -> SwitchTheme {
        Self::builder().build()
    }

    pub fn build(self) -> SwitchTheme {
        let base_theme = SwitchBaseTheme {
            switch: tw_merge!(
                "relative rounded-full hover:outline-none",
                Self::switch_base_color(&self.color.unwrap_or_default()),
                // match self.color {
                //     Some(color) => Self::switch_color(&color),
                //     None => Self::switch_color(&Color::default()),
                // },
                Self::switch_size(&self.size.unwrap_or_default())
            ),
            icon_container: tw_merge!(
                "absolute transition-all duration-150",
                Self::icon_size(&self.size.unwrap_or_default())
            ),
        };

        let on_modifier = SwitchBaseTheme {
            switch: tw_merge!("", Self::switch_on_color(&self.color.unwrap_or_default()),),
            icon_container: tw_merge!("left-[50%]"),
        };

        let off_modifier = SwitchBaseTheme {
            switch: tw_merge!("", Self::switch_off_color(),),
            icon_container: tw_merge!("left-0"),
        };

        let disabled_modifier = SwitchBaseTheme {
            switch: tw_merge!("cursor-not-allowed opacity-50"),
            icon_container: tw_merge!(""),
        };

        let enabled_modifier = SwitchBaseTheme {
            switch: tw_merge!("cursor-pointer hover:ring-4"),
            icon_container: tw_merge!(""),
        };

        let on_icon = Some(self.on_icon.unwrap_or(icondata::BsCircleFill));
        let off_icon = Some(self.off_icon.unwrap_or(icondata::BsCircle));

        SwitchTheme {
            base: base_theme,
            on_modifier,
            off_modifier,
            disabled_modifier,
            enabled_modifier,
            on_icon,
            off_icon,
        }
    }

    // Following are helper functions
    fn switch_base_color(color: &Color) -> String {
        tw_merge!(
            color.make_shade("hover:ring", Palette::S200),
            color.make_shade("dark:hover:ring", Palette::S800)
        )
    }

    fn switch_off_color() -> String {
        tw_merge!(
            Color::Default.make_shade("bg", Palette::S200),
            Color::Default.make_shade("dark:bg", Palette::S600)
        )
    }

    fn switch_on_color(color: &Color) -> String {
        tw_merge!(
            color.make_shade("bg", Palette::S300),
            color.make_shade("dark:bg", Palette::S700)
        )
    }

    fn switch_size(size: &Size) -> String {
        match size {
            Size::XSmall => String::from("w-8 h-4"),
            Size::Small => String::from("w-10 h-5"),
            Size::Medium => String::from("w-12 h-6"),
            Size::Large => String::from("w-16 h-8"),
            Size::XLarge => String::from("w-20 h-10"),
        }
    }

    fn icon_size(size: &Size) -> String {
        match size {
            Size::XSmall => String::from("text-[1rem]"),
            Size::Small => String::from("text-[1.25rem]"),
            Size::Medium => String::from("text-[1.5rem]"),
            Size::Large => String::from("text-[2rem]"),
            Size::XLarge => String::from("text-[2.5rem]"),
        }
    }
}
