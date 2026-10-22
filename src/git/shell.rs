use std::process::Command;

use crate::git::{
    contract::GitProvider,
    error::{with_op, GitCause, GitOp},
};

pub struct GitShell {}

impl GitProvider for GitShell {
    fn get_current_branch(&self) -> super::result::GitResult<String> {
        with_op(GitOp::CurrentBranch, || {
            let branch_name_raw = Command::new("sh")
                .arg("-c")
                .arg("git branch --show-current")
                .output()?;

            if !branch_name_raw.status.success() {
                let stderr = String::from_utf8_lossy(&branch_name_raw.stderr);
                return Err(GitCause::Failed(stderr.trim().to_owned()));
            }

            let name = String::from_utf8(branch_name_raw.stdout)?;
            let name = name.trim();

            if name.is_empty() {
                return Err(GitCause::Failed("HEAD is detached".to_owned()));
            }

            Ok(name.to_string())
        })
    }

    fn switch_branch(&self, branch_name: &str) -> super::result::GitResult<()> {
        with_op(GitOp::Switch(branch_name.to_string()), || {
            Command::new("sh")
                .arg("-c")
                .arg(format!("git checkout {}", branch_name))
                .status()?;

            Ok(())
        })
    }

    fn create_branch(&self, branch_name: &str) -> super::result::GitResult<()> {
        with_op(GitOp::Create(branch_name.to_string()), || {
            Command::new("sh")
                .arg("-c")
                .arg(format!("git checkout -b {}", branch_name))
                .status()?;

            Ok(())
        })
    }

    fn merge_branch(&self, branch_name: &str) -> super::result::GitResult<()> {
        with_op(GitOp::Merge(branch_name.to_string()), || {
            Command::new("sh")
                .arg("-c")
                .arg(format!("git merge {}", branch_name))
                .status()?;

            Ok(())
        })
    }
}
