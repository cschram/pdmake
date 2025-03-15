use crate::{config::Config, exec::exec};
use anyhow::Result;
use std::path::PathBuf;

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
        exec(
            &config
                .build
                .aseprite_path
                .clone()
                .unwrap_or_else(|| ASEPRITE_NAME.to_string()),
            &["-b", source, "--save-as", dest.as_path().to_str().unwrap()],
        )?;

        Ok(())
    }
}
