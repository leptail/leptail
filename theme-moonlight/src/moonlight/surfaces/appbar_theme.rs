// use navigation::drawer_theme::DrawerVariant;
use tailwind_fuse::tw_merge;

use crate::moonlight::{*};
use crate::moonlight::navigation::drawer_theme::DrawerVariant;

pub struct AppbarVariant {
    is_sticky: bool,
    shadow: Option<Size>,
    /// None being transparent background
    bg_color: Option<Color>,
    /// None means, full size 
    max_width: Option<Size>,
    drawer_variant: Option<DrawerVariant>,
}

impl AppbarVariant {

    pub fn builder() -> AppbarVariant {
        AppbarVariant{
            is_sticky: false,
            shadow: None,
            bg_color: None,
            max_width: None,
            drawer_variant: None,
        }
    }    

    pub fn sticky(mut self, is_sticky: bool) -> AppbarVariant{
        self.is_sticky = is_sticky;
        self
    } 

    pub fn shadow(mut self, size: Option<Size>) -> AppbarVariant{
        self.shadow = size;
        self
    }

    pub fn bg_color(mut self, color: Option<Color>) -> AppbarVariant{
        self.bg_color = color;
        self
    } 

    pub fn max_width(mut self, size: Option<Size>) -> AppbarVariant{
        self.max_width = size;
        self
    }

    pub fn drawer_variant(mut self, variant: DrawerVariant) -> AppbarVariant{
        self.drawer_variant = Some(variant);
        self
    } 


    pub fn default_variant() -> AppbarTheme {
        AppbarVariant::builder()
            .sticky(true)
            .shadow(Size::Large.into())
            .bg_color(Color::Primary.into())
            .max_width(Size::XLarge.into())
            .drawer_variant(DrawerVariant::Temporary { size: Size::Medium, side: Side::Left, has_inset: false })
            .build()
        // AppbarVariant::builder().build()
    }

    pub fn build(self) -> AppbarTheme {

        // let bg_gradient = Gradient::Radial(None)
        //     .make("bg".into(), 
        //         GradientColors::from(TWColorPalette::from(TWColor::Indigo, Palette::S50), TWColorPalette::from(TWColor::Blue, Palette::S50).into(), TWColorPalette::from(TWColor::Violet, Palette::S50)), 
        //         GradientColors::from(TWColorPalette::from(TWColor::Indigo, Palette::S950), TWColorPalette::from(TWColor::Blue, Palette::S950).into(), TWColorPalette::from(TWColor::Violet, Palette::S950))
        //     );
        // let bg_gradient = "";

        let layout = tw_merge!("relative");
        let appbar_container = tw_merge!(
            "px-1 py-1 md:px-0",
            "border-b border-purple-50 dark:border-purple-950",
            if self.is_sticky { "sticky top-0 z-[1] md:z-[102]" } else { "" },
            match self.shadow {
                Some(shadow) => match shadow {
                    Size::XSmall => "drop-shadow-sm",
                    Size::Small => "drop-shadow",
                    Size::Medium => "drop-shadow-md",
                    Size::Large => "drop-shadow-lg",
                    Size::XLarge => "drop-shadow-xl",
                },
                None => "",
            },
            // TODO: add colour  
            match self.bg_color  {
                Some(color) => tw_merge!(color.make_shade("bg", Palette::S100), color.make_shade("dark:bg", Palette::S900)),
                None => "bg-transparent backdrop-blur-sm".to_string(),
            }, 
        );

        let appbar_inner = tw_merge!(
            "relative flex flex-row space-x-2 h-16 mx-auto", 
            match self.max_width {
                Some(width) => match width {
                    Size::XSmall => "max-w-screen-sm",
                    Size::Small => "max-w-screen-sm",
                    Size::Medium => "max-w-screen-md",
                    Size::Large => "max-w-screen-lg",
                    Size::XLarge => "max-w-screen-xl",
                },
                // TODO: add fullwidth case. 
                None => "",
            }
        );

        let drawer_container = tw_merge!(
            "flex flex-row", 
            match self.max_width {
                Some(width) => match width {
                    Size::XSmall => "mx-auto max-w-screen-sm",
                    Size::Small => "mx-auto max-w-screen-sm",
                    Size::Medium => "mx-auto max-w-screen-md",
                    Size::Large => "mx-auto max-w-screen-lg",
                    Size::XLarge => "mx-auto max-w-screen-xl",
                },
                // TODO: add fullwidth case. 
                None => "",
            }
        );

        let default_drawer_variant = DrawerVariant::Temporary { size: Size::Medium, side: Side::Left, has_inset: false };
            
        

        AppbarTheme {
            layout,
            appbar_container,
            appbar_inner,
            toolbar: "flex space-x-4".to_string(),
            hamburger_icon: icondata::ChMenuHamburger,
            close_icon: icondata::CgClose,  
            hamburger_container: "left-0 flex items-center md:hidden".to_string(),
            hamburger_button: "relative inline-flex items-center justify-center rounded-md p-2 text-gray-400 hover:bg-gray-700 hover:text-white focus:outline-none focus:ring-2 focus:ring-inset focus:ring-white".to_string(),
            drawer_title_wrapper: "flex flex-row md:hidden".to_string(),
            drawer_title: "".to_string(),
            drawer_container,
            main_content: "w-full".to_string(),
            drawer_variant: DrawerVariant::variant(&self.drawer_variant.unwrap_or(default_drawer_variant))
        }
    }

}