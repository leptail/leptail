use cargo::core::{PackageId, SourceId, SourceKind, Workspace};
use cargo::ops::{self};
use cargo::util::GlobalContext;
use log::debug;
use std::path::{Path, PathBuf};
use std::{cmp, env, fs, io};

pub enum LeptailThemes {
    Gradiance,
    Moonlight,
}

impl LeptailThemes {
    fn directory_for_git_repo(&self) -> &str {
        match &self {
            LeptailThemes::Gradiance => "theme-gradiance",
            LeptailThemes::Moonlight => "theme-moonlight",
        }
    }

    fn package_name(&self) -> &str {
        match &self {
            LeptailThemes::Gradiance => "leptail_theme_gradiance",
            LeptailThemes::Moonlight => "leptail_theme_moonlight",
        }
    }
}

pub fn copy_theme_sources(theme: &LeptailThemes) {
    // TODO: figureout a way to use logger!
    // env_logger::init();

    let config: GlobalContext = GlobalContext::default().expect("Failed to create Cargo config");
    let manifest_path =
        PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set"))
            .join("Cargo.toml");

    let workspace =
        Workspace::new(manifest_path.as_path(), &config).expect("Failed to create Cargo workspace");

    let src_path = get_dependency_source_path(&config, &workspace, theme)
        .expect("Failed to locate the source path of the dependency");
    debug!("Source path for {:?}: {:?}", theme.package_name(), src_path);

    let target_path = workspace
        .target_dir()
        .join("leptail")
        .join(theme.package_name())
        .into_path_unlocked();

    debug!("Target path {:?}", target_path);

    copy_dir_all(&src_path, &target_path).expect("Failed to copy dependency source code");
    debug!("Successfully copied {:?} to {:?}", src_path, target_path);
    // panic!()
}

fn get_dependency_source_path(
    global_context: &GlobalContext,
    workspace: &Workspace,
    theme: &LeptailThemes,
) -> Option<PathBuf> {
    let package_id = get_package_id(workspace, theme.package_name())
        .expect("Failed to find dependency in Cargo.lock");

    debug!(
        "Found dependency version: {}: {}",
        theme.package_name(),
        package_id
    );

    let source_id = package_id.source_id();

    match package_id.source_id().kind() {
        SourceKind::Git(_git_reference) => {
            let package_name = ident(&source_id);

            let short_id = source_id
                .precise_git_fragment()
                .map(|s| {
                    let s = s.to_string();
                    let len = cmp::min(s.len(), 7);
                    s[..len].to_string()
                })
                .expect("Could not find git precise rev");

            let package_path = global_context
                .git_checkouts_path()
                .join(package_name)
                .join(short_id)
                .join(theme.directory_for_git_repo())
                .join("src")
                .into_path_unlocked();

            Some(package_path)
        }
        SourceKind::Path => source_id.url().to_file_path().ok(),
        SourceKind::LocalRegistry => source_id.url().to_file_path().ok(),
        SourceKind::Directory => source_id.url().to_file_path().ok(),
        SourceKind::Registry => {
            let package_path = global_context
                .registry_source_path()
                // TODO: get the path from global_context instead of hardcoding
                .join("index.crates.io-6f17d22bba15001f")
                .join(format!("{}-{}", package_id.name(), package_id.version()))
                .join("src")
                .into_path_unlocked();
            Some(package_path)
        }
        SourceKind::SparseRegistry => todo!(),
    }
}

fn get_package_id(workspace: &Workspace, dep_name: &str) -> Option<PackageId> {
    let (package_set, resolve) = ops::resolve_ws(workspace).expect("Failed to resolve workspace");

    for package_id in resolve.iter() {
        let package_name = package_id.name().as_str();

        if package_name == dep_name {
            return Some(package_id);
        }
    }
    None
}

fn ident(id: &SourceId) -> String {
    let ident = id
        .canonical_url()
        .raw_canonicalized_url()
        .path_segments()
        .and_then(|s| s.rev().next())
        .unwrap_or("");

    let ident = if ident.is_empty() { "_empty" } else { ident };

    format!("{}-{}", ident, cargo::util::short_hash(id.canonical_url()))
}

/// Like [`ident()`], but appends `-shallow` to it, turning
/// `proto://host/path/repo` into `repo-<hash-of-url>-shallow`.
///
/// It's important to separate shallow from non-shallow clones for reasons of
/// backwards compatibility --- older cargo's aren't necessarily handling
/// shallow clones correctly.
fn ident_shallow(id: &SourceId, is_shallow: bool) -> String {
    let mut ident = ident(id);
    if is_shallow {
        ident.push_str("-shallow");
    }
    ident
}

fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}
