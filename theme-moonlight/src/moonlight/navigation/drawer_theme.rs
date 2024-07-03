use leptail::navigation::DrawerBaseTheme;
use tailwind_fuse::tw_merge;

use crate::moonlight::*;

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct TemporaryDrawer {
    size: Size,
    side: Side,
    has_inset: bool,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct ResponsiveDrawer {
    side: HorizontalSide,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct StaggeredDrawer {
    breakover_point: Size,
    side: HorizontalSide,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct StaggeredMiniDrawer {
    breakover_point: Size,
    side: HorizontalSide,
}

impl TemporaryDrawer {
    pub fn size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }

    pub fn side(mut self, side: Side) -> Self {
        self.side = side;
        self
    }

    pub fn has_inset(mut self, has_inset: bool) -> Self {
        self.has_inset = has_inset;
        self
    }

    pub fn as_drawer_variant(self) -> DrawerVariant {
        DrawerVariant::Temporary(self)
    }

    pub fn build(self) -> DrawerTheme {
        let bg_gradient = DrawerVariant::background();

        let height = DrawerVariant::drawer_height(&self.size);
        let width = DrawerVariant::drawer_width(&self.size);
        let inset_class = if self.has_inset { "p-0 md:p-5" } else { "" };
        let inset_inner_class = if self.has_inset {
            "p-2 md:p-3 md:rounded-lg"
        } else {
            ""
        };

        let base_wrapper_side = match self.side {
            Side::Left => tw_merge!("h-full", width, "top-0 left-0"),
            Side::Right => tw_merge!("h-full", width, "top-0 right-0"),
            Side::Top => tw_merge!(height, "w-full", "top-0 right-0 left-0"),
            Side::Bottom => tw_merge!(height, "w-full", "bottom-0 right-0 left-0"),
        };

        let minimized_wrapper_side = match self.side {
            Side::Left => "-translate-x-full",
            Side::Right => "translate-x-full",
            Side::Top => "-translate-y-full",
            Side::Bottom => "translate-y-full",
        };

        let maximized_wrapper_side = match self.side {
            Side::Left => "translate-x-0",
            Side::Right => "translate-x-0",
            Side::Top => "translate-y-0",
            Side::Bottom => "translate-y-0",
        };

        let base = DrawerBaseTheme {
            wrapper: tw_merge!(
                "z-[101] fixed overflow-x-hidden transition-all duration-300 ease-out",
                base_wrapper_side,
                inset_class,
            ),
            inner: tw_merge!("h-full w-full", bg_gradient, inset_inner_class),
        };

        let minimized_modifier = DrawerBaseTheme {
            wrapper: tw_merge!(minimized_wrapper_side),
            inner: String::default(),
        };
        let maximized_modifier = DrawerBaseTheme {
            wrapper: tw_merge!(maximized_wrapper_side),
            inner: String::default(),
        };

        DrawerTheme {
            base,
            minimized_modifier,
            maximized_modifier,
            overlay_theme: None,
        }
    }
}

impl ResponsiveDrawer {
    pub fn side(mut self, side: HorizontalSide) -> Self {
        self.side = side;
        self
    }

    pub fn as_drawer_variant(self) -> DrawerVariant {
        DrawerVariant::Responsive(self)
    }

    pub fn build(self) -> DrawerTheme {
        let width = "w-80";
        let bg_gradient = DrawerVariant::background();

        let base_wrapper_side = match self.side {
            HorizontalSide::Left => "top-0 left-0 md:top-auto md:left-auto",
            HorizontalSide::Right => "top-0 right-0 md:top-auto md:right-auto",
        };

        let minimized_wrapper_side = match self.side {
            HorizontalSide::Left => "-translate-x-full md:translate-x-0",
            HorizontalSide::Right => "translate-x-full md:translate-x-0",
        };

        let maximized_wrapper_side = "translate-x-0";

        let base = DrawerBaseTheme {
            wrapper: tw_merge!(
                "z-[101] md:z-[1] fixed md:relative overflow-x-hidden md:overflow-x-visible transition-all duration-300 ease-out  h-full flex flex-col shrink-0",
                "h-full",
                width,
                base_wrapper_side,
            ),
            inner: tw_merge!("md:bg-transparent dark:md:bg-transparent shrink-0", "h-full", width, bg_gradient),
        };

        let minimized_modifier = DrawerBaseTheme {
            wrapper: tw_merge!(minimized_wrapper_side),
            inner: String::default(),
        };
        let maximized_modifier = DrawerBaseTheme {
            wrapper: tw_merge!(maximized_wrapper_side),
            inner: String::default(),
        };

        let overlay_theme = DrawerVariant::responsive_overlay();
        // let overlay_theme = OverlayVariant::default_variant();

        DrawerTheme {
            base,
            minimized_modifier,
            maximized_modifier,
            overlay_theme: Some(overlay_theme),
        }
    }
}

impl StaggeredDrawer {
    pub fn breakover_point(mut self, breakover_point: Size) -> Self {
        self.breakover_point = breakover_point;
        self
    }

    pub fn side(mut self, side: HorizontalSide) -> Self {
        self.side = side;
        self
    }

    pub fn as_drawer_variant(self) -> DrawerVariant {
        DrawerVariant::Staggered(self)
    }

    pub fn build(self) -> DrawerTheme {
        DrawerVariant::build_staggered_drawer(&self.side, false)
    }
}

impl StaggeredMiniDrawer {
    pub fn breakover_point(mut self, breakover_point: Size) -> Self {
        self.breakover_point = breakover_point;
        self
    }

    pub fn side(mut self, side: HorizontalSide) -> Self {
        self.side = side;
        self
    }

    pub fn as_drawer_variant(self) -> DrawerVariant {
        DrawerVariant::StaggeredMini(self)
    }

    pub fn build(self) -> DrawerTheme {
        DrawerVariant::build_staggered_drawer(&self.side, true)
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum DrawerVariant {
    Temporary(TemporaryDrawer),
    Responsive(ResponsiveDrawer),
    Staggered(StaggeredDrawer),
    StaggeredMini(StaggeredMiniDrawer),
}

impl DrawerVariant {
    /// Temporary drawer, opens temporarily above all other content.
    /// The Drawer can be cancelled by clicking the overlay or pressing the Esc key.
    /// Ideally used for varity of purposes like custom built caputruing user controled inputs such as form, dialogs etc.  
    pub fn temporary() -> TemporaryDrawer {
        TemporaryDrawer {
            size: Size::default(),
            side: Side::Left,
            has_inset: false,
        }
    }

    /// Responsive works as like temporary drawer in smaller screen(unitl a breakover_point).
    /// On the larger screen, the drawer sits on the same surface elevation as the content.
    /// Usually used in custombuilt navigation bars!
    pub fn responsive() -> ResponsiveDrawer {
        ResponsiveDrawer {
            side: HorizontalSide::Left,
        }
    }

    /// Staggered is an extension of reponsive drawer where on the larger screen,
    /// the drawer can be collapsed to give focus on the contnet instead of naviations menus
    pub fn staggered() -> StaggeredDrawer {
        StaggeredDrawer {
            breakover_point: Size::Large,
            side: HorizontalSide::Left,
        }
    }

    /// StaggeredMini is an extension of staggered drawer when the drawer is hidden
    /// it still stakes up a minimum space which is visible even when the drawer is closed on larger screens.
    /// The mini variant is recommended for apps sections that need quick selection access alongside content.
    pub fn staggered_mini() -> StaggeredMiniDrawer {
        StaggeredMiniDrawer {
            breakover_point: Size::Large,
            side: HorizontalSide::Left,
        }
    }

    pub fn build(self) -> DrawerTheme {
        match self {
            DrawerVariant::Temporary(drawer) => drawer.build(),
            DrawerVariant::Responsive(drawer) => drawer.build(),
            DrawerVariant::Staggered(drawer) => drawer.build(),
            DrawerVariant::StaggeredMini(drawer) => drawer.build(),
        }
    }

    pub fn default_variant() -> DrawerTheme {
        DrawerVariant::temporary().build()
    }

    fn build_staggered_drawer(side: &HorizontalSide, mini_drawer: bool) -> DrawerTheme {
        let minimized_width = if mini_drawer {
            "w-0 md:w-[48px]"
        } else {
            "w-0"
        };
        let translate_x = if mini_drawer {
            "-translate-x-[320px] md:-translate-x-[0px] "
        } else {
            "-translate-x-[320px] "
        };
        let maximized_width = "w-[320px]";

        let bg_gradient = Self::background();

        let base_wrapper_side = match side {
            HorizontalSide::Left => "top-0 left-0 md:top-auto md:left-auto",
            HorizontalSide::Right => "top-0 right-0 md:top-auto md:right-auto",
        };

        let base = DrawerBaseTheme {
            wrapper: tw_merge!(
                "z-[101] md:z-[1] fixed md:relative transition-all duration-300 ease-out h-full flex flex-col shrink-0 overflow-x-hidden",
                "h-full", 
                base_wrapper_side,
            ),
            inner: tw_merge!("md:bg-transparent dark:md:bg-transparent shrink-0", bg_gradient),
        };

        let minimized_modifier = DrawerBaseTheme {
            wrapper: tw_merge!("overflow-x-visible", translate_x, minimized_width),
            inner: String::default(),
        };
        let maximized_modifier = DrawerBaseTheme {
            wrapper: tw_merge!("overflow-x-visible translate-x-0", maximized_width),
            inner: String::default(),
        };

        let overlay_theme = Self::responsive_overlay();
        // let overlay_theme = OverlayVariant::default_variant();

        DrawerTheme {
            base,
            minimized_modifier,
            maximized_modifier,
            overlay_theme: Some(overlay_theme),
        }
    }

    fn responsive_overlay() -> OverlayTheme {
        OverlayTheme {
            wrapper: "relative md:hidden".to_string(),
            inner: "md:hidden z-[100] transition duration-300 fixed inset-0 bg-gray-900 bg-opacity-50 dark:bg-opacity-80 hs-overlay-backdrop".to_string() 
        }
    }

    fn background() -> String {
        String::from("bg-slate-50 dark:bg-slate-900 text-slate-950 dark:text-slate-50")
    }

    fn drawer_width(size: &Size) -> &str {
        match size {
            Size::XSmall => "w-40",
            Size::Small => "w-60",
            Size::Medium => "w-80",
            Size::Large => "w-96",
            Size::XLarge => "w-1/2",
        }
    }

    fn drawer_height(size: &Size) -> &str {
        match size {
            Size::XSmall => "h-40",
            Size::Small => "h-40",
            Size::Medium => "h-40",
            Size::Large => "h-60",
            Size::XLarge => "h-80",
        }
    }
}
