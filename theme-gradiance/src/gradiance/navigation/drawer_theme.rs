

use crate::gradiance::*;


pub enum DrawerVariant {
    Temporary{size: Size, side: Side, has_inset: bool}, 
    Persistent(Size), // argument: breakover_point, drawer size is md, and side is left 
    PersistentMini(Size), // argument: breakover_point, drawer size is md, and side is left
}

impl DrawerVariant {

    fn base_theme(has_overlay: bool) -> DrawerTheme{

        let bg_gradient = Gradient::Linear(Direction::Left)
            .make("bg".into(), 
                GradientColors::from(TWColorPalette::from(TWColor::Fuchsia, Palette::S50), TWColorPalette::from(TWColor::Indigo, Palette::S50).into(), TWColorPalette::from(TWColor::Cyan, Palette::S50)), 
                GradientColors::from(TWColorPalette::from(TWColor::Fuchsia, Palette::S950), TWColorPalette::from(TWColor::Indigo, Palette::S950).into(), TWColorPalette::from(TWColor::Cyan, Palette::S950))
            );
        
        let base_theme = DrawerTheme {
            base: format!("{bg_gradient} z-[101] fixed overflow-x-hidden transition-all duration-500 ease-out"),
            minimized: String::from(""),
            maximized: String::from(""),
            has_overlay: has_overlay,
        };

        base_theme
    }

    fn drawer_width(size: &Size) -> String {
        match  size {
            Size::XSmall => String::from("w-40"), 
            Size::Small => String::from("w-60"), 
            Size::Medium => String::from("w-80"), 
            Size::Large => String::from("w-80"),
            Size::XLarge => String::from("w-80"),
        }
    }

    fn drawer_height(size: &Size) -> String {
        match size {
            Size::XSmall => String::from("h-40"), 
            Size::Small => String::from("h-40"), 
            Size::Medium => String::from("h-40"), 
            Size::Large => String::from("h-40"),
            Size::XLarge => String::from("h-40"),
        }
    }

    fn side_modifier(side: &Side, size: &Size, has_overlay: bool) -> DrawerTheme {
        let height = Self::drawer_height(&size);
        let width = Self::drawer_width(&size);
        match side {
            Side::Left => DrawerTheme {
                base: format!("h-full {width} top-0 left-0"),
                minimized: String::from("-translate-x-full"),
                maximized: String::from("translate-x-0"),
                has_overlay: has_overlay,
            },
            Side::Right => DrawerTheme {
                base: format!("h-full {width} top-0 right-0"),
                minimized: String::from("translate-x-full"),
                maximized: String::from("translate-x-0"),
                has_overlay: has_overlay,
            },
            Side::Top => DrawerTheme {
                base: format!("w-full {height} top-0 right-0 left-0"),
                minimized: String::from("-translate-y-full"),
                maximized: String::from("translate-y-0"),
                has_overlay: has_overlay,
            },
            Side::Bottom => DrawerTheme {
                base: format!("w-full {height} bottom-0 right-0 left-0"),
                minimized: String::from("translate-y-full"),
                maximized: String::from("translate-y-0"),
                has_overlay: has_overlay,
            },
        }
    }


    fn merge(first_theme: DrawerTheme, second_theme: DrawerTheme) -> DrawerTheme { 
        let merged = DrawerTheme {
            base: format!("{} {}", first_theme.base, second_theme.base),
            minimized: format!("{} {}", first_theme.minimized, second_theme.minimized),
            maximized: format!("{} {}", first_theme.maximized, second_theme.maximized),
            has_overlay: first_theme.has_overlay && second_theme.has_overlay,
        };
        merged
    }

    // TODO: add variant: 
    // TODO: 
    pub fn variant(side: Option<Side>, size: Option<Size>) -> DrawerTheme{

        let side = side.unwrap_or(Side::Left);
        let size = size.unwrap_or(Size::Medium);
        
        let base = Self::base_theme(true);
        let side_modified_theme = Self::side_modifier(&side, &size, true);

        let theme = Self::merge(base, side_modified_theme);
        theme
    }
}

