use crate::exec::exec;
use anyhow::{anyhow, Result};
use std::path::PathBuf;

#[cfg(target_os = "macos")]
use crate::exec::find_osx_app;

pub(crate) trait AssetProcessor {
    fn process(&self, source: &str, destination: &str) -> Result<()>;
}

#[cfg(target_os = "macos")]
const ASEPRITE: &'static str = "Aseprite.app";
#[cfg(all(unix, not(target_os = "macos")))]
const ASEPRITE: &'static str = "aseprite";
#[cfg(windows)]
const ASEPRITE: &'static str = "aseprite.exe";
#[derive(Clone)]
pub(crate) struct AsepriteProcessor;

impl AssetProcessor for AsepriteProcessor {
    fn process(&self, source: &str, destination: &str) -> Result<()> {
        let mut dest = PathBuf::new();
        dest.push(destination);
        dest.set_extension("png");
        run_aseprite(source, dest.as_path().to_str().unwrap())?;
        Ok(())
    }
}

#[cfg(target_os = "macos")]
fn run_aseprite(source: &str, dest: &str) -> Result<()> {
    let app_path = find_osx_app(ASEPRITE).ok_or_else(|| anyhow!("Cannot find {}", ASEPRITE))?;
    let mut path = PathBuf::new();
    path.push(app_path);
    path.push("Contents/MacOS/aseprite");
    exec(path.to_str().unwrap(), &["-b", source, "--save-as", dest])
}

#[cfg(not(target_os = "macos"))]
fn run_aseprite(source: &str, dest: &str) -> Result<()> {
    exec(ASEPRITE, &["-b", source, "--save-as", dest])
}
