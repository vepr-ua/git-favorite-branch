use std::process::{Command, ExitStatus, Output};

use crate::errors::{GFBError, Result};

pub fn get_current_branch() -> Result<String> {
    let branch_name_raw = Command::new("sh")
        .arg("-c")
        .arg("git branch --show-current")
        .output()
        .map_err(|e| GFBError::IOError(e))?;

    if !branch_name_raw.status.success() {
        let stderr = String::from_utf8_lossy(&branch_name_raw.stderr);
        return Err(GFBError::CurrentBranchResolveFailed(
            stderr.trim().to_owned(),
        ));
    }

    let name = String::from_utf8(branch_name_raw.stdout)
        .map_err(|e| GFBError::CurrentBranchResolveFailed(e.to_string()))?;

    if name.trim().is_empty() {
        return Err(GFBError::CurrentBranchResolveFailed(
            "HEAD is detached".to_owned(),
        ));
    }

    Ok(name)
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

pub fn merge_branch(branch_name: &str) -> Result<ExitStatus> {
    Command::new("sh")
        .arg("-c")
        .arg(format!("git merge {}", branch_name))
        .status()
        .map_err(|e| GFBError::IOError(e))
}
