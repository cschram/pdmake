use anyhow::{anyhow, Result};
use log::{error, info};
use std::{env, process::Command};

pub(crate) fn exec(cmd: &str, args: &[&str]) -> Result<()> {
    let output = Command::new(&cmd).args(args).output()?;
    if output.status.success() {
        info!("{}", String::from_utf8(output.stdout)?);
    } else {
        error!("{}", String::from_utf8(output.stderr)?);
    }
    Ok(())
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
