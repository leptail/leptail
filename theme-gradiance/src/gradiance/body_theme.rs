use crate::gradiance::*;

pub struct BodyTheme {
    body: String,
}

impl BodyTheme {
    pub fn get_body(&self) -> &str {
        self.body.as_str()
    }
    pub fn default_variant() -> Self {
        let bg_gradient = Gradient::Radial(None).make(
            "bg".into(),
            GradientColors::from(
                TWColorPalette::from(TWColor::Indigo, Palette::S50),
                TWColorPalette::from(TWColor::Blue, Palette::S50).into(),
                TWColorPalette::from(TWColor::Violet, Palette::S50),
            ),
            GradientColors::from(
                TWColorPalette::from(TWColor::Indigo, Palette::S950),
                TWColorPalette::from(TWColor::Blue, Palette::S950).into(),
                TWColorPalette::from(TWColor::Violet, Palette::S950),
            ),
        );
        BodyTheme {
            body: format!(
                "min-h-screen text-slate-950 dark:text-slate-50 {}",
                bg_gradient
            ),
        }
    }
}
