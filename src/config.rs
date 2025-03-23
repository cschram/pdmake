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
    pub(crate) build: ConfigBuild,
}

impl Config {
    pub(crate) fn parse(source: &str) -> Result<Self> {
        let config: Self = toml::from_str(source)?;
        Ok(config)
    }
}

#[derive(Deserialize)]
pub(crate) struct ConfigBuild {
    #[serde(default = "default_directories_source")]
    pub(crate) source: String,
    #[serde(default = "default_directories_target")]
    pub(crate) target: String,
}

fn default_directories_source() -> String {
    "source".to_owned()
}

fn default_directories_target() -> String {
    "target".to_owned()
}

impl Default for ConfigBuild {
    fn default() -> Self {
        Self {
            source: default_directories_source(),
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

[build]
source = "alt_source"
target = "alt_target"
            "#,
        )?;

        assert_eq!(config.name, "PDMake Example");
        assert_eq!(config.author, "John Playdate");
        assert_eq!(config.description, "Example pdmake project");
        assert_eq!(config.bundle_id, "com.pdmake.Example");
        assert_eq!(config.version, "1.0");
        assert_eq!(config.build.source, "alt_source");
        assert_eq!(config.build.target, "alt_target");

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
        assert_eq!(config.build.source, "source");
        assert_eq!(config.build.target, "target");

        Ok(())
    }
}
