use leptail_build::{copy_theme_sources, LeptailThemes};

fn main() {
    copy_theme_sources(&LeptailThemes::Moonlight);

    copy_theme_sources(&LeptailThemes::Gradiance);

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=Cargo.lock");
}
