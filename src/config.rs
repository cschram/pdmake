use anyhow::Result;
use serde::Deserialize;
use std::collections::HashMap;
use toml::Value;

#[derive(Deserialize)]
pub struct PDMakeConfig {
    pub name: String,
    pub author: String,
    pub description: String,
    pub bundle_id: String,
    pub version: String,
    #[serde(default)]
    pub dependencies: PDMakeConfigDependencies,
    #[serde(default)]
    pub build: PDMakeConfigBuild,
}

impl PDMakeConfig {
    pub fn parse(source: &str) -> Result<Self> {
        let config: Self = toml::from_str(source)?;
        Ok(config)
    }
}

pub type PDMakeConfigDependencies = HashMap<String, String>;

#[derive(Deserialize, Default)]
pub struct PDMakeConfigBuild {
    pub directories: PDMakeConfigBuildDirectories,
    pub environment: PDMakeConfigBuildEnvironment,
}

#[derive(Deserialize)]
pub struct PDMakeConfigBuildDirectories {
    #[serde(default = "default_directories_src")]
    pub src: String,
    #[serde(default = "default_directories_assets")]
    pub assets: String,
    #[serde(default = "default_directories_target")]
    pub target: String,
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

impl Default for PDMakeConfigBuildDirectories {
    fn default() -> Self {
        Self {
            src: default_directories_src(),
            assets: default_directories_assets(),
            target: default_directories_target(),
        }
    }
}

pub type PDMakeConfigBuildEnvironment = HashMap<String, Value>;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse() -> Result<()> {
        let config = PDMakeConfig::parse(
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
        let config = PDMakeConfig::parse(
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
