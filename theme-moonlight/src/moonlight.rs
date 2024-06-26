use leptail::prelude::*;
use leptail_tailwind_utils::*;

use crate::SwitchVariant;

pub mod body_theme;
pub mod input;
pub mod navigation;
pub mod overlay_theme;
pub mod surfaces;

//TODO: following are the common accross gradiance and moonlight theme.
// Move them to single crate to maintain the parity with each of the leptail themes.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Default)]
pub enum Size {
    XSmall,
    Small,
    #[default]
    Medium,
    Large,
    XLarge,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Default)]
pub enum Color {
    #[default]
    Default,
    Primary,
    Secondary,
    Info,
    Success,
    Warning,
    Danger,
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
    TopLeft,
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
// TODO: end of reafactoring block

// #[macro_export]
// macro_rules! make_shade {
//     ($prefix:expr, $color:expr, $shade:expr) => {{
//         let color_name = match $color {
//             Color::Default => "slate",
//             Color::Primary => "cyan",
//             Color::Secondary => "fuchsia",
//             Color::Info => "sky",
//             Color::Success => "emerald",
//             Color::Warning => "amber",
//             Color::Danger => "rose",
//         };

//         let shade_name = match $shade {
//             ColorShade::S50 => "50",
//             ColorShade::S100 => "100",
//             ColorShade::S200 => "200",
//             ColorShade::S300 => "300",
//             ColorShade::S400 => "400",
//             ColorShade::S500 => "500",
//             ColorShade::S600 => "600",
//             ColorShade::S700 => "700",
//             ColorShade::S800 => "800",
//             ColorShade::S900 => "900",
//             ColorShade::S950 => "950",
//         };

//         format!("{}-{}-{}", $prefix, color_name, shade_name)
//     }}
// }

// pub fn build_theme() -> AppTheme {

//     let appbar_theme = AppbarTheme {
//         layout: "bg-slate-200 dark:bg-slate-950 text-blue-900 dark:text-blue-300".into(),
//         appbar_container: "mx-auto max-w-screen-xl sm:px-6 md:px-0".to_string(),
//         appbar_inner: "relative flex h-16 items-center justify-between".to_string(),
//         hamburger_icon: icondata::ChMenuHamburger,
//         close_icon: icondata::CgClose,
//         hamburger_container: "left-0 flex items-center sm:hidden".to_string(),
//         hamburger_button: "relative inline-flex items-center justify-center rounded-md p-2 text-gray-400 hover:bg-gray-700 hover:text-white focus:outline-none focus:ring-2 focus:ring-inset focus:ring-white".to_string(),
//         toolbar: "flex space-x-4".to_string(),
//         drawer_title_wrapper: "".to_string(),
//         drawer_title: "".to_string(),
//         drawer_container: "".to_string(),
//         main_content: "".to_string(),
//         drawer_variant: Default::default(),
//     };

//     let drawer_theme = DrawerTheme::default();

//     let overlay_theme = OverlayTheme {
//         wrapper: "relative".to_string(),
//         inner: "fixed inset-0 z-100 h-full w-full backdrop-blur-xs bg-gray-900 bg-opacity-50 dark:bg-opacity-80 ".to_string(),
//     };

//     let base_switch_theme = SwitchVariant::variant(None, None);

//     AppTheme {
//         body: String::from("bg-slate-50 dark:bg-slate-900 text-slate-950 dark:text-slate-50"),
//         appbar: appbar_theme,
//         drawer: drawer_theme,
//         overlay: overlay_theme,
//         switch: base_switch_theme,
//     }
// }
