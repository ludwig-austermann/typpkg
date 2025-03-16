use anyhow::{Context, Result};
use serde::Deserialize;
use toml::Table;

pub struct TypstPackageConfig {
    /// package.name
    pub name: String,
    /// package.version
    pub version: String,
    /// package.entrypoint
    pub entrypoint: String,
    /// tool own's config
    pub typpkg_config: TypPkgConfig,
    /// other tools and typst's config itself
    pub non_typpkg: Table,
}

#[derive(Default, Deserialize)]
#[serde(default)]
pub struct TypPkgConfig {
    /// pathes to exclude from typpkg packaging
    exclude: Vec<String>,
    /// pathes to include even though excluded from typpkg packaging
    include: Vec<String>,
    /// replace imports by typst universe import or local import
    replace_imports: Vec<String>,
    /// replace @local by @preview
    pub replace_locals: bool,
    /// script to run before this tool executes
    pub postscript: Option<Script>,
    /// files to compile
    pub typst_compile: Vec<TypstCompileConfig>,
    /// tests to check
    pub tests: Vec<()>,
}

impl TypPkgConfig {
    pub fn get_exclude_glob(&self) -> Result<wax::Any> {
        Ok(wax::any([
            wax::any([
                "typst.toml",
                ".git",
                ".git/**/*",
                ".gitignore",
                "tests",
                "test/**/*",
            ])
            .unwrap(),
            wax::any(self.exclude.iter().map(String::as_str))
                .context("Not valid (wax-)globs in exclude.")?,
        ])
        .unwrap())
    }

    pub fn get_include_glob(&self) -> Result<wax::Any> {
        wax::any(self.include.iter().map(String::as_str))
            .context("Not valid (wax-)globs in include.")
    }

    pub fn get_replace_imports_glob(&self) -> Result<wax::Any> {
        wax::any(self.replace_imports.iter().map(String::as_str))
            .context("Not valid (wax-)globs in replace_imports.")
    }
}

#[derive(Default, Deserialize)]
#[serde(default)]
pub struct Script {
    pub command: String,
    pub arguments: Vec<String>,
}

#[derive(Default, Deserialize)]
#[serde(default)]
pub struct TypstCompileConfig {
    /// path to compile
    pub path: String,
    pub format: CompileTarget,
    pub root: Option<String>,
}

#[derive(Default, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CompileTarget {
    #[default]
    Pdf,
    Png,
    Svg,
}

impl ToString for CompileTarget {
    fn to_string(&self) -> String {
        match self {
            CompileTarget::Pdf => "pdf".to_string(),
            CompileTarget::Png => "png".to_string(),
            CompileTarget::Svg => "svg".to_string(),
        }
    }
}
