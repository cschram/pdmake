use crate::config::Config;
use anyhow::{Context, Result};
use std::path::PathBuf;
use std::process::Command;

pub(crate) trait AssetProcessor {
    fn process(&self, config: &Config, source: &str, destination: &str) -> Result<()>;
}

#[cfg(unix)]
const ASEPRITE_NAME: &'static str = "aseprite";
#[cfg(windows)]
const ASEPRITE_NAME: &'static str = "aseprite.exe";

#[derive(Clone)]
pub struct AsepriteProcessor;

impl AssetProcessor for AsepriteProcessor {
    fn process(&self, config: &Config, source: &str, destination: &str) -> Result<()> {
        let mut dest = PathBuf::new();
        dest.push(destination);
        dest.set_extension("png");
        Command::new(
            &config
                .build
                .aseprite_path
                .clone()
                .unwrap_or_else(|| ASEPRITE_NAME.to_string()),
        )
        .args(["-b", source, "--save-as", dest.as_path().to_str().unwrap()])
        .output()
        .with_context(|| format!("Error processing aseprite file {}", source))?;
        Ok(())
    }
}
