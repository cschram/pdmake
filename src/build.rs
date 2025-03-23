use crate::{
    config::Config,
    exec::exec,
    processors::{AsepriteProcessor, AssetProcessor, PluaProcessor},
};
use anyhow::{Context, Error, Result};
use std::{
    collections::HashMap,
    fs, io,
    path::{Path, PathBuf},
};
use walkdir::WalkDir;

#[cfg(not(windows))]
const PDC: &str = "pdc";
#[cfg(windows)]
const PDC: &str = "pdc.exe";

pub(crate) struct Builder<'a> {
    config: &'a Config,
    _debug: bool,
    target: PathBuf,
    pdx: PathBuf,
    processors: HashMap<String, Box<dyn AssetProcessor>>,
}

impl<'a> Builder<'a> {
    pub(crate) fn new(config: &'a Config, debug: bool) -> Result<Self> {
        let mut target = PathBuf::new();
        target.push(&config.build.target);
        target.push(if debug { "debug" } else { "release" });

        let mut pdx = PathBuf::new();
        pdx.push(&config.build.target);
        pdx.push(format!("{}.pdx", config.bundle_id));

        let mut processors = HashMap::<String, Box<dyn AssetProcessor>>::new();
        let aseprite = Box::new(AsepriteProcessor {});
        processors.insert("ase".to_string(), aseprite.clone());
        processors.insert("aseprite".to_string(), aseprite.clone());
        let plua = Box::new(PluaProcessor::new(debug)?);
        processors.insert("plua".to_string(), plua);

        Ok(Self {
            config,
            _debug: debug,
            target,
            pdx,
            processors,
        })
    }

    pub(crate) fn build(config: &'a Config, debug: bool) -> Result<()> {
        let builder = Self::new(config, debug)?;
        builder.ensure_dir_exists(&builder.target)?;
        builder.process()?;
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
        dest.push(&self.target);
        dest.push(path.as_ref().strip_prefix(prefix)?);
        Ok(dest)
    }

    fn process(&'a self) -> Result<()> {
        for entry in WalkDir::new(&self.config.build.source) {
            let mut source_pathbuf = PathBuf::new();
            source_pathbuf.push(entry.unwrap().path());
            let source_path = source_pathbuf.as_path();
            let source_path_str = source_path.to_str().unwrap();

            if !source_path.is_dir() {
                let dest_path =
                    self.destination_path(source_path_str, &self.config.build.source)?;
                self.ensure_dir_exists(dest_path.parent().unwrap())?;

                match source_path
                    .extension()
                    .and_then(|ext| self.processors.get(ext.to_str().unwrap()))
                {
                    Some(processor) => {
                        processor.process(source_path_str, dest_path.to_str().unwrap())?;
                    }
                    None => {
                        let dest_path =
                            self.destination_path(source_path, &self.config.build.source)?;

                        fs::copy(source_path, &dest_path).with_context(|| {
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
        pdxinfo_path.push(&self.target);
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
        exec(
            PDC,
            &[
                "-q",
                self.target.to_str().unwrap(),
                self.pdx.to_str().unwrap(),
            ],
        )?;
        Ok(())
    }
}
