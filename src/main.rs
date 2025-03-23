mod build;
mod config;
mod processors;

use crate::{build::Builder, config::Config};
use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use duct::cmd;
use simple_logger::SimpleLogger;
use std::{fs, path::PathBuf};

#[cfg(target_os = "macos")]
use anyhow::anyhow;

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
pub fn find_osx_app(app: &str) -> Option<String> {
    let path_env = env::var_os("PATH")?;
    for path in env::split_paths(&path_env) {
        let mut app_path = path.clone();
        app_path.push(app);
        if app_path.as_path().try_exists().ok()? {
            return Some(app_path.to_str().unwrap().to_string());
        }
    }
    None
}

#[cfg(target_os = "macos")]
fn run(pdx_path: &str) -> Result<()> {
    let app_path =
        find_osx_app(SIMULATOR).ok_or_else(|| anyhow!("Unable to find {}", SIMULATOR))?;
    let mut path = PathBuf::new();
    path.push(app_path);
    path.push("Contents/MacOS/Playdate Simulator");
    run_spawn(path.to_str().unwrap(), pdx_path)
}

#[cfg(not(target_os = "macos"))]
fn run(pdx_path: &str) -> Result<()> {
    run_spawn(SIMULATOR, pdx_path)
}

fn run_spawn(sim_path: &str, pdx_path: &str) -> Result<()> {
    cmd!(sim_path, pdx_path).run()?;
    Ok(())
}

fn main() -> Result<()> {
    SimpleLogger::new().init().unwrap();

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
            if fs::exists(&config.build.target)? {
                fs::remove_dir_all(&config.build.target)?;
            }
        }
        Commands::Run => {
            let mut pdx_path = PathBuf::new();
            pdx_path.push(config.build.target);
            pdx_path.push(format!("{}.pdx", config.bundle_id));
            run(pdx_path.to_str().unwrap())?;
        }
    }

    Ok(())
}
