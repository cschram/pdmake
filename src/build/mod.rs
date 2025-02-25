use crate::{config::PDMakeConfig, plua::Plua};
use anyhow::{Context, Error, Result};
use glob::glob;
use std::{
    fs, io,
    path::{Path, PathBuf},
};

pub struct PDBuild<'a> {
    config: &'a PDMakeConfig,
    plua: Plua,
    build_dir: PathBuf,
    pdx_dir: PathBuf,
}

impl<'a> PDBuild<'a> {
    pub fn new(config: &'a PDMakeConfig) -> Self {
        let mut build_dir = PathBuf::new();
        build_dir.push(&config.build.directories.target);
        build_dir.push("debug");
        let mut pdx_dir = PathBuf::new();
        pdx_dir.push(&config.build.directories.target);
        pdx_dir.push(format!("{}.pdx", config.bundle_id));
        Self {
            config,
            plua: Plua::new(),
            build_dir: build_dir,
            pdx_dir: pdx_dir,
        }
    }

    pub fn build(&'a self) -> Result<()> {
        self.ensure_dir_exists(&self.build_dir)?;
        self.build_source().context("Failed to build source")?;
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

    fn build_source(&'a self) -> Result<()> {
        for entry in glob(&format!("{}/**/*.lua", self.config.build.directories.src))? {
            let source = entry?;
            let mut destination = PathBuf::new();
            destination.push(self.build_dir.clone());
            destination.push(
                source
                    .as_path()
                    .strip_prefix(&self.config.build.directories.src)?,
            );

            self.ensure_dir_exists(
                &destination
                    .parent()
                    .expect("Failed to get destination directory"),
            )?;

            fs::copy(source, destination)?;
        }
        for entry in glob(&format!("{}/**/*.plua", self.config.build.directories.src))? {
            let source = entry?;
            let mut destination = PathBuf::new();
            destination.push(self.build_dir.clone());
            destination.push(
                source
                    .as_path()
                    .strip_prefix(&self.config.build.directories.src)?,
            );
            destination.set_extension("lua");

            self.ensure_dir_exists(
                &destination
                    .parent()
                    .expect("Failed to get destination directory"),
            )?;

            // TODO: Preprocessor :)
            // let mp = plua.preprocess(source, destination)?;
            // let output = mp.compile()?;
            // fs::write(destination, output)?;
        }
        Ok(())
    }
}
