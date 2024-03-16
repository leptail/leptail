use leptail::prelude::*;
use leptail_tailwind_utils::*;
use crate::SwitchVariant;

use self::navigation::drawer_theme::DrawerVariant;

pub mod input; 
pub mod navigation;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Default)]
pub enum Size{
    XSmall,
    Small,
    #[default]
    Medium,
    Large,
    XLarge, 
}




#[derive(Debug, Clone, Copy, Eq, PartialEq, Default)] 
pub enum Color{
    #[default]
    Default,
    Primary,
    Secondary,
    Info,
    Success,
    Warning,
    Danger
}


impl Color {
    pub fn to_tw_color(&self) -> TWColor {
        match self {
            Color::Default => TWColor::Slate,
            Color::Primary => TWColor::Cyan,
            Color::Secondary => TWColor::Fuchsia,
            Color::Info => TWColor::Sky,
            Color::Success => TWColor::Emerald,
            Color::Warning => TWColor::Amber,
            Color::Danger => TWColor::Rose,
        }
    }
    pub fn base(&self) -> String {
        self.to_tw_color().tw_string()
    }
    pub fn make_shade(&self, prefix: &str, shade: Palette) -> String {
        format!("{}-{}-{}", prefix, self.base(), shade.tw_string())
    }
}
 
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Top,
    TopRight,
    Right,
    BottomRight,
    Bottom,
    BottomLeft,
    Left,
    TopLeft
}

 
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Side { 
    Left,
    Right,
    Top,
    Bottom,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HorizontalSide { 
    Left,
    Right, 
}


#[derive(Debug, Clone, PartialEq)]
pub struct GradientColors {
    from: TWColorPalette, 
    via: Option<TWColorPalette>, 
    to: TWColorPalette
}

impl GradientColors {

    pub fn from(from: TWColorPalette, via: Option<TWColorPalette>, to: TWColorPalette) -> Self {
        GradientColors { from, via, to }
    }

    pub fn tw_string(&self, prefix: Option<String>) -> String {
        let from_color = format!("from-{}", self.from.tw_string());
        let to_color = format!("to-{}", self.to.tw_string());
        let via_color = format!("via-{}", self.via.to_owned().map_or("".into(), |v| v.tw_string()));
        let prefix: String = prefix.unwrap_or("".into());
        format!("{}{} {}{} {}{}", prefix, from_color, prefix, via_color, prefix, to_color)
    }
}


#[derive(Debug, Clone, PartialEq)]
pub enum Gradient{
    Linear(Direction),
    Conic(Option<Direction>),
    Radial(Option<Direction>)
}


impl Gradient {
    pub fn make(&self, prefix: String, light_colors: GradientColors, dark_colors: GradientColors) -> String {
        // let from_color = format!("from-")
        // let colors = format!("{} {} {}", )
        let gradient: String = match  self {
            Gradient::Linear(direction) => match direction {
                Direction::Top => "gradient-to-t".into(),
                Direction::TopRight => "gradient-to-tr".into(),
                Direction::Right => "gradient-to-r".into(),
                Direction::BottomRight => "gradient-to-br".into(),
                Direction::Bottom => "gradient-to-b".into(),
                Direction::BottomLeft => "gradient-to-bl".into(),
                Direction::Left => "gradient-to-l".into(),
                Direction::TopLeft => "gradient-to-tl".into(),
            },

            Gradient::Conic(direction_opt) => match direction_opt {
                Some(direction) => match direction {
                    Direction::Top => "[conic-gradient(at_top,_var(--tw-gradient-stops))]".into(),
                    Direction::TopRight => "[conic-gradient(at_top_right,_var(--tw-gradient-stops))]".into(),
                    Direction::Right => "[conic-gradient(at_right,_var(--tw-gradient-stops))]".into(),
                    Direction::BottomRight => "[conic-gradient(at_bottom_right,_var(--tw-gradient-stops))]".into(),
                    Direction::Bottom => "[conic-gradient(at_bottom,_var(--tw-gradient-stops))]".into(),
                    Direction::BottomLeft => "[conic-gradient(at_bottom_left,_var(--tw-gradient-stops))]".into(),
                    Direction::Left => "[conic-gradient(at_left,_var(--tw-gradient-stops))]".into(),
                    Direction::TopLeft => "[conic-gradient(at_top_left,_var(--tw-gradient-stops))]".into(),
                },
                None => "[conic-gradient(var(--tw-gradient-stops))]".into(),
            },

            Gradient::Radial(direction_opt) => match direction_opt {
                Some(direction) => match direction {
                    Direction::Top => "[radial-gradient(ellipse_at_top,_var(--tw-gradient-stops))]".into(),
                    Direction::TopRight => "[radial-gradient(ellipse_at_top_right,_var(--tw-gradient-stops))]".into(),
                    Direction::Right => "[radial-gradient(ellipse_at_right,_var(--tw-gradient-stops))]".into(),
                    Direction::BottomRight => "[radial-gradient(ellipse_at_bottom_right,_var(--tw-gradient-stops))]".into(),
                    Direction::Bottom => "[radial-gradient(ellipse_at_bottom,_var(--tw-gradient-stops))]".into(),
                    Direction::BottomLeft => "[radial-gradient(ellipse_at_bottom_left,_var(--tw-gradient-stops))]".into(),
                    Direction::Left => "[radial-gradient(ellipse_at_left,_var(--tw-gradient-stops))]".into(),
                    Direction::TopLeft => "[radial-gradient(ellipse_at_top_left,_var(--tw-gradient-stops))]".into(),
                },
                // bg-[radial-gradient(ellipse_at_center,_var(--tw-gradient-stops))] from-green-300 via-blue-500 to-purple-600
                // bg-[radial-gradient(ellipse_at_center,_var(--tw-gradient-stops))] from-emerald-400 via-emerald-400 to-emerald-300 dark:from-emerald-600 dark:via-emerald-600 dark:to-emerald-700
                None => "[radial-gradient(ellipse_at_center,_var(--tw-gradient-stops))]".into(),
            },
        };
        format!("{}-{} {} {}", prefix, gradient, light_colors.tw_string(None), dark_colors.tw_string(Some("dark:".into())))
    }
}


pub fn build_theme() -> AppTheme {

    let bg_gradient = Gradient::Radial(None)
        .make("bg".into(), 
            GradientColors::from(TWColorPalette::from(TWColor::Indigo, Palette::S50), TWColorPalette::from(TWColor::Blue, Palette::S50).into(), TWColorPalette::from(TWColor::Violet, Palette::S50)), 
            GradientColors::from(TWColorPalette::from(TWColor::Indigo, Palette::S950), TWColorPalette::from(TWColor::Blue, Palette::S950).into(), TWColorPalette::from(TWColor::Violet, Palette::S950))
        );

    let appbar_theme = AppbarTheme {
        nav: format!("{} text-blue-900 dark:text-blue-300", bg_gradient),
        nav_inner: "mx-auto max-w-screen-2xl sm:px-6 md:px-0",
        lg_menu_container: "relative flex h-16 items-center justify-between",
        sm_menu_container: "space-y-1 px-2 pb-3 pt-2",
        hamburger_container: "left-0 flex items-center sm:hidden",
        hamburger_button: "relative inline-flex items-center justify-center rounded-md p-2 text-gray-400 hover:bg-gray-700 hover:text-white focus:outline-none focus:ring-2 focus:ring-inset focus:ring-white",
        left_side_container: "flex flex-1 items-stretch justify-start",
        right_side_container: "absolute inset-y-0 right-0 flex items-center pr-2 sm:static sm:inset-auto sm:ml-6 sm:pr-0",
        logo: "flex flex-shrink-0 items-center",
        menu: "flex space-x-4",
    };

    let drawer_theme = DrawerVariant::default();

    let overlay_theme = OverlayTheme {
        // overlay: "fixed inset-0 z-100 h-full w-full backdrop-blur-xs bg-gray-900 bg-opacity-50 dark:bg-opacity-80 ",
        overlay: "z-[100] transition duration-500 fixed inset-0 bg-gray-900 bg-opacity-50 dark:bg-opacity-80 hs-overlay-backdrop" 
    }; 

    let base_switch_theme = SwitchVariant::variant(None, None);

    AppTheme { 
        body: format!("min-h-screen text-slate-950 dark:text-slate-50 {}", bg_gradient),
        appbar: appbar_theme, 
        drawer: drawer_theme, 
        overlay: overlay_theme, 
        switch: base_switch_theme,
    } 
}

