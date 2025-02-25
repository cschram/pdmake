mod build;
mod config;

use crate::config::PDMakeConfig;
use anyhow::Result;
use std::fs;

fn main() -> Result<()> {
    let config = {
        let source = fs::read_to_string(".pdmake.toml")?;
        PDMakeConfig::parse(&source)
    }?;
    let build = PDBuild::new(&config);
    build.build()?;

    Ok(())
}
