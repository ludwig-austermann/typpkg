use crate::config;
use anyhow::{Context, Result};
use std::fs::File;
use std::io::Read;
use toml::Table;

fn toml_field_extractor<'a>(t: &'a Table, f: &str) -> Result<&'a str> {
    t.get(f)
        .context(format!(
            "[package] does not contain the field `{}` in `typst.toml`.",
            f
        ))?
        .as_str()
        .context(format!("Field `{}` in `typst.toml` should be a string.", f))
}

/// function to parse `typst.toml` of a package to extract its data
pub fn parse_typst_toml(path: &std::path::PathBuf) -> Result<config::TypstPackageConfig> {
    let mut config_file =
        File::open(path.join("typst.toml")).context("Failed to open `typst.toml`.")?;

    let mut config = String::new();
    config_file
        .read_to_string(&mut config)
        .context("Failed to read `typst.toml`.")?;

    let mut config = config
        .parse::<Table>()
        .context("Error parsing the toml in `typst.toml`.")?;

    let self_config = if let Some(tool) = config.get_mut("tool") {
        tool.as_table_mut()
            .context("The `tool` field is supposed to be a table.")?
            .remove("typpkg")
            .map(|x| {
                x.try_into::<Table>()
                    .context("The `tool.typpkg` field is supposed to be a table.")
            })
            .transpose()?
    } else {
        None
    };

    let (name, version, entrypoint) = {
        let package_config = config
            .get("package")
            .context("`typst.toml` does not contain the `package` table.")?
            .as_table()
            .context("The `package` field is not a table in `typst.toml`.")?;
        (
            toml_field_extractor(package_config, "name")?.to_owned(),
            toml_field_extractor(package_config, "version")?.to_owned(),
            toml_field_extractor(package_config, "entrypoint")?.to_owned(),
        )
    };

    log::info!(
        "Package details are: name: {}, version: {}, entrypoint: {}",
        name,
        version,
        entrypoint
    );

    let typpkg_config: config::TypPkgConfig = if let Some(c) = self_config {
        c.try_into()
            .context("Error parsing `[tool.typpkg]` in `typst.toml`.")?
    } else {
        log::warn!("No [tool.typpkg] found in `typst.toml`.");
        Default::default()
    };

    Ok(config::TypstPackageConfig {
        name,
        version,
        entrypoint,
        typpkg_config,
        non_typpkg: config,
    })
}
