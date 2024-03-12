use camino::Utf8PathBuf;
use cargo_leptos::ext::PathBufExt;
use cargo_leptos::{
    compile::{self, ChangeSet},
    config::{Cli, Config},
};
use clap_builder::Parser;

const BUILD_LOCK_ENV: &str = "__BUILD_LOCK__";

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=Cargo.lock");

    if std::env::var("SHUTTLE").is_err() || std::env::var(BUILD_LOCK_ENV).is_ok() {
        return;
    }

    std::env::set_var(BUILD_LOCK_ENV, "true");

    // technically should check if release/debug but since we are only running on shuttle we just force release
    let args = vec!["leptos", "build", "--release"].into_iter().map(str::to_string);
    let args = Cli::parse_from(args);

    let manifest_path = Utf8PathBuf::from("Cargo.toml").resolve_home_dir().unwrap();
    let mut cwd = Utf8PathBuf::from_path_buf(std::env::current_dir().unwrap()).unwrap();
    cwd.clean_windows_path();

    let opts = args.opts().unwrap();

    let config = Config::load(opts, &cwd, &manifest_path, false).unwrap();

    let proj = config.current_project().unwrap();
    let changes = ChangeSet::all_changes();

    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async move {
            let front_outcome = compile::front(&proj, &changes)
                .await
                .await
                .unwrap()
                .unwrap();
            assert!(front_outcome.is_success());

            let asset_outcome = compile::assets(&proj, &changes, true)
                .await
                .await
                .unwrap()
                .unwrap();
            assert!(asset_outcome.is_success());

            let style_outcome = compile::style(&proj, &changes)
                .await
                .await
                .unwrap()
                .unwrap();
            assert!(style_outcome.is_success());
        })
}