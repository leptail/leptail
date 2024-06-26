use crate::moonlight::*;

pub struct BodyTheme {
    body: String,
}

impl BodyTheme {
    pub fn get_body(&self) -> &str {
        self.body.as_str()
    }
    pub fn default_variant() -> Self {
        BodyTheme {
            body: String::from("bg-slate-50 dark:bg-slate-900 text-slate-950 dark:text-slate-50"),
        }
    }
}
