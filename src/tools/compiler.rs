use std::path::PathBuf;

use crate::config;
use anyhow::{Context, Result};

pub fn compile_files(config: &config::TypstPackageConfig, in_dir: &PathBuf) -> Result<()> {
    log::trace!("Starting to compile files.");
    for comp_conf in config.typpkg_config.typst_compile.iter() {
        for entry in wax::Glob::new(comp_conf.path.as_str())
            .context("Not valid (wax-)globs in typst_compile pathes.")?
            .walk(in_dir)
        {
            let entry = entry?;
            let current_dir = entry.path().parent().unwrap();

            std::process::Command::new("typst")
                .arg("c")
                .arg(entry.path())
                .arg("-f")
                .arg(comp_conf.format.to_string())
                .arg("--root")
                .arg(if let Some(r) = &comp_conf.root {
                    in_dir.join(r)
                } else {
                    current_dir.to_path_buf()
                })
                .current_dir(current_dir)
                .status()
                .unwrap_or_else(|e| {
                    log::error!("Error compiling {}. {}", entry.path().display(), e);
                    std::process::exit(0)
                });
        }
    }
    Ok(())
}
