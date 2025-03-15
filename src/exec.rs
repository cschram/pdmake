use anyhow::Result;
use log::error;
use std::process::Command;

pub fn exec(cmd: &str, args: &[&str]) -> Result<()> {
    let output = Command::new(cmd).args(args).output()?;
    if !output.status.success() {
        let err = String::from_utf8(output.stderr)?;
        error!("{}", err);
    }
    Ok(())
}
