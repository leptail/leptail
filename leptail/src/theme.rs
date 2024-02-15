use crate::prelude::*;


pub trait HasThemeVariant {
    fn default_variant() -> Self; 
}

#[derive(Debug, Clone, Default)]
pub struct AppTheme {
    pub body: String,
    pub appbar: AppbarTheme,
    pub drawer: DrawerTheme,
    pub overlay: OverlayTheme,
    pub switch: SwitchTheme,
}
