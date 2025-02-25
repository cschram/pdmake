use std::{io, fs};
use anyhow::Result;
use crate:config::PDMakeConfig;

pub struct PDBuild<'a> {
    config: &'a PDMakeConfig,
}

impl PDBuild {
    pub fn new<'a>(config: &'a PDMakeConfig) -> Self<'a> {
        Self {
            config,
        }
    }

    pub fn build(&self) -> Result<()> {
        self.make_target()?;
    }

    fn make_target(&self) -> Result<()> {
        if let Err(err) = fs::create_dir(&self.config.directories.target) {
            if err.kind() != io::ErrorKind::AlreadyExists {
                Err(err)
            }
        }
        Ok(())
    }
}
