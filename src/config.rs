use anyhow::Result;
use serde::Deserialize;
use std::collections::HashMap;
use toml::Value;

#[derive(Deserialize)]
pub(crate) struct Config {
    pub(crate) name: String,
    pub(crate) author: String,
    pub(crate) description: String,
    pub(crate) bundle_id: String,
    pub(crate) version: String,
    #[serde(default)]
    pub(crate) dependencies: ConfigDependencies,
    #[serde(default)]
    pub(crate) build: ConfigBuild,
}

impl Config {
    pub(crate) fn parse(source: &str) -> Result<Self> {
        let config: Self = toml::from_str(source)?;
        Ok(config)
    }
}

pub(crate) type ConfigDependencies = HashMap<String, String>;

#[derive(Deserialize, Default)]
pub(crate) struct ConfigBuild {
    pub(crate) directories: ConfigBuildDirectories,
    pub(crate) environment: ConfigBuildEnvironment,
    pub(crate) aseprite_path: Option<String>,
}

#[derive(Deserialize)]
pub(crate) struct ConfigBuildDirectories {
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

impl Default for ConfigBuildDirectories {
    fn default() -> Self {
        Self {
            src: default_directories_src(),
            assets: default_directories_assets(),
            target: default_directories_target(),
        }
    }
}

pub(crate) type ConfigBuildEnvironment = HashMap<String, Value>;

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

[dependencies]
sequence = "https://github.com/NicMagnier/PlaydateSequence.git"

[build.directories]
src = "src"
assets = "assets"
target = "target"

[build.environment]
playdate = "awesome"
            "#,
        )?;

        assert_eq!(config.name, "PDMake Example");
        assert_eq!(config.author, "John Playdate");
        assert_eq!(config.description, "Example pdmake project");
        assert_eq!(config.bundle_id, "com.pdmake.Example");
        assert_eq!(config.version, "1.0");
        assert_eq!(
            config.dependencies.get("sequence"),
            Some(&"https://github.com/NicMagnier/PlaydateSequence.git".to_owned())
        );
        assert_eq!(config.build.directories.src, "src");
        assert_eq!(config.build.directories.assets, "assets");
        assert_eq!(config.build.directories.target, "target");
        assert_eq!(
            config.build.environment.get("playdate"),
            Some(&toml::Value::String("awesome".to_owned()))
        );

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
