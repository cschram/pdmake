use crate::exec::exec;
use anyhow::{Context, Result};
use plua::Plua;
use std::{fs, path::PathBuf};

#[cfg(target_os = "macos")]
use crate::exec::find_osx_app;

pub(crate) trait AssetProcessor {
    fn process(&self, source: &str, destination: &str) -> Result<()>;
}

pub(crate) struct PluaProcessor {
    plua: Plua,
    debug: bool,
}

impl PluaProcessor {
    pub(crate) fn new(debug: bool) -> Result<Self> {
        let mut plua = Plua::new()?;
        plua.set_global("debug", debug)?;
        Ok(Self { plua, debug })
    }
}

impl AssetProcessor for PluaProcessor {
    fn process(&self, source: &str, destination: &str) -> Result<()> {
        let mut dest = PathBuf::new();
        dest.push(destination);
        dest.set_extension("lua");

        let contents =
            fs::read_to_string(source).with_context(|| format!("Error reading {}", source))?;
        let prog = Plua::compile(source, &contents)
            .with_context(|| format!("Error compiling {} metaprogram", source))?;
        let compiled = self
            .plua
            .exec(&prog)
            .with_context(|| format!("Error executing {} metaprogram", source));
        if self.debug {
            match compiled {
                Ok(compiled) => {
                    write_lua(dest.to_str().unwrap(), &compiled)?;
                }
                Err(_) => {
                    let mut meta_dest = PathBuf::new();
                    meta_dest.push(destination);
                    meta_dest.set_extension("meta.lua");
                    write_lua(meta_dest.to_str().unwrap(), &prog.metaprogram)?;
                }
            }
        } else {
            write_lua(dest.to_str().unwrap(), &compiled?)?;
        }
        Ok(())
    }
}

fn write_lua(filename: &str, source: &str) -> Result<()> {
    match stylua_lib::format_code(
        source,
        stylua_lib::Config::new(),
        None,
        stylua_lib::OutputVerification::None,
    )
    .context("Failed to format lua")
    {
        Ok(formatted) => {
            fs::write(filename, formatted)?;
            Ok(())
        }
        Err(err) => {
            // Fail gracefull and write the unformatted code so it can be debugged.
            fs::write(filename, source)?;
            Err(err)
        }
    }
}

#[cfg(target_os = "macos")]
const ASEPRITE: &str = "Aseprite.app";
#[cfg(all(unix, not(target_os = "macos")))]
const ASEPRITE: &str = "aseprite";
#[cfg(windows)]
const ASEPRITE: &str = "Aseprite.exe";
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
