use crate::moonlight::*;

pub struct OverlayVariant;

impl OverlayVariant {
    pub fn default_variant() -> OverlayTheme {
        OverlayTheme {
            // overlay: "fixed inset-0 z-100 h-full w-full backdrop-blur-xs bg-gray-900 bg-opacity-50 dark:bg-opacity-80 ",
            wrapper: "relative".to_string(),
            inner: "z-[100] transition duration-500 fixed inset-0 bg-gray-900 bg-opacity-50 dark:bg-opacity-80 hs-overlay-backdrop".to_string() 
        }
    }
}
