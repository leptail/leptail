use leptail::navigation::DrawerSideTheme;

use crate::gradiance::*;

pub struct DrawerVariant {}

impl DrawerVariant {

    pub fn base_theme() -> DrawerTheme{
        
        let left_side = DrawerSideTheme {
            side: String::from("h-full top-0 left-0 transition-all delay-100 duration-1000 ease-out"),
            minimized: String::from("w-0"),
            maximized: String::from("w-80"),
        };

        let right_side = DrawerSideTheme {
            side: String::from("h-full top-0 right-0 transition-all duration-1000 ease-out"),
            minimized: String::from("w-0"),
            maximized: String::from("w-80"),
        };

        let top_side = DrawerSideTheme {
            side: String::from("w-full top-0 right-0 left-0 transition-all duration-1000 ease-out"),
            minimized: String::from("h-0"),
            maximized: String::from("h-40"),
        };

        let bottom_side = DrawerSideTheme {
            side: String::from("w-full bottom-0 right-0 left-0 transition-all duration-1000 ease-out"),
            minimized: String::from("h-0"),
            maximized: String::from("h-40"),
        };

        let drawer_theme = DrawerTheme {
            container: String::from("bg-rose-500 fixed overflow-x-hidden"),
            left_side: left_side,
            right_side: right_side,
            top_side: top_side,
            bottom_side: bottom_side 
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