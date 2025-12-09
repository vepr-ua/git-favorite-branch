use std::process::{Command, ExitStatus, Output};

use crate::errors::{GFBError, Result};

pub fn get_current_branch() -> Result<String> {
    let branch_name_raw = Command::new("sh")
        .arg("-c")
        .arg("git branch --show-current")
        .output()
        .map_err(|e| GFBError::IOError(e))?;

    String::from_utf8(branch_name_raw.stdout)
        .map_err(|e| GFBError::CurrentBranchResolveFailed(e.to_string()))
}

pub fn switch_branch(branch_name: &str) -> Result<ExitStatus> {
    Command::new("sh")
        .arg("-c")
        .arg(format!("git checkout {}", branch_name))
        .status()
        .map_err(|e| GFBError::IOError(e))
}

pub fn create_branch(branch_name: &str) -> Result<ExitStatus> {
    Command::new("sh")
        .arg("-c")
        .arg(format!("git checkout -b {}", branch_name))
        .status()
        .map_err(|e| GFBError::IOError(e))
}

pub fn get_local_hash() -> Result<Output> {
    Command::new("sh")
        .arg("-c")
        .arg("git rev-parse --verify HEAD")
        .output()
        .map_err(|e| GFBError::IOError(e))
}
