use anyhow::Result;
use serde::Deserialize;

#[derive(Deserialize)]
pub(crate) struct Config {
    pub(crate) name: String,
    pub(crate) author: String,
    pub(crate) description: String,
    pub(crate) bundle_id: String,
    pub(crate) version: String,
    #[serde(default)]
    pub(crate) directories: ConfigDirectories,
}

impl Config {
    pub(crate) fn parse(source: &str) -> Result<Self> {
        let config: Self = toml::from_str(source)?;
        Ok(config)
    }
}

#[derive(Deserialize)]
pub(crate) struct ConfigDirectories {
    #[serde(default = "default_directories_src")]
    pub(crate) src: String,
    #[serde(default = "default_directories_assets")]
    pub(crate) assets: String,
    #[serde(default = "default_directories_target")]
    pub(crate) target: String,
}

fn default_directories_src() -> String {
    "src".to_owned()
}

fn default_directories_assets() -> String {
    "assets".to_owned()
}

fn default_directories_target() -> String {
    "target".to_owned()
}

impl Default for ConfigDirectories {
    fn default() -> Self {
        Self {
            src: default_directories_src(),
            assets: default_directories_assets(),
            target: default_directories_target(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse() -> Result<()> {
        let config = Config::parse(
            r#"
name = "PDMake Example"
author = "John Playdate"
description = "Example pdmake project"
bundle_id = "com.pdmake.Example"
version = "1.0"

[directories]
src = "src"
assets = "assets"
target = "target"
            "#,
        )?;

        assert_eq!(config.name, "PDMake Example");
        assert_eq!(config.author, "John Playdate");
        assert_eq!(config.description, "Example pdmake project");
        assert_eq!(config.bundle_id, "com.pdmake.Example");
        assert_eq!(config.version, "1.0");
        assert_eq!(config.directories.src, "src");
        assert_eq!(config.directories.assets, "assets");
        assert_eq!(config.directories.target, "target");

        Ok(())
    }

    #[test]
    fn parse_defaults() -> Result<()> {
        let config = Config::parse(
            r#"
name = "PDMake Example"
author = "John Playdate"
description = "Example pdmake project"
bundle_id = "com.pdmake.Example"
version = "1.0"
            "#,
        )?;

        assert_eq!(config.name, "PDMake Example");
        assert_eq!(config.author, "John Playdate");
        assert_eq!(config.description, "Example pdmake project");
        assert_eq!(config.bundle_id, "com.pdmake.Example");
        assert_eq!(config.version, "1.0");
        assert_eq!(config.build.directories.src, "src");
        assert_eq!(config.build.directories.assets, "assets");
        assert_eq!(config.build.directories.target, "target");

        Ok(())
    }
}
