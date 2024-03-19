use crate::gradiance::*;


#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DrawerVariant {
    /// Temporary drawer, opens temporarily above all other content. 
    /// The Drawer can be cancelled by clicking the overlay or pressing the Esc key. 
    /// Ideally used for varity of purposes like custom built caputruing user controled inputs such as form, dialogs etc.  
    Temporary{size: Size, side: Side, has_inset: bool}, 
    /// Responsive works as like temporary drawer in smaller screen(unitl a breakover_point).
    /// On the larger screen, the drawer sits on the same surface elevation as the content. 
    /// Usually used in custombuilt navigation bars!
    Responsive{side: HorizontalSide}, 
    /// Staggered is an extension of reponsive drawer where on the larger screen, 
    /// the drawer can be collapsed to give focus on the contnet instead of naviations menus  
    Staggered{breakover_point: Size, side: HorizontalSide}, 
    /// StaggeredMini is an extension of staggered drawer when the drawer is hidden 
    /// it still stakes up a minimum space which is visible even when the drawer is closed on larger screens. 
    /// The mini variant is recommended for apps sections that need quick selection access alongside content. 
    StaggeredMini{breakover_point: Size, side: HorizontalSide}, 
}


impl DrawerVariant {

    fn background() -> String {
        // let bg_gradient = Gradient::Linear(Direction::Left)
        //     .make("bg".into(), 
        //         GradientColors::from(TWColorPalette::from(TWColor::Blue, Palette::S50), TWColorPalette::from(TWColor::Indigo, Palette::S50).into(), TWColorPalette::from(TWColor::Cyan, Palette::S50)), 
        //         GradientColors::from(TWColorPalette::from(TWColor::Blue, Palette::S950), TWColorPalette::from(TWColor::Indigo, Palette::S950).into(), TWColorPalette::from(TWColor::Cyan, Palette::S950))
        //     );

        Gradient::Radial(None)
            .make("bg".into(), 
                GradientColors::from(TWColorPalette::from(TWColor::Indigo, Palette::S50), TWColorPalette::from(TWColor::Blue, Palette::S50).into(), TWColorPalette::from(TWColor::Violet, Palette::S50)), 
                GradientColors::from(TWColorPalette::from(TWColor::Indigo, Palette::S950), TWColorPalette::from(TWColor::Blue, Palette::S950).into(), TWColorPalette::from(TWColor::Violet, Palette::S950))
            )
    }

    fn base_theme() -> DrawerTheme{

        let bg_gradient = Self::background();
        
        let base_theme = DrawerTheme {
            wrapper: "z-[101] fixed overflow-x-hidden transition-all duration-500 ease-out".to_string(),
            inner: format!("{bg_gradient}"),
            minimized: String::from(""),
            maximized: String::from(""), 
            overlay_theme: None,
        };

        base_theme
    }

    fn drawer_width(size: &Size) -> String {
        match  size {
            Size::XSmall => String::from("w-40"), 
            Size::Small => String::from("w-60"), 
            Size::Medium => String::from("w-80"), 
            Size::Large => String::from("w-96"),
            Size::XLarge => String::from("w-1/2"),
        }
    }

    fn drawer_height(size: &Size) -> String {
        match size {
            Size::XSmall => String::from("h-40"), 
            Size::Small => String::from("h-40"), 
            Size::Medium => String::from("h-40"), 
            Size::Large => String::from("h-60"),
            Size::XLarge => String::from("h-80"),
        }
    }

    fn gen_temprovary_drawer(side: &Side, size: &Size, has_inset: bool) -> DrawerTheme {
        let height = Self::drawer_height(&size);
        let width = Self::drawer_width(&size);
        let inset_class = if has_inset { "p-0 md:p-5" } else {""};
        let inset_inner_class = if has_inset { "p-2 md:p-3 md:rounded-lg" } else {""};

        match side {
            Side::Left => DrawerTheme {
                wrapper: format!("h-full {width} top-0 left-0 {inset_class}"),
                inner: format!("h-full w-full {inset_inner_class}"),
                minimized: String::from("-translate-x-full"),
                maximized: String::from("translate-x-0"), 
                overlay_theme: None,
            },
            Side::Right => DrawerTheme {
                wrapper: format!("h-full {width} top-0 right-0 {inset_class}"),
                inner: format!("h-full w-full {inset_inner_class}"),
                minimized: String::from("translate-x-full"),
                maximized: String::from("translate-x-0"), 
                overlay_theme: None,
            },
            Side::Top => DrawerTheme {
                wrapper: format!("w-full {height} top-0 right-0 left-0 {inset_class}"),
                inner: format!("h-full w-full {inset_inner_class}"),
                minimized: String::from("-translate-y-full"),
                maximized: String::from("translate-y-0"), 
                overlay_theme: None,
            },
            Side::Bottom => DrawerTheme {
                wrapper: format!("w-full {height} bottom-0 right-0 left-0 {inset_class}"),
                inner: format!("h-full w-full {inset_inner_class}"),
                minimized: String::from("translate-y-full"),
                maximized: String::from("translate-y-0"), 
                overlay_theme: None,
            },
        }
    }


    fn gen_responsive_drawer(side: &HorizontalSide) -> DrawerTheme {
        let width = "w-80";

        let bg_gradient = Self::background();

        let base_theme = DrawerTheme {
            wrapper: "z-[101] md:z-[1] fixed md:relative overflow-x-hidden md:overflow-x-visible transition-all duration-500 ease-out  h-full flex flex-col shrink-0".to_string(),
            inner: format!("{bg_gradient} md:bg-transparent dark:md:bg-transparent shrink-0"),
            minimized: String::from(""),
            maximized: String::from(""), 
            overlay_theme: None,
        };

        let side_modifier = match side {
            HorizontalSide::Left => DrawerTheme {
                wrapper: format!("h-full {width} top-0 left-0 md:top-auto md:left-auto"),
                inner: format!("h-full {width}"),
                minimized: String::from("-translate-x-full md:translate-x-0"),
                maximized: String::from("translate-x-0"), 
                overlay_theme: None,
            },
            HorizontalSide::Right => DrawerTheme {
                wrapper: format!("h-full {width} top-0 right-0 md:top-auto md:left-auto"),
                inner: format!("h-full {width}"),
                minimized: String::from("translate-x-full md:translate-x-0"),
                maximized: String::from("translate-x-0"), 
                overlay_theme: None,
            }, 
        };

        let overlay_theme = OverlayTheme {
            wrapper: "relative md:hidden".to_string(),
            inner: "md:hidden z-[100] transition duration-500 fixed inset-0 bg-gray-900 bg-opacity-50 dark:bg-opacity-80 hs-overlay-backdrop".to_string() 
        }; 

        let mut theme = Self::merge(base_theme, side_modifier);
        theme.overlay_theme = Some(overlay_theme);
        theme
    }

    fn gen_staggered_drawer(side: &HorizontalSide, mini_drawer: bool) -> DrawerTheme {
        let minimized_width = if mini_drawer { "w-0 md:w-[48px]" } else { "w-0" };
        let translate_x = if mini_drawer { "-translate-x-[320px] md:-translate-x-[0px] " } else { "-translate-x-[320px] "};
        let maximized_width = "w-[320px]";
        

        let bg_gradient = Self::background();

        let base_theme = DrawerTheme {
            wrapper: "z-[101] md:z-[1] fixed md:relative transition-all duration-500 ease-out h-full flex flex-col shrink-0 overflow-x-hidden".to_string(),
            inner: format!("{bg_gradient} md:bg-transparent dark:md:bg-transparent shrink-0"),
            minimized: String::from(""),
            maximized: String::from(""), 
            overlay_theme: None,
        };

        let side_modifier = match side {
            HorizontalSide::Left => DrawerTheme {
                wrapper: format!("h-full top-0 left-0 md:top-auto md:left-auto"),
                inner: format!("h-full w-full flex flex-col"),
                minimized: format!("{translate_x} {minimized_width} overflow-x-visible"),
                maximized: format!("translate-x-0 {maximized_width} overflow-x-visible"), 
                overlay_theme: None,
            },
            HorizontalSide::Right => DrawerTheme {
                wrapper: format!("h-full top-0 right-0 md:top-auto md:left-auto"),
                inner: format!("h-full {maximized_width}"),
                minimized: format!("{translate_x} {minimized_width} overflow-x-visible"),
                maximized: format!("translate-x-0 {maximized_width} overflow-x-visible"), 
                overlay_theme: None,
            }, 
        };

        let overlay_theme = OverlayTheme {
            wrapper: "relative md:hidden".to_string(),
            inner: "md:hidden z-[100] transition duration-500 fixed inset-0 bg-gray-900 bg-opacity-50 dark:bg-opacity-80 hs-overlay-backdrop".to_string() 
        }; 

        let mut theme = Self::merge(base_theme, side_modifier);
        theme.overlay_theme = Some(overlay_theme);
        theme
    }


    fn merge(first_theme: DrawerTheme, second_theme: DrawerTheme) -> DrawerTheme { 
        DrawerTheme {
            wrapper: format!("{} {}", first_theme.wrapper, second_theme.wrapper),
            inner: format!("{} {}", first_theme.inner, second_theme.inner),
            minimized: format!("{} {}", first_theme.minimized, second_theme.minimized),
            maximized: format!("{} {}", first_theme.maximized, second_theme.maximized), 
            overlay_theme: None,
        }
    }

    pub fn default() -> DrawerTheme {
        let dv = DrawerVariant::Temporary { size: Size::Medium, side: Side::Left, has_inset: false };
        Self::variant(&dv)
    }

    // TODO: add variant: 
    pub fn variant(variant: &DrawerVariant) -> DrawerTheme{ 
         
        match variant {
            DrawerVariant::Temporary { size, side, has_inset } => {
                // TODO: simplify this 
                let base = Self::base_theme();
                let side_modified_theme = Self::gen_temprovary_drawer(side, size, *has_inset);
                Self::merge(base, side_modified_theme)
            }
            DrawerVariant::Responsive { side } => Self::gen_responsive_drawer(side),
            // TODO: check if breakover_point is needed!
            DrawerVariant::Staggered { breakover_point, side } => Self::gen_staggered_drawer(side, false),
            DrawerVariant::StaggeredMini { breakover_point, side } => Self::gen_staggered_drawer(side, true),
        }
    }
}

