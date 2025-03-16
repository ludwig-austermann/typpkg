use crate::config;
use anyhow::{Context, Result};
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use wax::Pattern;

/// function to package the package in the required format plus typpkg's abilities
pub fn package_into(
    config: &config::TypstPackageConfig,
    in_dir: &PathBuf,
    out_dir: &PathBuf,
    is_local_dir: bool,
) -> Result<()> {
    log::trace!("Starting packaging.");
    fs::create_dir_all(&*out_dir)?;

    fs::write(out_dir.join("typst.toml"), config.non_typpkg.to_string())?;

    let exclude_glob = config.typpkg_config.get_exclude_glob()?;
    let include_glob = config.typpkg_config.get_include_glob()?;
    let replace_imports_glob = config.typpkg_config.get_replace_imports_glob()?;

    for entry in WalkDir::new(in_dir).min_depth(1) {
        let entry = entry?;

        let in_project_path = entry.path().strip_prefix(in_dir).unwrap();

        if !exclude_glob.is_match(in_project_path) || include_glob.is_match(in_project_path) {
            let new_path = out_dir.join(in_project_path);
            if entry.metadata().unwrap().is_file() {
                // copy* the file
                if replace_imports_glob.is_match(in_project_path) {
                    let entrypoint = Path::new(
                        "../"
                            .repeat(in_project_path.ancestors().count() - 2)
                            .as_str(),
                    )
                    .join(&config.entrypoint);
                    let replace_from = format!("import \"{}\"", entrypoint.display());
                    let replace_to = format!(
                        "import \"@{}/{}:{}\"",
                        if is_local_dir { "local" } else { "preview" },
                        config.name,
                        config.version
                    );
                    log::debug!(
                        "Copying file `{}`, but replacing `{}` with `{}`",
                        in_project_path.display(),
                        replace_from,
                        replace_to
                    );
                    fs::write(new_path, {
                        let file_content = fs::read_to_string(&entry.path())
                            .context(format!(
                                "Error opening file `{}`.",
                                in_project_path.display()
                            ))?
                            .replace(&replace_from, &replace_to);
                        if config.typpkg_config.replace_locals {
                            file_content.replace("#import \"@local/", "#import \"@preview/")
                        } else {
                            file_content
                        }
                    })
                    .context(format!(
                        "Error writing new `{}`.",
                        in_project_path.display()
                    ))?;
                } else {
                    log::trace!("Copying file `{}`", in_project_path.display());
                    fs::copy(entry.path(), new_path).context(format!(
                        "Error copying file `{}`.",
                        in_project_path.display()
                    ))?;
                }
            } else if !new_path.exists() {
                log::trace!("Creating path {}.", new_path.display());
                fs::create_dir(new_path).context("Error creating directory.")?;
            }
        } else {
            log::trace!("Skipping {}", in_project_path.display())
        }
    }

    Ok(())
}
