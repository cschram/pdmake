mod build;
mod config;
mod exec;
mod processors;

use crate::{build::Builder, config::Config, exec::exec};
use anyhow::{anyhow, Context, Result};
use clap::{Parser, Subcommand};
use std::{fs, path::PathBuf};

#[cfg(target_os = "macos")]
use crate::exec::find_osx_app;

#[cfg(target_os = "macos")]
const SIMULATOR: &str = "Playdate Simulator.app";
#[cfg(all(unix, not(target_os = "macos")))]
const SIMULATOR: &str = "playdatesimulator";
#[cfg(windows)]
const SIMULATOR: &str = "PlaydateSimulator.exe";

#[derive(Parser)]
#[command(name = "pdmake")]
#[command(version = "0.1-alpha")]
#[command(about = "Lua development toolchain for the Playdate system")]
pub(crate) struct Cli {
    /// Specify the config file
    #[arg(short, long, default_value = "pdmake.toml")]
    config: String,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Build the project
    Build {
        /// Build in debug mode
        #[arg(short, long)]
        debug: bool,
    },
    /// Clean the project build directory
    Clean,
    /// Run the project in the simulator
    Run,
}

#[cfg(target_os = "macos")]
fn run(pdx_path: &str) -> Result<()> {
    let app_path =
        find_osx_app(SIMULATOR).ok_or_else(|| anyhow!("Unable to find {}", SIMULATOR))?;
    let mut path = PathBuf::new();
    path.push(app_path);
    path.push("Contents/MacOS/Playdate Simulator");
    exec(path.to_str().unwrap(), &[pdx_path])
}

#[cfg(not(target_os = "macos"))]
fn run(pdx_path: &str) -> Result<()> {
    exec(SIMULATOR_NAME, &[pdx_path])
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let config = {
        let source = fs::read_to_string(&cli.config)
            .with_context(|| format!("Error reading config {}", &cli.config))?;
        Config::parse(&source).with_context(|| format!("Error parsing config {}", &cli.config))
    }?;

    match &cli.command {
        Commands::Build { debug } => {
            Builder::build(&config, *debug).context("Error building")?;
        }
        Commands::Clean => {
            if fs::exists(&config.directories.target)? {
                fs::remove_dir_all(&config.directories.target)?;
            }
        }
        Commands::Run => {
            let mut pdx_path = PathBuf::new();
            pdx_path.push(config.directories.target);
            pdx_path.push(format!("{}.pdx", config.bundle_id));
            run(pdx_path.to_str().unwrap())?;
        }
    }

    Ok(())
}
