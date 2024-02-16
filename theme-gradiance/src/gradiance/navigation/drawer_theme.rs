use leptail::navigation::DrawerSideTheme;

use crate::gradiance::*;

pub struct DrawerVariant {}

impl DrawerVariant {

    pub fn base_theme() -> DrawerTheme{
        
        let left_side = DrawerSideTheme {
            side: String::from("h-full w-80 top-0 left-0 transition-all duration-500 ease-out"),
            minimized: String::from("-translate-x-full"),
            maximized: String::from("translate-x-0"),
        };

        let right_side = DrawerSideTheme {
            side: String::from("h-full w-80 top-0 right-0 transition-all duration-500 ease-out"),
            minimized: String::from("translate-x-full"),
            maximized: String::from("translate-x-0"),
        };

        let top_side = DrawerSideTheme {
            side: String::from("w-full h-40 top-0 right-0 left-0 transition-all duration-500 ease-out"),
            minimized: String::from("-translate-y-full"),
            maximized: String::from("translate-y-0"),
        };

        let bottom_side = DrawerSideTheme {
            side: String::from("w-full h-40 bottom-0 right-0 left-0 transition-all duration-500 ease-out"),
            minimized: String::from("translate-y-full"),
            maximized: String::from("translate-y-0"),
        };

        let bg_gradient = Gradient::Linear(Direction::Left)
            .make("bg".into(), 
                GradientColors::from(TWColorPalette::from(TWColor::Fuchsia, Palette::S50), TWColorPalette::from(TWColor::Indigo, Palette::S50).into(), TWColorPalette::from(TWColor::Cyan, Palette::S50)), 
                GradientColors::from(TWColorPalette::from(TWColor::Fuchsia, Palette::S950), TWColorPalette::from(TWColor::Indigo, Palette::S950).into(), TWColorPalette::from(TWColor::Cyan, Palette::S950))
            );

        let drawer_theme = DrawerTheme {
            container: format!("{bg_gradient} z-[101] fixed overflow-x-hidden"),
            left_side,
            right_side,
            top_side,
            bottom_side 
        };
    
        drawer_theme
    }

    pub fn variant(color: Option<Color>, size: Option<Size>) -> DrawerTheme{
        
        let mut theme = Self::base_theme().clone();
       
        // Self::apply_color(&mut theme, color.unwrap_or_default());
        // Self::apply_size(&mut theme, size.unwrap_or_default());

        theme
    }
}