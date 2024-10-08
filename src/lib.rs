use std::result::Result::Ok;
use std::env;
use cargo_metadata::MetadataCommand;
use fs_extra::copy_items;
use fs_extra::dir::CopyOptions;
use std::path::{Path, PathBuf};
use anyhow::*;

pub fn copy_to_output(path: &str, build_type: &str) -> Result<()> {
    let mut out_path = PathBuf::new();

    let metadata = MetadataCommand::new().no_deps().exec().unwrap();
    let cargo_target = metadata.target_directory.as_str();

    out_path.push(&cargo_target);

    // This is a hack, ideally we would plug into https://docs.rs/cargo/latest/cargo/core/compiler/enum.CompileKind.html
    // However, since the path follows predictable rules https://doc.rust-lang.org/cargo/guide/build-cache.html
    // we can just check our parent path for the pattern target/{triple}/{profile}.
    // If it is present, we know CompileKind::Target was used, otherwise CompileKind::Host was used.
    // Best effort since the existing tests aren't intended to be run in a real build this won't exist.
    // Unclear if that also means people in the wild are using the crate similarly, so avoiding any risk of break.
    if let Ok(triple) = build_target::target_triple() {
        if let Some(out_dir) = env::var_os("OUT_DIR") {
            if let Some(out_dir) = out_dir.to_str() {
                if out_dir.contains(&format!("{}{}{}", cargo_target, std::path::MAIN_SEPARATOR, triple)) {
                    out_path.push(triple);
                }
            }
        }
    }

    out_path.push(build_type);

    // Overwrite existing files with same name
    let mut options = CopyOptions::new();
    options.overwrite = true;

    let mut from_path = Vec::new();
    from_path.push(path);
    copy_items(&from_path, &out_path, &options)?;

    Ok(())
}

pub fn copy_to_output_path(path: &Path, build_type: &str) -> Result<()> {
    let path_str = path.to_str().expect("Could not convert file path to string");
    copy_to_output(path_str, build_type)?;

    Ok(())
}
