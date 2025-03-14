use crate::{
    config::Config,
    processors::{AsepriteProcessor, AssetProcessor},
};
use anyhow::{Context, Error, Result};
use glob::glob;
use plua::Plua;
use std::{
    collections::HashMap,
    fs,
    io::{self, Write},
    path::{Path, PathBuf},
    process::Command,
};
use walkdir::WalkDir;

#[cfg(unix)]
const PDC_NAME: &'static str = "pdc";
#[cfg(windows)]
const PDC_NAME: &'static str = "pdc.exe";

pub(crate) struct Builder<'a> {
    config: &'a Config,
    debug: bool,
    build_dir: PathBuf,
    pdx_dir: PathBuf,
    plua: Plua,
    processors: HashMap<String, Box<dyn AssetProcessor>>,
}

impl<'a> Builder<'a> {
    pub(crate) fn new(config: &'a Config, debug: bool) -> Result<Self> {
        let mut build_dir = PathBuf::new();
        build_dir.push(&config.build.directories.target);
        build_dir.push(if debug { "debug" } else { "release" });

        let mut pdx_dir = PathBuf::new();
        pdx_dir.push(&config.build.directories.target);
        pdx_dir.push(format!("{}.pdx", config.bundle_id));

        let mut plua = Plua::new()?;
        plua.set_global("debug", debug)?;

        let mut processors = HashMap::<String, Box<dyn AssetProcessor>>::new();
        let aseprite = Box::new(AsepriteProcessor {});
        processors.insert("ase".to_string(), aseprite.clone());
        processors.insert("aseprite".to_string(), aseprite.clone());

        Ok(Self {
            config,
            debug,
            build_dir,
            pdx_dir,
            plua,
            processors,
        })
    }

    pub(crate) fn build(config: &'a Config, debug: bool) -> Result<()> {
        let builder = Self::new(config, debug)?;
        builder.ensure_dir_exists(&builder.build_dir)?;
        builder.build_source().context("Failed to build source")?;
        builder
            .process_assets()
            .context("Failed to process assets")?;
        builder.generate_pdxinfo()?;
        builder.compile()?;
        Ok(())
    }

    fn ensure_dir_exists<P: AsRef<Path>>(&'a self, path: P) -> Result<()> {
        if let Err(err) = fs::create_dir_all(path) {
            if err.kind() != io::ErrorKind::AlreadyExists {
                return Err(Error::from(err));
            }
        }
        Ok(())
    }

    fn destination_path<P: AsRef<Path>>(&'a self, path: P, prefix: &str) -> Result<PathBuf> {
        let mut dest = PathBuf::new();
        dest.push(&self.build_dir);
        dest.push(path.as_ref().strip_prefix(prefix)?);
        Ok(dest)
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

    fn build_source(&'a self) -> Result<()> {
        for entry in glob(&format!("{}/**/*.lua", self.config.build.directories.src))? {
            let source = entry?;
            let destination = self.destination_path(&source, &self.config.build.directories.src)?;

            self.ensure_dir_exists(&destination.parent().unwrap())?;

            fs::copy(source, destination)?;
        }
        for entry in glob(&format!("{}/**/*.plua", self.config.build.directories.src))? {
            let source = entry?;
            let mut destination =
                self.destination_path(&source, &self.config.build.directories.src)?;
            destination.set_extension("lua");

            self.ensure_dir_exists(&destination.parent().unwrap())?;

            let source_str = source.to_str().unwrap();
            let contents = fs::read_to_string(&source_str)
                .with_context(|| format!("Error reading {}", source_str))?;
            let prog = Plua::compile(&source_str, &contents)
                .with_context(|| format!("Error compiling {} metaprogram", source_str))?;
            let compiled = self
                .plua
                .exec(&prog)
                .with_context(|| format!("Error executing {} metaprogram", source_str))?;
            Self::write_lua(destination.to_str().unwrap(), &compiled)?;
        }
        Ok(())
    }

    fn process_assets(&'a self) -> Result<()> {
        for entry in WalkDir::new(&self.config.build.directories.assets) {
            let mut source_pathbuf = PathBuf::new();
            source_pathbuf.push(entry.unwrap().path());
            let source_path = source_pathbuf.as_path();
            let source_path_str = source_path.to_str().unwrap();

            if !source_path.is_dir() {
                let dest_path =
                    self.destination_path(source_path_str, &self.config.build.directories.assets)?;
                self.ensure_dir_exists(&dest_path.parent().unwrap())?;

                match source_path
                    .extension()
                    .map(|ext| self.processors.get(ext.to_str().unwrap()))
                    .flatten()
                {
                    Some(processor) => {
                        processor.process(
                            &self.config,
                            source_path_str,
                            dest_path.to_str().unwrap(),
                        )?;
                    }
                    None => {
                        let dest_path = self.destination_path(
                            &source_path,
                            &self.config.build.directories.assets,
                        )?;

                        fs::copy(&source_path, &dest_path).with_context(|| {
                            format!(
                                "Failed to copy {} to {}",
                                &source_path.to_str().unwrap(),
                                &dest_path.to_str().unwrap()
                            )
                        })?;
                    }
                }
            }
        }
        Ok(())
    }

    fn generate_pdxinfo(&'a self) -> Result<()> {
        let mut pdxinfo_path = PathBuf::new();
        pdxinfo_path.push(&self.build_dir);
        pdxinfo_path.push("pdxinfo");

        let pdxinfo = format!(
            "
name={}
author={}
description={}
bundleID={}
version={}
buildNumber=1
        ",
            &self.config.name,
            &self.config.author,
            &self.config.description,
            &self.config.bundle_id,
            &self.config.version
        );
        fs::write(pdxinfo_path.to_str().unwrap(), &pdxinfo).context("Error generating pdxinfo")?;
        Ok(())
    }

    fn compile(&'a self) -> Result<()> {
        let output = Command::new(PDC_NAME)
            .args([
                "-q",
                self.build_dir.to_str().unwrap(),
                self.pdx_dir.to_str().unwrap(),
            ])
            .output()
            .context("Error compiling pdx")?;
        if !output.status.success() {
            io::stderr().write_all(&output.stderr)?;
        }
        Ok(())
    }
}
